import { defineConfig, mergeConfig } from 'vite';
import { resolve } from 'node:path';
import base from '../vite.config';
export default mergeConfig(
  base,
  defineConfig({
    base: './',
    build: {
      outDir:
        process.env.PERF1_BROWSER_OUT ??
        resolve(import.meta.dirname, '../../dist-test/perf-1/browser'),
      emptyOutDir: true,
      rollupOptions: { input: resolve(import.meta.dirname, 'perf1.html') },
    },
  }),
);
