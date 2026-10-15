/** Structural presentation contract; actual interaction/focus is verified in the browser. */
import { readFileSync } from 'node:fs';
import { it, expect } from 'vitest';
import en from '../i18n/messages/en.json';
import de from '../i18n/messages/de.json';
const source = (name: string) => readFileSync(new URL(`./${name}.svelte`, import.meta.url), 'utf8');
it('keeps Welcome focused on the original three entry actions', () => {
  const welcome = source('WelcomeScreen');
  expect(welcome).not.toMatch(/onPalette|palette\./);
  for (const key of ['shell.new', 'shell.open', 'shell.import', 'shell.recents'])
    expect(welcome).toContain(`t('${key}')`);
});
it('uses the existing menu callback and platform shortcut without a new launcher', () => {
  const menu = source('ProjectMenu');
  expect(menu).toContain('run(onPalette)');
  expect(menu).toContain("t('palette.menu')");
  expect(menu).toContain('paletteShortcut()');
});
it('documents open, result navigation, execution and dismissal in existing Help', () => {
  const help = source('ShortcutHelp');
  expect(help).toContain('paletteShortcut()');
  expect(help).toContain("t('palette.help_usage')");
  for (const catalog of [en, de]) {
    expect(catalog['palette.menu']).toMatch(/…$/);
    for (const key of ['↑/↓', 'Enter', 'Esc']) expect(catalog['palette.help_usage']).toContain(key);
  }
});
