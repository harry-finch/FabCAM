/** Neutral presentation vocabulary; no cutting ratings or equivalence rules. */
export const MATERIAL_CATALOG = [
  'wood',
  'wood.mdf',
  'wood.plywood',
  'wood.plywood.poplar',
  'wood.plywood.birch',
  'wood.hardwood',
  'wood.softwood',
  'plastic',
  'plastic.pmma',
  'plastic.pmma.cast',
  'plastic.pmma.extruded',
  'plastic.pom',
  'plastic.hdpe',
  'plastic.polycarbonate',
  'plastic.pvc',
  'metal',
  'metal.aluminium',
  'metal.aluminium.6061',
  'metal.aluminium.7075',
].map((id) => ({
  identity: { id: `material/${id}`, revision: '1' },
  label: id
    .split('.')
    .map((s) => s.toUpperCase())
    .join(' / '),
}));
