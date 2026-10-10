# ShopBot modernization

University-fablab fork of ivaCAM. First target: ShopBot PRS Gantry, approximately
2011; future target: OMAX ProtoMAX. Windows, macOS and Linux remain required.
The first objective is a correct, reviewable program from simple SVG/DXF using
the existing engine, replacing the lab's VCarve 8 workflow progressively.

## Navigation

- [STATUS](STATUS.md): current handoff and Git baseline.
- [ROADMAP](ROADMAP.md): scope, milestone gates and approval policy.
- [ARCHITECTURE](ARCHITECTURE.md): verified module map and boundaries.
- [UPSTREAM_ANALYSIS](UPSTREAM_ANALYSIS.md): feature evidence and dispositions.
- [TESTING_STRATEGY](TESTING_STRATEGY.md): build procedure and executed baseline.
- [CAM_VALIDATION](CAM_VALIDATION.md): qualification and physical evidence status.
- [M2 qualification](M2_QUALIFICATION.md): OpenSBP contract, output evidence and limits.
- [M1 qualification](M1_QUALIFICATION.md): corpus, defects, fixes and limits.
- [FUTURE_PROCESSES](FUTURE_PROCESSES.md): deferred ProtoMAX assessment.
- [ShopBot post audit](shopbot/POST_PROCESSING_AUDIT.md): existing extension seams.
- [DEVLOG](DEVLOG.md): engineering history.
- [ADRs](adr/README.md): decision policy.

Repository-local skills live in `.agents/skills/`. Apply only relevant skills.
This documentation supplements upstream `docs/`; it does not replace it.
Evidence is pinned to upstream `9168104b4fc4e02e916740a54bd1e56b5019f662`.
Source presence and passing UI tests do not establish machining correctness.
