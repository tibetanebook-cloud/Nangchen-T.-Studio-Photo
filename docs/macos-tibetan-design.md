# Tibetan Design Photo for macOS

The upstream Rust editor supports macOS. Tibetan Design Photo keeps the upstream
Mac packaging and uses the supplied brand assets. The reusable release workflow
builds Apple Silicon and Intel packages; verify the completed workflow before
publishing a GitHub release.

## Build on GitHub

1. Merge the reviewed source update after its pull-request checks pass.
2. Run the repository's release workflow for the release branch.
3. Wait for both architecture jobs to finish successfully.
4. Download the corresponding artifact from the workflow run:
   - **Tibetan-Design-macOS-aarch64**: Apple Silicon (M-series).
   - **Tibetan-Design-macOS-x86_64**: Intel Macs.
5. Extract the artifact ZIP and open its `.dmg` on the matching Mac.
6. Drag **Tibetan Design Photo.app** to **Applications**.

Each artifact also contains a separate command-line ZIP; ordinary app users only
need the DMG. The application and DMG use the supplied Tibetan Design Photo logo.
The UI embeds the Noto Serif Tibetan Regular font (SHA-256
`ee97bf3dc56e813651db734c9f35f8f1d41e7e31acf5f7d893e64ad22b292446`) under the
SIL Open Font License so the bilingual language switch renders without a
system-installed Tibetan font. The bundle declares Tibetan and English, retains upstream format IDs,
and uses a separate Tibetan Design Photo application identifier/settings directory.

The build targets macOS 11.0 or newer, matching the upstream deployment target.
Compatibility on older supported systems still needs real-device testing. The
workflow runs translation/font tests, the native AppKit menu test, and verifies the actual app inside the DMG,
including its product name, CPU architecture, signature integrity and licenses.

## Signing and public distribution

Without Apple credentials, the workflow produces ad-hoc signed, **not notarized**
test packages. A successful build does not mean Gatekeeper will accept the app on
another Mac. Do not present these as a signed/notarized public release.

For a normal public distribution, obtain a Developer ID Application certificate
through your Apple Developer account and configure these repository Actions
secrets (never put certificates/passwords in source or chat):

- `APPLE_CERTIFICATE`: base64-encoded Developer ID Application `.p12` export.
- `APPLE_CERTIFICATE_PASSWORD`: password for that export.
- `KEYCHAIN_PASSWORD`: optional password for the temporary CI keychain.
- `APPLE_ID`: the Apple account used for notarization.
- `APPLE_PASSWORD`: its app-specific password.
- `APPLE_TEAM_ID`: your developer team ID.

With the complete credentials, packaging signs with Developer ID, submits to
Apple for notarization, staples tickets, and verifies Gatekeeper acceptance.

## Build on a Mac

Install Rust and Xcode command-line tools. From the project root:

```bash
rustup target add aarch64-apple-darwin   # Apple Silicon
bash packaging/macos/package.sh --arch aarch64
bash packaging/macos/verify.sh --arch aarch64
```

Use `x86_64` and `x86_64-apple-darwin` for Intel. A local universal build is also
supported by `--arch universal` after installing both Rust targets.

Test both languages, native Mac menus, Tibetan fonts, text editing, layer names,
file open/save and PSD import on real Macs before publishing the installers.
