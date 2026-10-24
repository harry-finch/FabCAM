# FabApps — product vision

FabApps is a family of approachable, technically capable digital-fabrication
applications. Ambitious and fast development must be matched by deliberate UX,
software maturity and physical validation. Implementation is distinct from software
qualification, personal evaluation, supervised physical use, controlled workshop
availability and public distribution. A polished screenshot is not release evidence.

## Product family

| Product | Responsibility and identity | Current direction / evidence boundary |
|---|---|---|
| **FabCAM** | CNC CAM: operations, toolpaths, validation and program export. **Blue** | Primary stabilization focus: [V1.1 macOS Personal Alpha](releases/RELEASE_ROADMAP.md#current-focus--fabcam-v11-macos-personal-alpha), then scoped supervised ShopBot pilot; no general/public deployment |
| **FabSpeed** | Feeds/speeds calculation, optimization, experiments and validated cutting-parameter knowledge. **Magenta** | Independent application and upstream authority for future synchronized catalogues. No release maturity inferred from FabCAM |
| **FabVents** | Parametric perforation design and generation. **Green/lime** | Independent functionality and roadmap; no release commitment added here |
| **FabParts** | **Electronic component manager**, formerly KiPart Manager. **Amber/yellow** | Renaming/UI harmonization implemented in the inspected restyling report; off-screen Qt checks do not complete native acceptance or establish a public release |
| **FabWeave** | Parametric G-code and extrusion-path design for 3D printing, inspired by FullControlGCode's explored capabilities with an approachable graphical model | Development/exploration. Repository records a v0.1 vase/CORE One pilot and v0.2 work, not a completed/public product. Its recorded blue identity is a separate existing branding decision, not a new assignment here |

FabWeave's direction includes parameterizable form galleries, configurable
extrusion paths, printer-specific settings, add/import modular forms and a simple
module-authoring workflow. Its repository is the authority for actual delivery;
these concepts do not imply every planned capability is implemented. Its branding
record selects interlaced wave circles and blue; this vision adds no final-branding
or all-printer qualification claim.

## Independent maturity and sustainable focus

Each app owns its repository/architecture, roadmap, version history, qualification
scope, acceptance criteria and release decisions. Related identity does not require
shared releases or a monolithic platform. FabSpeed, FabVents, FabParts and FabWeave
can evolve without becoming prerequisites for FabCAM Personal Alpha.

Recommend **one primary stabilization effort at a time**, with at most one other
bounded effort if capacity allows. FabCAM V1.1 is that primary effort now. Parallel
prototypes are possible; intensive suite-wide stabilization is not the default.
No other project's roadmap, source, release or priority is changed by this document.

## Shared design language — future initiative

**PLANNED**, separately scoped FabApps Design System: recognizable family identity,
professional dark-mode surfaces, clear hierarchy, compact technical controls,
consistent navigation/interaction, app-specific accent colors, transparent source/
validation semantics and good documentation. Start with principles, tokens/density,
accessibility, iconography/navigation patterns, reference screenshots and component
examples. Keep status colors distinct from product accents and accessible without
color alone. Existing per-app technologies/components remain independent.

No automatic cross-repository UI package. Decide reuse/extraction only after
technology and maintenance costs are understood; this task creates no shared code.

## FabApps website — future portfolio initiative

**PLANNED**, separate from product stabilization. Present use cases, actual maturity,
and links to demos/docs/releases when available. No release announcement based on
visual polish, no launch date, site implementation, publication or hosting work here.

## Optional integration without runtime coupling

Retain [ADR-0030](shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md)
and [proposed ADR-0031](shopbot-modernization/adr/ADR-0031-optional-one-way-fabspeed-catalogue.md):
strict **FabSpeed → FabCAM** business-data direction; upstream-owned common
catalogue fields; persistent local FabCAM copies with stable shared/local IDs and
CAM extensions; explicit linking; independent offline operation. No automatic
changes to authored operations, implicit workshop-attestation transfer, experiment/
project writes back to FabSpeed, automatic apply or integrated calculator.

INT-0/1/2/3 remain separate future development, likely V1.2 intent rather than an
immediate publication commitment. Transport/storage/API remain subject to contract
review. No suite application or live service is a mandatory FabCAM runtime dependency.

## Maturity decisions

FabCAM Personal Alpha is a limited maintainer decision, potentially software-only
until supervised cases pass. Workshop Beta means a later controlled, supervised
fablab decision. Public Release is deferred without an announced date; it requires
new audience/platform scope and qualification. Other products decide their own
maturity independently. [Planning evidence and limits](releases/MACOS_ALPHA_ROADMAP_UPDATE.md)
record this snapshot; no acceptance or implementation is granted here.
