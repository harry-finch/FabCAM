// Runtime smoke for the Tool library: seed a 120-tool inventory via
// localStorage, then verify list rendering, search filtering, sorting,
// pagination and tool selection in the rendered Tool library area, and
// that no uncaught JS exception occurs.
import { connect, sleep, MAIN_NAV } from './ui-smoke-lib.mjs';

const { evalIn, clickButton, exceptions } = await connect();

// localStorage is per-origin: wait for first boot on the app origin, seed
// 120 inventory tools, reload.
await sleep(4000);
await evalIn(`
  const tools = Array.from({length: 120}, (_, i) => ({
    id: i + 1,
    name: 'tool-' + String(i + 1).padStart(3, '0'),
    kind: i % 10 === 0 ? 'plasma_torch' : 'endmill',
    diameter: ((i % 12) + 1) * 0.5,
    flutes: 2, speed: 18000, plungeRate: 100, feedRate: 800, coolant: 'off',
  }));
  const ws0 = { workspace_schema_version: 1, last_project: null, recent_projects: [],
    camera: null, panels: { left_width: 0, right_width: 360, bottom_height: 240 },
    per_project: {}, last_post_processor: 'linuxcnc', machine_profiles: [],
    tool_inventory: tools };
  localStorage.setItem('ivac-workspace', JSON.stringify(ws0));
  location.reload();
  true
`);
await sleep(5000);
const booted = await evalIn(`document.querySelectorAll('${MAIN_NAV} button.main-tab').length === 6`);
console.log('boot:', booted ? 'OK' : 'FAILED');
if (!booted) process.exit(1);

await clickButton('Tool library', MAIN_NAV);
await sleep(1500);

const snapshot = () => evalIn(`({
  items: document.querySelectorAll('.tool-list button[data-tool-id]').length,
  first: document.querySelector('.tool-list button[data-tool-id]')?.textContent.replace(/\\s+/g, ' ').trim() ?? null,
  pager: document.querySelector('.table-pager span')?.textContent ?? null,
  hasPager: !!document.querySelector('.table-pager'),
  count: document.querySelector('.tc-count')?.textContent ?? null,
})`);

const r1 = await snapshot();
console.log('page1:', JSON.stringify(r1));

// Filter to torches: 12 items, pager disappears.
await evalIn(`
  const inp = document.querySelector('.tc-search');
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set.call(inp, 'plasma');
  inp.dispatchEvent(new Event('input', { bubbles: true }));
  true`);
await sleep(400);
const r2 = await snapshot();
console.log('filtered:', JSON.stringify(r2));

// Clear the filter, sort by diameter, check ordering across pages.
await clickButton('Clear');
await sleep(300);
await evalIn(`
  const s = document.querySelector('label.sort select');
  s.value = 'diameter';
  s.dispatchEvent(new Event('change', { bubbles: true }));
  true`);
await sleep(300);
const r3 = await snapshot();
console.log('sorted:', JSON.stringify(r3));

// Next page: still sorted (diameters rise), pager advances.
await clickButton('Next', '.table-pager');
await sleep(300);
const r4 = await snapshot();
console.log('page2:', JSON.stringify(r4));

// Select the first tool on this page: detail pane shows it, row is pressed.
await evalIn(`document.querySelector('.tool-list button[data-tool-id]').click(); true`);
await sleep(400);
const r5 = await evalIn(`({
  id: document.querySelector('.tool-list button[data-tool-id]')?.dataset.toolId,
  pressed: document.querySelector('.tool-list button[data-tool-id]')?.getAttribute('aria-pressed'),
  heading: document.querySelector('section.detail h2')?.textContent ?? null,
})`);
console.log('selected:', JSON.stringify(r5));

console.log('page exceptions:', exceptions.length === 0 ? 'none' : exceptions.slice(0, 3).join(' | '));
const ok = r1.items === 50 && r1.pager === 'Page 1 of 3' && r1.count === '120 tools'
  && r2.items === 12 && r2.hasPager === false && r2.count === '12 of 120 tools'
  && r3.items === 50 && r3.count === '120 tools' && r3.first?.includes('Ø0.5 mm')
  && r4.pager === 'Page 2 of 3' && r4.first?.includes('Ø3 mm')
  && r5.pressed === 'true' && r5.heading?.startsWith(`#${r5.id} `)
  && exceptions.length === 0;
console.log(ok ? 'TABLE SMOKE: PASS' : 'TABLE SMOKE: FAIL');
process.exit(ok ? 0 : 3);
