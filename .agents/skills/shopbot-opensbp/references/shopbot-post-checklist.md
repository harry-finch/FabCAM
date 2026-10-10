# ShopBot post-processor checklist

## Semantics to map

Verify whether the post needs XY rapid, Z rapid, XYZ rapid, linear cutting motion, circular motion, feed/speed changes, spindle on/off, tool-change boundaries, program start/end, units, and absolute/relative coordinate assumptions.

Use only commands verified against the targeted ShopBot/OpenSBP environment.

## Determinism

Given identical CAM IR and post settings, output should be deterministic.

Test numeric formatting, units, safe retract sequence, first/last cutting move, spindle state at start/end, and arc output or intentional linearization.

## Unsupported semantics

Do not silently approximate unsupported behavior if doing so can alter machining intent. Prefer an explicit error or a clearly documented warning when safe.

## External validation progression

1. textual review,
2. syntax/parser/preview validation,
3. bounds review,
4. supervised air cut,
5. sacrificial material,
6. measurement,
7. comparison against trusted reference output where useful.
