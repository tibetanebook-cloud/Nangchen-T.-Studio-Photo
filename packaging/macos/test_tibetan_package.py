"""Platform-independent regression checks for Tibetan Design Mac packaging."""
import hashlib
from pathlib import Path
import plistlib
import unittest

ROOT = Path(__file__).resolve().parents[2]


class TibetanMacPackageTests(unittest.TestCase):
    def test_bundle_identity_and_language_preserve_format_compatibility(self):
        info = plistlib.loads((ROOT / "packaging/macos/Info.plist.in").read_bytes())
        for key in ("CFBundleName", "CFBundleDisplayName", "CFBundleExecutable", "CFBundleIconFile"):
            self.assertEqual(info[key], "Tibetan Design")
        self.assertEqual(info["CFBundleIdentifier"], "com.tibetanebook.tibetan-design")
        self.assertIn("bo", info["CFBundleLocalizations"])
        self.assertIn("en", info["CFBundleLocalizations"])
        self.assertEqual(info["LSMinimumSystemVersion"], "11.0")
        native = info["UTExportedTypeDeclarations"][0]
        self.assertEqual(native["UTTypeIdentifier"], "ai.storyteller.photocraft.document")
        self.assertEqual(native["UTTypeTagSpecification"]["public.filename-extension"], ["pcraft"])

    def test_packaged_artifacts_have_matching_names_and_supplied_icon(self):
        package = (ROOT / "packaging/macos/package.sh").read_text(encoding="utf-8")
        verify = (ROOT / "packaging/macos/verify.sh").read_text(encoding="utf-8")
        for name in ("tibetan-design-$VERSION-macos-$ARCH.dmg", "tibetan-design-cli-$VERSION-macos-$ARCH.zip"):
            self.assertIn(name, package)
            self.assertIn(name, verify)
        self.assertIn("assets/app-icon/photocraft-1024.png", package)
        self.assertNotIn('cp "$ROOT/assets/app-icon/photocraft.icns"', package)
        self.assertNotIn('cp "$HERE/dmg/background.tiff"', package)
        self.assertIn('Contents/MacOS/Tibetan Design', package)
        self.assertIn('Contents/MacOS/Tibetan Design', verify)
        self.assertIn('hdiutil attach -readonly', verify)
        self.assertIn('codesign --verify --strict --deep', verify)

    def test_font_input_and_license_are_checked_and_distributed(self):
        shell = (ROOT / "scripts/prepare-tibetan-font.sh").read_text(encoding="utf-8")
        windows = (ROOT / "scripts/prepare-tibetan-font.ps1").read_text(encoding="utf-8").lower()
        for digest in ("060ec022b04c306de3f58d051fb0e1cf81a5b610c5910fbfc43bba154c057cda",
                       "0077789fc3cb20f358278ba4c6437eb14ade97d4d3e3281eaa42853efcd9dc8b"):
            self.assertIn(digest, shell)
            self.assertIn(digest, windows)
        license_bytes = (ROOT / "assets/app-icon/OFL-Tibetan.txt").read_bytes()
        self.assertEqual(hashlib.sha256(license_bytes).hexdigest(),
                         "0077789fc3cb20f358278ba4c6437eb14ade97d4d3e3281eaa42853efcd9dc8b")
        package = (ROOT / "packaging/macos/package.sh").read_text(encoding="utf-8")
        self.assertIn('. "$ROOT/scripts/prepare-tibetan-font.sh"', package)
        for notice in ("LICENSE-MIT", "LICENSE-APACHE", "NOTICE", "ATTRIBUTION.md", "OFL-Tibetan.txt"):
            self.assertIn(notice, package)


if __name__ == "__main__":
    unittest.main()
