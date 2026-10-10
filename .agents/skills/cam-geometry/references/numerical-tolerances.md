# Numerical tolerance guidance

Before introducing a tolerance, identify what problem it solves.

Do not use one global epsilon for unrelated concerns.

Typical categories include topology epsilon, curve flattening tolerance, boolean/offset precision, test comparison tolerance, post-processor numeric precision, and physical machining tolerance.

A physical tolerance such as 0.1 mm is not automatically suitable as a topology epsilon.

When changing a tolerance:

1. name it by purpose,
2. document its units,
3. add a regression case that requires it,
4. test geometry both above and below the threshold,
5. avoid hiding malformed geometry by making the tolerance arbitrarily large.
