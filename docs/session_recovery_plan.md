# Session Recovery Feature Plan

## Objective
Prevent data loss (triage progress) if the application crashes or is accidentally closed mid-session. The feature will automatically save the current state and offer to resume it upon the next launch.

## 1. Data Persistence Strategy

**Storage Mechanism**: JSON file in the application data directory.
**File Path**: `<appDataDir>/session_recovery.json` (Managed via Tauri `fs` and `path` APIs).

### Data Structure (`SavedSession`)

We need to persist enough data to restore the `App.svelte` state exactly where the user left off.

```typescript
import type { FileInfo, Action } from '../types';

interface SavedSession {
  version: 1; // For future migrations
  timestamp: number; // Unix timestamp of save
  folderPath: string;
  files: FileInfo[];
  currentIndex: number;
  actionHistory: Action[];
  keptCount: number;
  // Computed stats can be derived from actionHistory, but keptCount is stateful in App.svelte
}
```

## 2. Implementation Architecture

### A. Session Service (`src/lib/services/session.ts`)

Create a dedicated service to handle persistence logic:

*   `saveSession(state: SavedSession): Promise<void>`
    *   Writes the state to the JSON file.
    *   Should be debounced (e.g., 500ms) to avoid excessive disk writes during rapid usage.
*   `loadSession(): Promise<SavedSession | null>`
    *   Reads the JSON file.
    *   Validates the path exists (optional, but good practice).
    *   Returns the state or `null` if no file/invalid.
*   `clearSession(): Promise<void>`
    *   Deletes the JSON file.
    *   Called when the user completes a session, explicitly cancels, or chooses to discard a recovered session.

### B. Integration in `App.svelte`

1.  **Auto-Save Hook**:
    *   Use a reactive statement (`$:`) to trigger `saveSession` whenever relevant state changes (`currentIndex`, `actionHistory`, `keptCount`).
    *   Only save if `appState === 'triaging'`.

2.  **Startup Logic (`onMount`)**:
    *   Call `SessionService.loadSession()`.
    *   If a session exists:
        *   Set `appState` to a new state `"recovering"`.
        *   Show a modal/dialog: "Review session found for [folder]. [Resume] [Start Over]".

3.  **Resume Flow**:
    *   If user clicks "Resume":
        *   Restore all state (`files`, `currentIndex`, `actionHistory`, etc.).
        *   Set `appState` back to `"triaging"`.
    *   If user clicks "Start Over":
        *   Call `SessionService.clearSession()`.
        *   Set `appState` to `"idle"`.

## 3. Detailed Steps

1.  **Backend Config**:
    *   Ensure `tauri.conf.json` allowlist permits writing to `appData` or `appLocalData`. Current `fs` scope is `["**"]`, which is permissive but double-check `path` API access to resolve the app data directory.

2.  **Frontend Implementation**:
    *   Create `src/lib/services/session.ts`.
    *   Import and use in `App.svelte`.
    *   Create "Resume Session" UI (Modal or simplified view).

## 4. Edge Cases

*   **File Changes**: If the user moved/deleted files externally while the app was closed, the saved `files` list might be stale.
    *   *Mitigation*: On resume, we could quickly validate the current file exists before showing it, or just handle errors gracefully when `FileViewer` tries to load it.
*   **Corrupted Save File**: `JSON.parse` errors should be caught, and the session should be treated as non-existent (log the error).
