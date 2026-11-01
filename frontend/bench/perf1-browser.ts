import { runFrontendBenchmarks } from './perf1-run';
const target = document.getElementById('results')!;
runFrontendBenchmarks(() => {
  const p = performance as Performance & { memory?: { usedJSHeapSize: number } };
  return p.memory?.usedJSHeapSize ?? null;
}).then(
  (rows) => {
    target.textContent = JSON.stringify({ environment: navigator.userAgent, rows }, null, 2);
  },
  (error) => {
    target.textContent = String(error);
  },
);
