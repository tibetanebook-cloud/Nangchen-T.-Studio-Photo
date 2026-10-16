//! Non-blocking release notifications. Network access belongs to the native service.
use crate::PhotocraftApp;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

pub const RELEASES: &str = "https://github.com/tibetanebook-cloud/tibetan-design/releases/latest";
const DAY: u64 = 24 * 60 * 60;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub version: String,
    pub url: String,
}
pub type Pending = Receiver<Result<Option<Release>, String>>;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub automatic: bool,
    pub last_check: u64,
    pub skipped_version: String,
}
impl Default for Preferences {
    fn default() -> Self {
        Self { automatic: true, last_check: 0, skipped_version: String::new() }
    }
}
#[derive(Default)]
pub(crate) struct State {
    loaded: bool,
    started: Option<f64>,
    pending: Option<Pending>,
    manual: bool,
    offer: Option<Release>,
    message: Option<String>,
}
fn key() -> egui::Id {
    egui::Id::new("tibetan-design-update-preferences")
}
fn initialize(app: &mut PhotocraftApp, ctx: &egui::Context) {
    if !app.update_state.loaded {
        if let Some(saved) = ctx.data_mut(|d| d.get_persisted::<Preferences>(key())) {
            app.ui.update_preferences = saved;
        }
        app.update_state.loaded = true;
    }
}
fn persist(app: &PhotocraftApp, ctx: &egui::Context) {
    ctx.data_mut(|d| d.insert_persisted(key(), app.ui.update_preferences.clone()));
}
fn now_seconds() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
    }
    #[cfg(target_arch = "wasm32")]
    {
        0
    }
}
fn due(p: &Preferences, now: u64) -> bool {
    p.automatic && (p.last_check == 0 || p.last_check > now || now.saturating_sub(p.last_check) >= DAY)
}
pub fn request(app: &mut PhotocraftApp, ctx: &egui::Context, manual: bool) -> Result<(), String> {
    initialize(app, ctx);
    if app.update_state.pending.is_some() {
        app.update_state.manual |= manual;
        return Ok(());
    }
    let check = app.services.check_updates.as_ref().ok_or("Update checks are unavailable on this platform")?;
    let pending = check(ctx)?;
    app.update_state.pending = Some(pending);
    app.update_state.manual = manual;
    app.ui.update_preferences.last_check = now_seconds();
    persist(app, ctx);
    if manual {
        app.ui.status = tl!("Checking for updates…").into();
    }
    ctx.request_repaint_after(Duration::from_millis(200));
    Ok(())
}
pub fn toggle_automatic(app: &mut PhotocraftApp, ctx: &egui::Context) {
    initialize(app, ctx);
    app.ui.update_preferences.automatic = !app.ui.update_preferences.automatic;
    persist(app, ctx);
}
pub(crate) fn tick(app: &mut PhotocraftApp, ctx: &egui::Context) {
    initialize(app, ctx);
    if app.services.check_updates.is_none() {
        return;
    }
    let elapsed = ctx.input(|i| i.time);
    let started = *app.update_state.started.get_or_insert(elapsed);
    let now = now_seconds();
    if app.update_state.pending.is_none() && elapsed - started >= 15.0 && due(&app.ui.update_preferences, now) {
        if let Err(error) = request(app, ctx, false) {
            // A failed worker start must not retry on every UI frame.
            app.ui.update_preferences.last_check = now;
            persist(app, ctx);
            log::warn!("update check could not start: {error}");
        }
    }
    let received = app.update_state.pending.as_ref().map(|rx| rx.try_recv());
    let result = match received {
        Some(Ok(result)) => Some(result),
        Some(Err(TryRecvError::Disconnected)) => Some(Err("Update worker stopped".into())),
        _ => None,
    };
    if let Some(result) = result {
        app.update_state.pending = None;
        match result {
            Ok(Some(release)) if app.update_state.manual || release.version != app.ui.update_preferences.skipped_version => {
                app.update_state.offer = Some(release);
            }
            Ok(None) if app.update_state.manual => {
                app.update_state.message = Some(tl!("No newer release is available for this computer.").into());
            }
            Err(error) => {
                log::debug!("update check failed: {error}");
                if app.update_state.manual {
                    app.update_state.message = Some(tl!("Could not check for updates. Please try again later.").into());
                }
            }
            _ => {}
        }
    }
    if app.update_state.pending.is_some() {
        ctx.request_repaint_after(Duration::from_millis(200));
    } else if app.ui.update_preferences.automatic {
        let remaining = if elapsed - started < 15.0 { 15 } else { DAY.saturating_sub(now.saturating_sub(app.ui.update_preferences.last_check)).max(1) };
        ctx.request_repaint_after(Duration::from_secs(remaining));
    }
}
pub(crate) fn show(app: &mut PhotocraftApp, ctx: &egui::Context) {
    if !app.ui.dialogs.is_empty() || app.discard.is_some() || app.camera_raw.is_some() {
        return;
    }
    let offer = app.update_state.offer.clone();
    let message = app.update_state.message.clone();
    if offer.is_none() && message.is_none() {
        return;
    }
    let mut open = true;
    let mut visit = None;
    let mut dismiss = false;
    egui::Window::new(tl!("Software Update"))
        .id(egui::Id::new("tibetan-software-update"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(420.0)
        .show(ctx, |ui| {
            if let Some(release) = &offer {
                ui.heading(tl!("A new version is available."));
                ui.label(crate::i18n::fmt(tl!("Installed version: {version}"), &[("version", env!("CARGO_PKG_VERSION"))]));
                ui.label(crate::i18n::fmt(tl!("Latest version: {version}"), &[("version", &release.version)]));
                ui.label(tl!("Download the new installer from the official release page."));
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if crate::widgets::primary_button(ui, tl!("Download Update"), 120.0).clicked() {
                        visit = Some(release.url.clone());
                        dismiss = true;
                    }
                    if ui.button(tl!("Later")).clicked() {
                        dismiss = true;
                    }
                    if ui.button(tl!("Skip This Version")).clicked() {
                        app.ui.update_preferences.skipped_version = release.version.clone();
                        dismiss = true;
                    }
                });
            } else if let Some(message) = &message {
                ui.label(message);
                if ui.button(tl!("Open Downloads")).clicked() {
                    visit = Some(RELEASES.into());
                    dismiss = true;
                }
                if ui.button(tl!("Close")).clicked() {
                    dismiss = true;
                }
            }
            ui.separator();
            ui.checkbox(&mut app.ui.update_preferences.automatic, tl!("Check for updates automatically"));
        });
    persist(app, ctx);
    if let Some(url) = visit {
        crate::links::open(app, ctx, &url);
    }
    if dismiss || !open {
        app.update_state.offer = None;
        app.update_state.message = None;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };
    fn mock(result: Result<Option<Release>, String>) -> PhotocraftApp {
        PhotocraftApp::new(
            photocraft_engine::Session::new(),
            crate::Services {
                check_updates: Some(Box::new(move |_| {
                    let (tx, rx) = mpsc::channel();
                    tx.send(result.clone()).unwrap();
                    Ok(rx)
                })),
                ..Default::default()
            },
        )
    }
    #[test]
    fn update_schedule_is_daily_and_respects_opt_out_and_clock_changes() {
        let mut p = Preferences::default();
        assert!(due(&p, DAY));
        p.last_check = DAY;
        assert!(!due(&p, DAY + 1));
        assert!(due(&p, DAY * 2));
        assert!(due(&p, 1));
        p.automatic = false;
        assert!(!due(&p, DAY * 3));
    }
    #[test]
    fn manual_check_reveals_a_skipped_version_and_keeps_documents() {
        let ctx = egui::Context::default();
        let release = Release { version: "9.0.0".into(), url: RELEASES.into() };
        let mut app = mock(Ok(Some(release.clone())));
        initialize(&mut app, &ctx);
        app.ui.update_preferences.skipped_version = release.version.clone();
        app.run("file.new", serde_json::json!({"width":16,"height":16})).unwrap();
        let before = app.session.active().unwrap().doc.id;
        request(&mut app, &ctx, false).unwrap();
        tick(&mut app, &ctx);
        assert!(app.update_state.offer.is_none());
        request(&mut app, &ctx, true).unwrap();
        tick(&mut app, &ctx);
        assert_eq!(app.update_state.offer, Some(release));
        assert_eq!(app.session.active().unwrap().doc.id, before);
    }
    #[test]
    fn automatic_errors_are_quiet_and_manual_errors_are_visible() {
        let ctx = egui::Context::default();
        let mut app = mock(Err("offline".into()));
        request(&mut app, &ctx, false).unwrap();
        tick(&mut app, &ctx);
        assert!(app.update_state.message.is_none());
        request(&mut app, &ctx, true).unwrap();
        tick(&mut app, &ctx);
        assert!(app.update_state.message.is_some());
    }
    #[test]
    fn preferences_persist_and_parallel_requests_are_suppressed() {
        let ctx = egui::Context::default();
        let mut app = mock(Ok(None));
        toggle_automatic(&mut app, &ctx);
        let mut restored = mock(Ok(None));
        initialize(&mut restored, &ctx);
        assert!(!restored.ui.update_preferences.automatic);
        let calls = Arc::new(AtomicUsize::new(0));
        let seen = calls.clone();
        let (tx, rx) = mpsc::channel();
        let slot = std::sync::Mutex::new(Some(rx));
        app.services.check_updates = Some(Box::new(move |_| {
            seen.fetch_add(1, Ordering::Relaxed);
            Ok(slot.lock().unwrap().take().unwrap())
        }));
        request(&mut app, &ctx, true).unwrap();
        request(&mut app, &ctx, true).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        drop(tx);
    }
}
