# Accessibility and safety UI

## Do not encode state by color alone

Warnings, selected paths, inside/outside compensation, and simulation states should also use shape, icon, label, line style, or text.

## Severity

Use a small consistent model such as info, warning, and blocking error. A blocking error should explain what must change before export.

## Dangerous actions

Make affected scope clear, support undo when appropriate, avoid confirmation dialogs for routine reversible actions, and reserve confirmation for genuinely destructive/irreversible actions.

## Numeric input

CAM numeric fields should show units, reject invalid ranges explicitly, handle decimal input predictably, preserve reasonable precision, and distinguish derived values from manual overrides.

## Simulation

Simulation must not imply certified safety. Label what was actually checked: toolpath geometry, stock removal, fixture collision, holder collision, machine envelope, etc.
