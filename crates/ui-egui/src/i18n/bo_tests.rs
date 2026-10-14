use super::*;

#[test]
fn tibetan_resolves_regional_tags_and_dictionary_terms() {
    let bo = Lang::from_code("bo").expect("Tibetan registered");
    for tag in ["bo", "BO", "bo-CN", "bo-IN", "bo_CN.UTF-8", "bo-Tibt-CN"] {
        assert_eq!(lang_from_tag(tag), Some(bo), "{tag}");
    }
    for (source, expected) in [("File", "ཡིག་ཆ"), ("Save", "ཉར་ཚགས"), ("Layer", "རིས་རིམ"), ("Help", "རོགས་རམ")]
    {
        assert_eq!(tr(bo, source), expected);
        assert_eq!(tr(Lang::EN, source), source);
    }
    assert_eq!(dir_for_code("bo"), Dir::Ltr);
    assert!(crate::menus::is_live("app.language.bo"));
    assert!(crate::menus::is_live("app.language.en"));
    assert!(!crate::menus::is_live("app.language.unknown"));
    assert_eq!(fmt(tr(bo, "Opened {name}"), &[("name", "བོད་པར.psd")]), "བོད་པར.psd ཁ་ཕྱེ་ཟིན");
}

#[test]
fn tibetan_covers_static_menus_and_every_tool() {
    let bo = Lang::from_code("bo").expect("Tibetan registered");
    let cat = bo.catalog();
    let mut labels = std::collections::BTreeSet::new();
    for &(path, label, _, _) in crate::menu_catalog::CATALOG {
        labels.extend(path.iter().copied());
        labels.insert(label);
    }
    for &(_, label, path, _) in crate::menus::UI_COMMANDS {
        labels.extend(path.iter().copied());
        labels.insert(label);
    }
    for command in photocraft_engine::command_specs().iter().filter(|c| !c.menu.is_empty()) {
        labels.extend(command.menu.iter().copied());
        labels.insert(command.label);
    }
    labels.remove("---");
    let missing: Vec<_> = labels.iter().filter(|s| cat.plain(s).is_none()).collect();
    assert!(missing.is_empty(), "Tibetan menu translations missing: {missing:#?}");
    for tool in crate::state::Tool::ALL {
        assert_ne!(tr(bo, tool.label()), tool.label(), "{}", tool.label());
    }
}

#[test]
fn switching_languages_keeps_document_and_saves_preference() {
    let ctx = egui::Context::default();
    let mut app = crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
    app.run("file.new", serde_json::json!({"width": 32, "height": 32})).expect("new document");
    let doc = app.session.active().expect("active document").doc.id;
    for code in ["bo", "en", "bo"] {
        crate::menus::invoke(&mut app, &ctx, &format!("app.language.{code}"), serde_json::json!({})).expect("switch language");
        assert_eq!(app.session.prefs().interface.language, code);
        assert_eq!(app.session.active().expect("document survives").doc.id, doc);
        let items = crate::menus::menu_items(&app);
        assert!(items.iter().any(|item| item.id == format!("app.language.{code}") && item.checked == Some(true)));
    }
    let saved = serde_json::to_string(app.session.prefs()).expect("serialize preference");
    let restored: photocraft_engine::prefs::Preferences = serde_json::from_str(&saved).expect("restore preference");
    assert_eq!(restored.interface.language, "bo");
}

#[test]
fn tibetan_layer_names_survive_language_switch_undo_and_native_save() {
    let ctx = egui::Context::default();
    let mut app = crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
    app.run("file.new", serde_json::json!({"width": 32, "height": 32})).expect("new document");
    let name = "བོད་ཀྱི་རི་མོ། Tibetan ༡";
    let original = app.session.active().expect("document").doc.layers[0].name.clone();
    app.run("layer.renameLayer", serde_json::json!({"name": name})).expect("rename in Tibetan");
    app.run("edit.undo", serde_json::json!({})).expect("undo rename");
    assert_eq!(app.session.active().expect("document").doc.layers[0].name, original);
    app.run("edit.redo", serde_json::json!({})).expect("redo rename");
    for code in ["bo", "en"] {
        crate::menus::invoke(&mut app, &ctx, &format!("app.language.{code}"), serde_json::json!({})).expect("switch language");
        assert_eq!(app.session.active().expect("document").doc.layers[0].name, name);
    }
    let bytes = photocraft_format::save_to_bytes(&app.session.active().expect("document").doc, &Default::default()).expect("save native document");
    let restored = photocraft_format::load_from_bytes(&bytes).expect("reopen native document");
    assert_eq!(restored.layers[0].name, name);
}

#[test]
fn tibetan_welcome_screen_has_translations() {
    let bo = Lang::from_code("bo").expect("Tibetan registered");
    for source in [
        "open source",
        "Create a new document or open an existing file.",
        "Drop an image or PSD anywhere to open it.",
        "Join us on Discord",
        "Tibetan Design website",
    ] {
        assert_ne!(tr(bo, source), source, "{source}");
    }
}
