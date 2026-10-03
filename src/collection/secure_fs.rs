//! Filesystem traversal anchored to opened directory descriptors; never follows symlinks.
use super::{Gap, GapCode, failure};
use rustix::fs::{CWD, Dir, Mode, OFlags, openat};
use std::{
    fs::{File, Metadata},
    io::Read,
    path::{Component, Path},
};

pub struct Root {
    directory: File,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stamp {
    pub length: u64,
    pub modified: Option<std::time::SystemTime>,
    #[cfg(unix)]
    pub device: u64,
    #[cfg(unix)]
    pub inode: u64,
    #[cfg(unix)]
    pub changed: (i64, i64),
}
impl Stamp {
    fn from(m: &Metadata) -> Self {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;
        Self {
            length: m.len(),
            modified: m.modified().ok(),
            #[cfg(unix)]
            device: m.dev(),
            #[cfg(unix)]
            inode: m.ino(),
            #[cfg(unix)]
            changed: (m.ctime(), m.ctime_nsec()),
        }
    }
}
fn io(path: &str, error: impl std::fmt::Display) -> Gap {
    failure(
        GapCode::PartialCollection,
        format!("cannot read selected input {path}: {error}"),
    )
}
impl Root {
    pub fn open(path: &Path) -> Result<Self, Gap> {
        let mut directory: File = openat(
            CWD,
            ".",
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(|e| io("root", e))?
        .into();
        for part in path.components() {
            match part {
                Component::RootDir => {
                    directory = openat(
                        CWD,
                        "/",
                        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                        Mode::empty(),
                    )
                    .map_err(|e| io("root", e))?
                    .into()
                }
                Component::CurDir => {}
                Component::Normal(name) => {
                    directory = openat(
                        &directory,
                        name,
                        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                        Mode::empty(),
                    )
                    .map_err(|e| io("root", e))?
                    .into()
                }
                _ => return Err(failure(GapCode::InvalidFact, "invalid root path")),
            }
        }
        Ok(Self { directory })
    }
    fn open_path(&self, path: &str) -> Result<File, Gap> {
        let mut file = self.directory.try_clone().map_err(|e| io(path, e))?;
        let parts: Vec<_> = path.split('/').collect();
        for (i, part) in parts.iter().enumerate() {
            let flags = OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK;
            let flags = if i + 1 < parts.len() {
                flags | OFlags::DIRECTORY
            } else {
                flags
            };
            file = openat(&file, *part, flags, Mode::empty())
                .map_err(|e| {
                    if e == rustix::io::Errno::NOENT {
                        failure(
                            GapCode::MissingBuildSelection,
                            format!("selected input is missing: {path}"),
                        )
                    } else {
                        io(path, e)
                    }
                })?
                .into();
        }
        Ok(file)
    }
    pub fn entries(&self, path: &str) -> Result<Vec<String>, Gap> {
        let directory = if path.is_empty() {
            self.directory.try_clone().map_err(|e| io(path, e))?
        } else {
            self.open_path(path)?
        };
        let mut names = Vec::new();
        let entries = Dir::read_from(&directory).map_err(|e| io(path, e))?;
        for entry in entries {
            let entry = entry.map_err(|e| io(path, e))?;
            let bytes = entry.file_name().to_bytes();
            if bytes == b"." || bytes == b".." {
                continue;
            }
            if names.len() >= 100_000 {
                return Err(failure(
                    GapCode::InvalidFact,
                    "source traversal exceeds 100000 entry limit",
                ));
            }
            let name = std::str::from_utf8(bytes)
                .map_err(|_| failure(GapCode::InvalidFact, "selected path is not UTF-8"))?;
            names.push(name.into());
        }
        names.sort();
        Ok(names)
    }
    pub fn metadata(&self, path: &str) -> Result<Metadata, Gap> {
        self.open_path(path)?.metadata().map_err(|e| io(path, e))
    }
    pub fn stamp(&self, path: &str) -> Result<Stamp, Gap> {
        Ok(Stamp::from(&self.metadata(path)?))
    }
    pub fn read(&self, path: &str, limit: usize) -> Result<(String, Stamp), Gap> {
        let mut file = self.open_path(path)?;
        let meta = file.metadata().map_err(|e| io(path, e))?;
        if !meta.is_file() {
            return Err(failure(
                GapCode::InvalidFact,
                format!("selected input is not a regular file: {path}"),
            ));
        }
        if meta.len() > limit as u64 {
            return Err(failure(
                GapCode::InvalidFact,
                "selected file exceeds 4 MiB limit",
            ));
        }
        let before = Stamp::from(&meta);
        let mut bytes = Vec::new();
        Read::by_ref(&mut file)
            .take((limit + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|e| io(path, e))?;
        if bytes.len() > limit {
            return Err(failure(
                GapCode::InvalidFact,
                "selected file exceeds 4 MiB limit",
            ));
        }
        let after = Stamp::from(&file.metadata().map_err(|e| io(path, e))?);
        if before != after || bytes.len() as u64 != before.length {
            return Err(failure(
                GapCode::SourceChangedDuringCollection,
                format!("selected input changed: {path}"),
            ));
        }
        let text = String::from_utf8(bytes).map_err(|_| {
            failure(
                GapCode::InvalidFact,
                format!("selected input is not UTF-8: {path}"),
            )
        })?;
        Ok((text, after))
    }
}
