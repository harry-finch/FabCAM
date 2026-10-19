// Shared CDP helpers for the runtime UI smoke scripts (ui-smoke-tabs.mjs,
// ui-smoke-table.mjs). Selectors in the scripts prefer accessible names
// (landmark aria-labels, button text, aria-pressed) over CSS classes.
export const CDP_PORT = process.env.CDP_PORT ?? '9224';
export const APP_URL = process.env.APP_URL ?? 'http://localhost:4173/';
export const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

export async function connect() {
  const res = await fetch(`http://127.0.0.1:${CDP_PORT}/json/new?${encodeURIComponent(APP_URL)}`, { method: 'PUT' });
  const target = await res.json();
  const ws = new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((r, j) => { ws.addEventListener('open', r); ws.addEventListener('error', j); });
  const exceptions = [];
  ws.addEventListener('message', (ev) => {
    const msg = JSON.parse(ev.data);
    if (msg.method === 'Runtime.exceptionThrown') {
      exceptions.push(msg.params.exceptionDetails?.exception?.description ?? msg.params.exceptionDetails?.text ?? 'unknown');
    }
  });
  let id = 1;
  ws.send(JSON.stringify({ id: id++, method: 'Runtime.enable', params: {} }));

  // A frozen effect loop never answers: every evaluate has a hard timeout.
  const evalIn = (expression, timeoutMs = 6000) => new Promise((resolve, reject) => {
    const myId = id++;
    const timer = setTimeout(() => reject(new Error(`FROZEN: evaluate #${myId} timed out`)), timeoutMs);
    const onMsg = (ev) => {
      const msg = JSON.parse(ev.data);
      if (msg.id === myId) {
        clearTimeout(timer);
        ws.removeEventListener('message', onMsg);
        resolve(msg.result?.result?.value);
      }
    };
    ws.addEventListener('message', onMsg);
    ws.send(JSON.stringify({ id: myId, method: 'Runtime.evaluate', params: { expression, returnByValue: true } }));
  });

  // Click the <button> inside the landmark `nav[aria-label=nav]` (or any
  // element when nav is null) whose trimmed text starts with `text`.
  const clickButton = (text, scope = null) => evalIn(`(() => {
    const root = ${scope ? `document.querySelector(${JSON.stringify(scope)})` : 'document'};
    const b = root && [...root.querySelectorAll('button')].find((x) => x.textContent.trim().startsWith(${JSON.stringify(text)}));
    if (!b) return false; b.click(); return true; })()`);
  return { ws, evalIn, clickButton, exceptions };
}

export const MAIN_NAV = 'nav[aria-label="Main areas"]';
export const MACHINE_NAV = 'nav[aria-label="Machine sections"]';
