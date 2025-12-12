export interface FileInfo {
  path: string;
  name: string;
  file_type: "image" | "pdf" | "text" | "other";
  size: number;
}

export interface FileMetadata {
  path: string;
  name: string;
  file_type: string;
  size: number;
  extension: string;
}

export type AppState = "idle" | "triaging" | "confirming" | "complete" | "settings";

export interface Action {
  type: "keep" | "delete";
  fileIndex: number;
  originalPath: string;
}

export interface TriageResult {
  kept: number;
  deleted: number;
  total: number;
  savedSize?: number;
}

export interface DeleteResult {
  success: string[];
  failed: [string, string][]; // [path, error message]
}



export interface KeyCombo {
  key: string;
  modifiers: {
    ctrl: boolean;
    alt: boolean;
    meta: boolean;
    shift: boolean;
  };
}

export type TriageMode = 'classic' | 'explore';

export interface Settings {
  mode: TriageMode;
  keybindings: {
    keep: KeyCombo;
    delete: KeyCombo;
    undo: KeyCombo;
    preview: KeyCombo;
    // Explore mode bindings
    exploreNext: KeyCombo;
    explorePrevious: KeyCombo;
    exploreDelete: KeyCombo; // Toggle mark for delete
  };
}
