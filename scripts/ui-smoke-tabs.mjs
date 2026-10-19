// CDP smoke test for the Atelier shell: boots the built app and checks
//  - the Welcome screen is shown with no active project (workspace hidden),
//  - creating a project reveals the workspace and hides Welcome,
//  - every major area opens, stays responsive (the d41i/tab-freeze bug was
//    an infinite $effect loop that hard-froze the app on panel mount) and
//    shows exactly one visible panel,
//  - Machine > General (settings form) and Machine > Tools (tooling) render,
//  - no uncaught JS exceptions occur anywhere in the run.
import { connect, sleep, MAIN_NAV, MACHINE_NAV } from './ui-smoke-lib.mjs';

const { evalIn, clickButton, exceptions } = await connect();
let failed = false;
const check = (label, ok) => { console.log(`${label}: ${ok ? 'OK' : 'FAIL'}`); if (!ok) failed = true; };

// Boot: the six-area navigation landmark is present.
let booted = false;
for (let i = 0; i < 30 && !booted; i++) {
  await sleep(500);
  booted = await evalIn(`document.querySelectorAll('${MAIN_NAV} button.main-tab').length === 6`).catch(() => false);
}
check('boot (6 areas)', booted);
if (!booted) process.exit(1);

// Welcome screen with no active project.
const welcome = await evalIn(`({
  welcome: !!document.querySelector('main.welcome'),
  hasNew: [...document.querySelectorAll('main.welcome button')].some((b) => b.textContent.trim().startsWith('New project')),
  workspaceHidden: getComputedStyle(document.querySelector('main.split')).display === 'none',
})`);
check('welcome shown, workspace hidden', welcome.welcome && welcome.hasNew && welcome.workspaceHidden);

// Create a project from Welcome.
check('click "New project"', await clickButton('New project', 'main.welcome'));
await sleep(2000);
const created = await evalIn(`({
  welcomeGone: !document.querySelector('main.welcome'),
  workspaceVisible: getComputedStyle(document.querySelector('main.split')).display !== 'none',
})`);
check('project created, workspace visible', created.welcomeGone && created.workspaceVisible);

// Navigate every major area; each must answer a probe and show one panel.
for (const [label, name] of [['Tool library', 'tools'], ['Machine', 'machine'], ['Settings', 'settings'], ['Help', 'help'], ['About', 'about']]) {
  await clickButton(label, MAIN_NAV);
  await sleep(1500);
  const alive = await evalIn('1 + 1').then((v) => v === 2).catch(() => false);
  if (!alive) { console.log(`area ${name}: FROZEN`); process.exit(2); }
  const visiblePanels = await evalIn(`[...document.querySelectorAll('main.tab-panel')].filter((p) => getComputedStyle(p).display !== 'none').length`);
  const splitHidden = await evalIn(`getComputedStyle(document.querySelector('main.split')).display === 'none'`);
  check(`area ${name}: responsive, one panel, workspace hidden`, visiblePanels === 1 && splitHidden);
}

// Machine sections: General (settings form) then Tools (tooling).
await clickButton('Machine', MAIN_NAV);
await sleep(500);
await clickButton('General', MACHINE_NAV);
await sleep(1000);
check('machine > general: settings form', await evalIn(`!!document.querySelector('.embedded-shell .grid')`));
await clickButton('Tools', MACHINE_NAV);
await sleep(800);
const tooling = await evalIn(`!!document.querySelector('.tooling')`);
const pressed = await evalIn(`document.querySelector('${MACHINE_NAV} button[aria-pressed="true"]')?.textContent.trim()`);
check('machine > tools: tooling rendered', tooling && pressed === 'Tools');

// Back to the project workspace.
await clickButton('Project', MAIN_NAV);
await sleep(800);
check('project area: workspace visible again', await evalIn(`getComputedStyle(document.querySelector('main.split')).display !== 'none'`));

console.log('page exceptions:', exceptions.length === 0 ? 'none' : exceptions.slice(0, 3).join(' | '));
if (exceptions.length > 0) failed = true;
process.exit(failed ? 3 : 0);
