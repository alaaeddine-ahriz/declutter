import { writable } from 'svelte/store';
import type { Settings, KeyCombo } from '../../types';

const DEFAULT_SETTINGS: Settings = {
    keybindings: {
        keep: { key: 'ArrowRight', modifiers: { ctrl: false, alt: false, meta: false, shift: false } },
        delete: { key: 'ArrowLeft', modifiers: { ctrl: false, alt: false, meta: false, shift: false } },
        undo: { key: 'z', modifiers: { ctrl: false, alt: false, meta: true, shift: false } },
        preview: { key: ' ', modifiers: { ctrl: false, alt: false, meta: false, shift: false } }, // ' ' is space
    },
};

const STORE_KEY = 'declutter-settings';

function loadSettings(): Settings {
    try {
        const stored = localStorage.getItem(STORE_KEY);
        if (stored) {
            // Merge with defaults to ensure all keys exist if we add new ones later
            const parsed = JSON.parse(stored);
            return {
                ...DEFAULT_SETTINGS,
                keybindings: {
                    ...DEFAULT_SETTINGS.keybindings,
                    ...parsed.keybindings,
                },
            };
        }
    } catch (e) {
        console.warn('Failed to load settings:', e);
    }
    return DEFAULT_SETTINGS;
}

function createSettingsStore() {
    const { subscribe, set, update } = writable<Settings>(loadSettings());

    return {
        subscribe,
        updateKeybinding: (action: keyof Settings['keybindings'], combo: KeyCombo) => {
            update((settings) => {
                const newSettings = {
                    ...settings,
                    keybindings: {
                        ...settings.keybindings,
                        [action]: combo,
                    },
                };
                saveSettings(newSettings);
                return newSettings;
            });
        },
        reset: () => {
            set(DEFAULT_SETTINGS);
            saveSettings(DEFAULT_SETTINGS);
        }
    };
}

function saveSettings(settings: Settings) {
    try {
        localStorage.setItem(STORE_KEY, JSON.stringify(settings));
    } catch (e) {
        console.warn('Failed to save settings:', e);
    }
}

export const settings = createSettingsStore();

export function checkKeyCombo(event: KeyboardEvent, combo: KeyCombo): boolean {
    // Normalize key: ' ' for Space
    const eventKey = event.key;

    if (eventKey.toLowerCase() !== combo.key.toLowerCase()) return false;

    // Strict modifier check? Or loose?
    // Let's go with exact match for now to avoid accidental triggers
    return (
        event.ctrlKey === combo.modifiers.ctrl &&
        event.altKey === combo.modifiers.alt &&
        event.metaKey === combo.modifiers.meta &&
        event.shiftKey === combo.modifiers.shift
    );
}

export function formatKeyCombo(combo: KeyCombo): string {
    const parts = [];
    if (combo.modifiers.meta) parts.push('⌘');
    if (combo.modifiers.ctrl) parts.push('Ctrl');
    if (combo.modifiers.alt) parts.push('Alt');
    if (combo.modifiers.shift) parts.push('Shift');

    let key = combo.key;
    if (key === ' ') key = 'Space';
    if (key === 'ArrowRight') key = '→';
    if (key === 'ArrowLeft') key = '←';
    if (key === 'ArrowUp') key = '↑';
    if (key === 'ArrowDown') key = '↓';

    parts.push(key.toUpperCase());
    return parts.join(combo.modifiers.meta ? '' : '+');
}
