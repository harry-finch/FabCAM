# ShopBot MVP output goldens

Required deterministic full text produced by the owned CPS post from M1 SVGs.
The integration suite also executes equivalent DXFs with analytic assertions.

- square.sbp: outside square 100 mm, Ø6, Z−2/−4/−5.
- circle.sbp: outside circle D50, Ø6, radius 28, same depths.
- pocket.sbp: 100 mm square with 20 mm central island, Ø6, cascade, 50% overlap.
- drill.sbp: single center (60,60), Ø6 118° drill, shoulder −5, pecks 2 mm,
  authored clearance 1.5 mm, dwell 0.2 seconds.
- multiple.sbp: that drill then outside circle with tool 2, explicit manual stops.

Common stock 140×140×10, top Z0, program clearance Z5, CW 18000 RPM requested
manually; feed 1200 mm/min, plunge 200. These are analytic test parameters and
are **not the real lab machine profile or cutting recommendations**.

Normal tests never rewrite goldens and fail if any file is missing. Intentional
refresh after reviewing a change:

```sh
IVAC_UPDATE_SHOPBOT_GOLDENS=1 RUSTC_WRAPPER="" cargo test -p ivac-core --features cps --test shopbot_m2_post --locked
```

Then inspect the text diff and rerun without the update variable. The M2 review
checks start/end, tool/zero/RPM prompts, speeds, coordinates, circle chords,
peck re-entry/retract and operation order; automated assertions supply numeric
checks. Pocket is 31,733 lines: machine buffering/performance remains untested.
See `docs/shopbot-modernization/M2_QUALIFICATION.md` before controller preview.
No hardware validation is claimed; operator pauses require actual manual action.
