import { t, type MsgKey } from '../i18n';
import type { SearchRecord } from './palette-search';
export interface PaletteRecord extends SearchRecord {
  action?: string;
  argument?: string;
  origin?: 'inventory' | 'project';
  toolId?: number;
  shortcut?: string;
}
const COMMANDS = [
  ['new', 'palette.new', 'nouveau projet neues projekt'],
  ['open', 'palette.open', 'ouvrir datei offnen', '⌘/Ctrl O'],
  ['import', 'palette.import', 'importer dessin zeichnung'],
  ['save', 'palette.save', 'enregistrer sauvegarder speichern', '⌘/Ctrl S'],
  ['close', 'palette.close', 'fermer schließen'],
  ['undo', 'palette.undo', 'annuler rückgängig', '⌘/Ctrl Z'],
  ['redo', 'palette.redo', 'retablir wiederholen', '⌘/Ctrl ⇧ Z'],
  ['add-profile', 'palette.profile', 'contour profil außenkontur'],
  ['add-pocket', 'palette.pocket', 'poche tasche'],
  ['add-drill', 'palette.drill', 'percage trou bohren'],
  ['duplicate', 'palette.duplicate', 'dupliquer kopieren'],
  ['delete', 'palette.delete', 'supprimer löschen'],
  ['sidebar-stock', 'palette.stock', 'brut material rohteil'],
  ['sidebar-layers', 'palette.layers', 'calques geometrie ebenen'],
  ['sidebar-text', 'palette.text', 'texte schrift'],
  ['sidebar-operations', 'palette.operations', 'operations usinage bearbeitungen'],
  ['cad', 'palette.cad', 'dogbone t bone preparation geometrie eckenfreistellung'],
  ['nav-machine', 'palette.machine', 'machine maschine'],
  ['nav-tools', 'palette.tools', 'outils fraises werkzeugbibliothek'],
  ['nav-settings', 'palette.settings', 'preferences reglages einstellungen'],
  ['nav-help', 'palette.help', 'aide raccourcis hilfe shortcuts'],
  ['nav-about', 'palette.about', 'a propos uber'],
  ['view-2d', 'palette.view2d', 'dessin drawing'],
  ['view-3d', 'palette.view3d', 'simulation preview'],
  ['generate', 'palette.generate', 'generer calculer toolpath berechnen'],
  ['cancel', 'palette.cancel', 'arreter stop abbrechen'],
  ['warnings', 'palette.warnings', 'avertissements alertes warnungen'],
  ['export-front', 'palette.export_front', 'exporter telecharger download sbp gcode vorderseite'],
  ['export-back', 'palette.export_back', 'exporter retour rückseite'],
  ['templates', 'palette.templates', 'modeles recettes vorlagen'],
  ['capture-template', 'palette.capture', 'sauver modele vorlage speichern'],
] as const;
export function commandCatalog(): PaletteRecord[] {
  return COMMANDS.map(([action, key, aliases, shortcut]) => ({
    id: `command:${action}`,
    category: 'command',
    label: t(key as MsgKey),
    aliases,
    action,
    shortcut,
  }));
}
export const TOOL_ALIASES: Record<string, string> = {
  endmill: 'endmill fraise cylindrique schaftfraser',
  v_bit: 'v bit fraise en v v fraser',
  cone: 'cone conical fraise en v',
  engraver: 'engraver graveur gravierfraser',
  drill: 'drill foret bohrer',
  ball_nose: 'ball nose boule kugelfraser',
  compression: 'compression',
  bull_nose: 'bull nose rayon eckradius',
  laser_beam: 'laser',
  plasma_torch: 'plasma',
  drag_knife: 'drag knife couteau schleppmesser',
  form_profile: 'form profile profil',
  thread_mill: 'thread mill filetage gewindefraser',
};

export const TEMPLATE_KIND_ALIASES: Record<string, string> = {
  profile: 'profile contour profil',
  pocket: 'pocket poche tasche',
  planar_surface: 'planar surface surfacing surfaçage planfrasen spoilboard',
};
