import assert from "node:assert/strict";
import test from "node:test";
import { createRuntime } from "./harness.mjs";

// A record budget, independent of wall-clock scheduling. This does not claim
// that callbacks, JS parsing or IR conversion themselves are preemptible.
test("live cancellation is observed within 64 subsequent record checkpoints", () => {
  const rt = createRuntime();
  rt.eval(`
    __ivacRun.recordsSinceAbortCheck = 0;
    __ivac.abortCheck = function () { return false; };
    for (var i = 0; i < 137; ++i) __ivacCheckAbort();
    __ivac.abortCheck = function () { return true; };
  `);
  assert.throws(
    () => rt.eval("for (var i = 0; i < 64; ++i) __ivacCheckAbort();"),
    /__IVAC_CANCELLED__/
  );
});

test("uncancelled record checkpoints preserve output and do not halt", () => {
  const rt = createRuntime();
  rt.eval(`
    __ivac.abortCheck = function () { return false; };
    for (var i = 0; i < 4096; ++i) { __ivacCheckAbort(); writeln(i); }
  `);
  assert.equal(rt.output(), Array.from({length: 4096}, (_, i) => i + "\n").join(""));
});
