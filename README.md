# Declutter

A minimal, keyboard-driven desktop application for quickly sorting through files in a folder. Built with Tauri and Svelte.

## Features

- **Quick file triage**: Review files one at a time with simple keyboard shortcuts
- **File previews**: Supports images, text files, and shows icons for other file types
- **Non-destructive**: Files marked for deletion are renamed with a `DELETE_` prefix, not actually deleted
- **Undo support**: Made a mistake? Press backspace to undo your last action
- **Progress tracking**: See how many files you've kept vs marked for deletion

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `→` (Right Arrow) | Keep the current file |
| `←` (Left Arrow) | Mark file for deletion |
| `⌫` (Backspace) | Undo last action |

## Prerequisites

Before running the application, make sure you have the following installed:

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### macOS

```bash
xcode-select --install
```

### Linux

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## Installation

1. Clone the repository:

```bash
cd folder-cleanup-swipe
```

2. Install dependencies:

```bash
npm install
```

## Development

Run the application in development mode:

```bash
npm run tauri dev
```

This will start the Vite dev server and launch the Tauri application with hot-reload enabled.

## Building

Build the application for production:

```bash
npm run tauri build
```

The built application will be available in `src-tauri/target/release/bundle/`.

## How It Works

1. **Select a folder**: Click "Choose Folder" to select a directory containing files you want to triage
2. **Review files**: Each file is displayed one at a time with a preview
3. **Make decisions**: Use arrow keys to keep (→) or mark for deletion (←)
4. **Review summary**: After all files are processed, see a summary of your decisions
5. **Clean up**: Files marked for deletion have been renamed with `DELETE_` prefix - review and permanently delete them using your file manager

## Project Structure

```
folder-cleanup-swipe/
├── src/                    # Svelte frontend
│   ├── App.svelte          # Main application component
│   ├── main.ts             # Entry point
│   ├── app.css             # Global styles
│   ├── types.ts            # TypeScript types
│   └── lib/
│       ├── FolderSelect.svelte   # Folder selection screen
│       ├── FileViewer.svelte     # Main triage view
│       ├── Preview.svelte        # File preview component
│       └── Summary.svelte        # Completion summary
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Application entry
│   │   └── lib.rs          # Tauri commands
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
└── package.json            # Node.js dependencies
```

## Tech Stack

- **Frontend**: Svelte 4, TypeScript, Vite
- **Backend**: Tauri 1.6, Rust
- **Styling**: CSS custom properties (no framework)

## License

MIT


