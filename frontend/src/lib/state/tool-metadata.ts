/** Extensible authored specifications. Do not infer, normalize or calculate rates. */
export function validateToolMetadata(tool: { substrate?: unknown; coating?: unknown }): void {
  for (const key of ['substrate', 'coating'] as const) {
    if (tool[key] != null && typeof tool[key] !== 'string')
      throw new Error(`Invalid tool ${key}: expected text or unspecified`);
  }
}
