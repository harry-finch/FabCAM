# Calculation model

Use this reference when designing or changing Feeds & Speeds mathematics.

## Core quantities

For ordinary milling:

```text
feed_mm_min = rpm × flute_count × chipload_mm_per_tooth
```

and:

```text
chipload_mm_per_tooth =
    feed_mm_min / (rpm × flute_count)
```

Validate divisors before calculating.

## Optional derived quantity: cutting speed

When useful for tool/material validation, cutting speed for a tool diameter in millimetres can be expressed as:

```text
Vc_m_min = π × diameter_mm × rpm / 1000
```

Do not make cutting speed mandatory if the initial router-focused dataset is primarily chipload-based.

## DOC and WOC

Keep axial and radial engagement conceptually separate:

- DOC / axial depth of cut,
- WOC / radial width of cut,
- stepover may provide a useful WOC proxy in some operations.

A percentage stepover can be represented as:

```text
radial_engagement_ratio = stepover / tool_diameter
```

only when that interpretation matches the operation.

Do not use nominal stepover as an exact instantaneous engagement model for complex trajectories.

## Slotting

Full-width slotting means approximately:

```text
radial_engagement_ratio = 1.0
```

and is generally a more demanding condition than lower radial engagement.

The MVP should use a transparent conservative slotting policy rather than sophisticated engagement physics.

## Finishing

Finishing may optimize for surface quality rather than material-removal rate.

Do not assume "finishing" always means faster or slower. Define the project policy explicitly and test it.

## Plunge

Plunge rate is not derived from chipload alone.

Treat it as a separate recommendation informed by:

- tool/material profile,
- cutter geometry,
- operation/entry strategy,
- machine constraints.

Do not pretend a universal feed-to-plunge ratio is physically authoritative.

## RPM selection

RPM may come from:

- manufacturer/reference profile,
- validated preset,
- target chipload + practical feed range,
- spindle range,
- material/tool constraints.

Document which strategy the engine uses.

Avoid arbitrary RPM selection solely to make an equation balance.

## Clamping

If a recommendation is clamped by a known limit, expose that fact.

Example:

```text
calculated feed = 5200 mm/min
machine max feed = 4500 mm/min
resolved recommended feed = 4500 mm/min
warning = MachineFeedLimitApplied
```

After clamping one variable, decide deliberately whether dependent values such as effective chipload should be recomputed and reported.

## Correction factors

If the MVP uses correction factors such as:

- slotting factor,
- prudent-mode factor,
- material-family factor,

they must be:

- named,
- documented,
- deterministic,
- testable,
- preferably data-driven where they represent empirical knowledge.

Avoid anonymous constants such as `0.73` embedded in algorithm code.

## Precision

Separate:

- calculation precision,
- displayed precision,
- persisted precision,
- post-processor formatting precision.

Do not round intermediate calculations unnecessarily.
