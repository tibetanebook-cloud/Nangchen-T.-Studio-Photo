# Tibetan Design Photo

Tibetan Design Photo is a Tibetan and English branded edition of the [PhotoCraft image editor](https://github.com/storytold/photocraft). It keeps the upstream Rust engine and file-format behavior, carries the supplied Tibetan Design Photo logo, and includes Tibetan interface translations. Original contributor credits and MIT/Apache notices are retained.

Choose **English** or **བོད་ཡིག · Tibetan** from the highlighted **English / བོད་ཡིག** menu beside **Help**. The selection uses the editor's existing language preference and remains available after restarting.

## Build and release

The upstream release workflow builds Windows and macOS packages. Pull requests run the repository CI, including Windows validation when UI or packaging files change. A GitHub release should be published only after CI and the release workflow succeed; the user-facing download page is [GitHub Releases](https://github.com/tibetanebook-cloud/Tibetan-Design-Photo/releases).

For local development, install Rust 1.95 or newer. Windows MSI packaging also needs the Windows SDK, .NET and WiX 5. On a Mac, install Xcode command-line tools and the Rust target for the Mac architecture. Commands and package signing requirements are documented in [upstream's release guide](https://github.com/storytold/photocraft/blob/main/docs/releasing.md).

The application source continues to use upstream crate names, executable names and file-format identifiers internally for compatibility. Those machine identifiers are not user-facing product branding; the desktop application is named Tibetan Design Photo.

## Branding and attribution

The supplied icon and Tibetan terminology are project assets. Existing upstream MIT/Apache notices and third-party asset licenses remain in the repository. See [ATTRIBUTION.md](ATTRIBUTION.md) and [NOTICE](NOTICE) for details.
