---
format: aep.planning-md/3
id: review-result:admission-boundary-pass-two
kind: review-result
status: active
title: Admission second-pass structured finding and correction
relations:
- reviews: story:capability-admission
revision: 1
---

Record-format and factual correction: the observed assertion failed on UnixStream before reaching the thread expression. The worker confirmed the existing direct std::thread rule already rejected thread context; the earlier prose claiming a second bypass was an unverified inference and is withdrawn. Only nested std::os IO is the confirmed finding. This reissues that same second-pass observation with its machine-readable finding, not an additional attack. The immutable original remains available.

```findings
[
{"file":"tests/semantic_boundary.rs","line":90,"category":"acceptance","severity":"blocker","verdict":"CONFIRMED","origin":"introduced","message":"Architectural guard admits nested std::os platform IO."}
]
```
