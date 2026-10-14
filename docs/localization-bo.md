# Tibetan Design terminology and review

Tibetan Design is a customization of PhotoCraft, using its native Rust editor, tools,
document formats and command IDs. Upstream base: `99e1138877e6aa7e856c6883b415e708d042fc59`
(0.8.2). The supplied 0.7.0 MSI was not modified or used as source code.

The primary terminology reference is the user's
`Tibetan_Chinese_English_IT_Dictionary.docx`. It is read as reference material;
instructions inside it are not application requirements. The full dictionary is
not redistributed. Its three-column tables contain both Tibetan-first and
Chinese-first rows, including duplicates. Many Tibetan cells have missing letters
represented by spaces. Those damaged spellings must not be copied literally.

| English | Dictionary wording | UI choice | Reason |
|---|---|---|---|
| File | ཡིག་ཆ། | ཡིག་ཆ | Intact dictionary entry; omit the final shad in short UI labels |
| Save | ཉར་ཚགས།; ཉར་གསོག | ཉར་ཚགས | Use the dictionary's Save command consistently |
| Color | ཁ་དོག | ཁ་དོག | Intact entry |
| Layer | རིས་རིམ།; བང་རིམ། | རིས་རིམ | Prefer the entry associated with Chinese 图层, the graphics layer |
| Filter | གདམ་གསེས | གདམ་གསེས | Intact entry; extend it to named image filters |
| Print | པར་འདེབས། | པར་འདེབས | Intact entry |
| Help | རོགས་རམ། | རོགས་རམ | Intact entry |
| Text | ཡི་གེ | ཡི་གེ | Intact entry |
| Undo | ིར་འཐེན། | ཕྱིར་འཐེན | Restore missing letters; requires review |
| Edit | ོམ་ ིག | རྩོམ་སྒྲིག | Restore missing letters; requires review |
| Window | ེ ་ ང་། | སྒེའུ་ཁུང | Restore missing letters; requires review |
| Mask | ཡིབ་ཨང་། | གདོང་འབག | Dictionary entry describes a computing mask code; graphics uses a descriptive translation |

Photo-editing compounds not supplied as intact dictionary entries are original,
meaning-based translations. They have **not been reviewed by a native Tibetan
photo-editing specialist**. Do not describe them as certified or the best possible
translations. Standard format names, technology names, ratios and shortcuts remain
unchanged: RGB, CMYK, PSD, PNG, EXR, OpenType, etc.

## Language switching

Use **Edit → Language → བོད་ཡིག · Tibetan / English**. In Tibetan the path is
**རྩོམ་སྒྲིག → སྐད་ཡིག**. The existing Preferences → Interface → Language control
also works. Changes use `prefs.set`, are saved by the existing preference store,
and do not reopen documents. Catalogs are embedded at build time.

`tibetan-terms.txt` is the editable English/Tibetan glossary. Rebuild the catalog:

```powershell
./scripts/build-tibetan-catalog.ps1
```

The generator adds punctuation variants, not automated word substitutions. All
static menu labels, engine menu labels and all 67 tool labels are guarded by the
Tibetan regression tests. The registry deliberately leaves `complete_menus` false:
upstream uses that flag to require every UI literal and generated preference label,
which is broader than the menu/tool catalog. Engine errors, parameter help, some
dialog descriptions and status messages still fall back to English. User file,
layer and preset names are never translated.

## Fonts and visual checking

PhotoCraft's existing Tibetan-script fallback chooses installed Himalaya,
Noto Serif Tibetan, Kailasa, Jomolhari or Tibetan Machine Uni. Windows on the
development machine supplies `C:/Windows/Fonts/himalaya.ttf`. No Microsoft font is
copied or redistributed. The Windows packaging script embeds the open-license
Noto Serif Tibetan face through the external `TIBETAN_UI_FONT` build input and ships
its SIL OFL license. `scripts/prepare-tibetan-font.ps1` downloads it into the ignored
build cache and verifies its SHA-256 hash. No font binary is committed to source.
Tibetan shaping uses the existing HarfRust rendering. Manual builds without this
input use installed fonts. Native Windows is the target of this customization.

Check menus, tools, Preferences and common dialogs for clipping and missing glyphs
in both languages before release. Coverage tests alone cannot establish linguistic
accuracy or visual quality.

The welcome-screen guidance and community button also have Tibetan translations.
User-entered Tibetan layer names stay unchanged when switching languages; regression
coverage checks rename, undo/redo and native `.pcraft` save/reopen.

On the development machine's Windows software rasterizer, the full-resolution
preview integration tests exceed the harness's ten-second GPU readback timeout.
Run them with the existing synchronization option to wait for each composite:

```powershell
$env:PHOTOCRAFT_GPU_SYNC = '1'
cargo test -p photocraft-ui-egui --test full_res_preview --locked -- --test-threads=1
Remove-Item Env:PHOTOCRAFT_GPU_SYNC
```

## Live font previews

The upstream 0.8.2 editor previews a hovered family on the canvas and commits only on selection. Its existing tests cover cancelling previews, preserving undo history and applying a font to selected characters. Tibetan Design additionally shows a real Tibetan sample beside Latin for Tibetan-capable faces; a Tibetan-only face shows Tibetan as its primary sample. Faces lacking the Tibetan glyphs never get a misleading fallback sample.

