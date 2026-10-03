---
format: aep.planning-md/3
id: verification-report:release-0-1-0-completion
kind: verification-report
status: draft
title: Verified initial release and enrollment completion
relations:
- reviews: story:public-delivery
- reviews: task:publish-codegate-0-1-0
- reviews: task:integrate-semantic-foundation-release
revision: 1
---
## Release completion reconciled 2026-10-03

PR https://github.com/beyond10x/codegate/pull/1 merged to remote main 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. Annotated tag 0.1.0 object 775d9e4393597209535c3aa9027ca4a18789886f points to that exact commit. The bot-owned release is public at https://github.com/beyond10x/codegate/releases/tag/0.1.0, published 2026-10-03T01:43:01Z. Linux archive and SHA256SUMS were uploaded and downloaded checksums verified; archive SHA256 1fa12b20dfc0cc57f0d6bce18c68c0b8ccd8d609f43dfeb8f48aa6d9dbe31e1b.

Required tag checks succeeded: common security run37086993620, Gate run37086993142, release artifacts run37086993155. Main docs/site runs37086809592 and37086884075 succeeded. Required-check ruleset24402354 is active and has no bypass actors. The policy secret enrollment was completed using the operator's single explicit gh exception; this grants no ongoing personal-write authority. General bot Secrets access remains unavailable but no longer blocks this completed release.

Live https://beyond10x.github.io/codegate/.well-known/b10x-site.json and .well-known/b10x-docs.json were re-read during collection planning and both identify 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. Publisher runtime fb4024ef7846729e5456591b9070db3d48c87e64; artifact e01d174ec402f464fef61cd8c07825e66d4b0dbbf577e26f5007ec0f01cabf0f. Primary main is clean at that remote commit; previous release worktrees were retired with recovery proof. Current collection trees belong to subsequent work.

This resolves the earlier historical pending-release text. No new release is part of the collection wave.
