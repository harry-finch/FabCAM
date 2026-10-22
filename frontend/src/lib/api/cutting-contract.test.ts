import { afterEach, expect, it, vi } from 'vitest';
import { HttpWiacClient } from './http';
import type { GenerateRequest } from './types';
afterEach(() => vi.unstubAllGlobals());
const request = { project: { cutting_version: 9 } } as GenerateRequest;
it('refuses an old engine before sending any machining request', async () => {
  const fetch = vi
    .fn()
    .mockResolvedValue(
      new Response(JSON.stringify({ version: 'old', capabilities: ['generate-gcode'] })),
    );
  vi.stubGlobal('fetch', fetch);
  await expect(new HttpWiacClient('http://engine').generate(request)).rejects.toThrow('Restart');
  expect(fetch).toHaveBeenCalledTimes(1);
  expect(fetch.mock.calls[0][0]).toContain('/version');
});
it('sends unchanged explicit values only to an engine with the v9 contract', async () => {
  const fetch = vi
    .fn()
    .mockResolvedValueOnce(
      new Response(JSON.stringify({ capabilities: ['cutting-parameters-v9'] })),
    )
    .mockResolvedValueOnce(new Response(JSON.stringify({ gcode: 'verified response' })));
  vi.stubGlobal('fetch', fetch);
  const result = await new HttpWiacClient('http://engine').generate(request);
  expect(result.gcode).toBe('verified response');
  expect(JSON.parse(fetch.mock.calls[1][1].body)).toEqual(request);
});
