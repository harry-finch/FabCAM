import { withPhysicalIdentity } from './physical-identities';
import type { MachineSettings, ToolEntry, StockConfig } from './project-types';

// Fresh values per document; never seed a loaded project from the previous job.
export function defaultProjectTools(): ToolEntry[] {
  return [
    withPhysicalIdentity<ToolEntry>(
      {
        id: 1,
        name: '3mm endmill',
        kind: 'endmill',
        diameter: 3,
        flutes: 2,
        speed: 18000,
        plungeRate: 100,
        feedRate: 800,
        coolant: 'off',
      },
      'tool',
    ),
  ];
}

export function defaultProjectMachine(): MachineSettings {
  return withPhysicalIdentity<MachineSettings>(
    {
      unit: 'mm',
      mode: 'mill',
      comments: true,
      arcs: true,
      toolchangeStrategy: 'manual_m0_pause',
      fastMoveZ: 5,
      accel: { x: 250, y: 250, z: 250 },
      toolchangeS: 5,
      rapidSpeed: 5000,
      workArea: { x: 200, y: 300, z: 50 },
    },
    'machine',
  );
}

export function defaultProjectStock(): StockConfig {
  return {
    visible: true,
    mode: 'auto',
    margin: 5,
    thickness: 5,
    customX: 100,
    customY: 100,
  };
}
