# Tibetan Design

Native Windows image editor based on [PhotoCraft](https://github.com/storytold/photocraft),
with the supplied StudyTibetan logo, Tibetan menu/tool translations and English.
The editing engine and file-format behavior are inherited from the upstream source.
Original MIT/Apache notices and contributor credits remain included.

Switch languages using **Edit → Language**. Tibetan terminology and remaining
English fallback areas are documented in [localization-bo.md](docs/localization-bo.md).

Build prerequisites: Rust 1.95 or newer, Microsoft C++ Build Tools with the Windows
SDK. For MSI packaging, install .NET and WiX 5 with its UI and Util extensions.

```powershell
cargo test -p photocraft-ui-egui --lib i18n::
cargo run -p photocraft
pwsh packaging/windows/package.ps1 -Arch x64
```

Cargo crate/binary names and `.pcraft` remain compatible with upstream. The installed
desktop executable is named `Tibetan Design.exe`; the installer has its own upgrade
and component GUIDs, application registrations and settings directory.

The logo was supplied by the user for this application. No public license or
ownership claim is inferred for the supplied logo or dictionary. This source is a
local customization; it has not been published to a GitHub account.

## Building in an approved Windows environment

If Windows reports `An Application Control policy has blocked this file (os error
4551)`, a generated build helper has been blocked by the operating system. See
[Windows source builds](docs/development.md#windows-source-builds). Use an approved
development environment rather than relocating or renaming the blocked helper.

The included `.github/workflows/tibetan-design-windows.yml` workflow can build on a
GitHub Windows runner after this source is placed in a repository you control.
From **Actions → Tibetan Design Windows → Run workflow**, run the build and download
the **Tibetan-Design-Windows** artifact. It contains the MSI and portable ZIP. The
workflow does not publish a release. Without signing credentials, packages are unsigned.
