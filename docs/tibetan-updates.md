# Tibetan Design Photo update notifications

The update-enabled desktop release checks the public GitHub latest-release API in a worker thread, 15 seconds after startup, at most once per day. A new stable version is offered only when a matching Windows or Mac installer asset exists. A 12-second timeout and 64 KiB response limit bound network work. Drafts, prereleases, older/equal versions, and unexpected release/download URLs are rejected. No document data, account identifiers, or installed-font inventory is sent.

Help → Check for Updates runs a manual check. Help → Automatically Check for Updates toggles automatic checks. The prompt offers Download Update, Later, and Skip This Version. Download Update opens the official release page; it does not download, execute, or install software automatically. Skip preferences and automatic checks persist with egui's per-user window settings. Manual checks can reveal a skipped release. Offline startup checks are quiet; manual errors are shown.

Existing installations without this feature must install the update-enabled version once. Thereafter, publishing a newer stable GitHub release with the appropriate installer asset makes it discoverable automatically. Release tags and the embedded application version must use matching semantic versions.

The highlighted English / བོད་ཡིག menu sits immediately before Help on Windows and in the native Mac menu bar. Its two choices use the existing persisted language commands.

Validation and new installers are pending.
