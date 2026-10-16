"""Fast release-notification catalog validation before expensive native compilation."""
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
CATALOGS = ROOT / "crates/ui-egui/src/i18n"
updates = (ROOT / "crates/ui-egui/src/updates.rs").read_text(encoding="utf-8")
menus = (ROOT / "crates/ui-egui/src/menus.rs").read_text(encoding="utf-8")
required = set(re.findall(r'tl!\("([^"\n]+)"\)', updates))
required.update(re.findall(r'\("help\.(?:checkUpdates|autoUpdates)", "([^"\n]+)"', menus))
errors = []
count = 0
for path in sorted(CATALOGS.glob("*.tsv")):
    entries = {}
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) != 3 or not parts[1] or not parts[2]:
            errors.append(f"{path.name}:{number}: malformed catalog row")
            continue
        context, original, translated = parts
        key = (context, original)
        if key in entries:
            errors.append(f"{path.name}:{number}: duplicate {original!r}")
        entries[key] = translated
        if not context and original.endswith("…") != translated.endswith("…"):
            errors.append(f"{path.name}:{number}: ellipsis mismatch for {original!r}")
        if original in required:
            placeholders = lambda value: sorted(re.findall(r"\{[^{}]+\}", value))
            if placeholders(original) != placeholders(translated):
                errors.append(f"{path.name}:{number}: version placeholder mismatch")
        count += 1
    for original in sorted(required):
        if ("", original) not in entries:
            errors.append(f"{path.name}: missing release notification {original!r}")
if errors:
    raise SystemExit("\n".join(errors))
print(f"Release notification catalogs validated: {count} entries, {len(required)} required strings.")
