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

export interface Settings {
  keybindings: {
    keep: KeyCombo;
    delete: KeyCombo;
    undo: KeyCombo;
    preview: KeyCombo;
  };
}
