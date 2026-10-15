import { it, expect, vi } from 'vitest';
import { PaletteDispatch } from './palette-dispatch';
it('calls the existing owner once and passes the stable argument', async () => {
  const dispatch = new PaletteDispatch();
  const run = vi.fn();
  dispatch.register('templates', { state: () => ({ token: 'epoch1' }), run });
  await dispatch.execute('templates', dispatch.state('templates')!, 'recipe-7');
  expect(run).toHaveBeenCalledTimes(1);
  expect(run).toHaveBeenCalledWith('recipe-7');
});
it('refuses a changed document or selected target', async () => {
  const dispatch = new PaletteDispatch();
  let token = '1:7';
  const run = vi.fn();
  dispatch.register('delete', { state: () => ({ token }), run });
  const offer = dispatch.state('delete')!;
  token = '2:9';
  expect(await dispatch.execute('delete', offer)).toBe('changed');
  expect(run).not.toHaveBeenCalled();
});
it('refuses disposed or replaced owners, including identical tokens', async () => {
  const dispatch = new PaletteDispatch();
  const run = vi.fn();
  const dispose = dispatch.register('export', { state: () => ({ token: 'same' }), run });
  const offer = dispatch.state('export')!;
  dispose();
  expect(await dispatch.execute('export', offer)).toBe('changed');
  dispatch.register('export', { state: () => ({ token: 'same' }), run });
  expect(await dispatch.execute('export', offer)).toBe('changed');
  expect(run).not.toHaveBeenCalled();
});
it('rechecks critical/availability reasons at execution', async () => {
  const dispatch = new PaletteDispatch();
  let reason: string | undefined = undefined;
  const run = vi.fn();
  dispatch.register('export', { state: () => ({ token: '1', reason }), run });
  const offer = dispatch.state('export')!;
  reason = 'Critical warnings';
  expect(await dispatch.execute('export', offer)).toBe('disabled');
  expect(run).not.toHaveBeenCalled();
});
it('prevents concurrent replay but leaves cancel usable during generation', async () => {
  const dispatch = new PaletteDispatch();
  let finish!: () => void;
  const run = vi.fn(
    () =>
      new Promise<void>((resolve) => {
        finish = resolve;
      }),
  );
  const cancel = vi.fn();
  dispatch.register('generate', { state: () => ({ token: '1' }), run });
  dispatch.register('cancel', { state: () => ({ token: '1' }), run: cancel });
  const offer = dispatch.state('generate')!;
  const pending = dispatch.execute('generate', offer);
  expect(await dispatch.execute('generate', offer)).toBe('pending');
  expect(await dispatch.execute('cancel', dispatch.state('cancel')!)).toBe('done');
  finish();
  await pending;
  expect(run).toHaveBeenCalledTimes(1);
  expect(cancel).toHaveBeenCalledTimes(1);
});
it('releases execution after owner rejection and respects disposal subscription', async () => {
  const dispatch = new PaletteDispatch();
  const notice = vi.fn();
  const unsubscribe = dispatch.subscribe(notice);
  const dispose = dispatch.register('save', {
    state: () => ({ token: '1' }),
    run: () => {
      throw new Error('storage');
    },
  });
  await expect(dispatch.execute('save', dispatch.state('save')!)).rejects.toThrow('storage');
  expect(await dispatch.execute('missing', { token: '1' })).toBe('changed');
  dispose();
  unsubscribe();
  expect(notice).toHaveBeenCalledTimes(2);
});
