//! HTTPS release metadata only: no installer download, execution or document access.
use photocraft_ui_egui::updates::{Pending, Release};
use semver::Version;
use serde_json::Value;
use std::time::Duration;
const API: &str = "https://api.github.com/repos/tibetanebook-cloud/tibetan-design/releases/latest";
const REPO: &str = "https://github.com/tibetanebook-cloud/tibetan-design";
fn platform_suffix() -> Option<&'static str> {
    if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        Some("windows-x64.msi")
    } else if cfg!(all(target_os = "windows", target_arch = "aarch64")) {
        Some("windows-arm64.msi")
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Some("macos-aarch64.dmg")
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        Some("macos-x86_64.dmg")
    } else {
        None
    }
}
fn parse_release(body: &str, current: &str, suffix: &str) -> Result<Option<Release>, String> {
    let json: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    if json.get("draft").and_then(Value::as_bool) != Some(false) || json.get("prerelease").and_then(Value::as_bool) != Some(false) {
        return Ok(None);
    }
    let tag = json.get("tag_name").and_then(Value::as_str).filter(|s| s.len() <= 80).ok_or("Invalid release version")?;
    let version = Version::parse(tag.strip_prefix('v').unwrap_or(tag)).map_err(|e| e.to_string())?;
    let installed = Version::parse(current).map_err(|e| e.to_string())?;
    if !version.pre.is_empty() || version <= installed {
        return Ok(None);
    }
    let url = format!("{REPO}/releases/tag/{tag}");
    if json.get("html_url").and_then(Value::as_str) != Some(url.as_str()) {
        return Err("Unexpected release page URL".into());
    }
    let name = format!("tibetan-design-{version}-{suffix}");
    let asset_url = format!("{REPO}/releases/download/{tag}/{name}");
    let compatible = json.get("assets").and_then(Value::as_array).is_some_and(|assets| {
        assets.iter().any(|a| {
            a.get("name").and_then(Value::as_str) == Some(name.as_str()) && a.get("browser_download_url").and_then(Value::as_str) == Some(asset_url.as_str())
        })
    });
    Ok(compatible.then(|| Release { version: version.to_string(), url }))
}
fn latest() -> Result<Option<Release>, String> {
    let Some(suffix) = platform_suffix() else { return Ok(None) };
    let config = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(12))).build();
    let agent: ureq::Agent = config.into();
    let mut response =
        agent.get(API).header("User-Agent", "TibetanDesign-update-check").header("Accept", "application/vnd.github+json").call().map_err(|e| e.to_string())?;
    let body = response.body_mut().with_config().limit(64 * 1024).read_to_string().map_err(|e| e.to_string())?;
    parse_release(&body, env!("CARGO_PKG_VERSION"), suffix)
}
pub fn check(ctx: &egui::Context) -> Result<Pending, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    std::thread::Builder::new()
        .name("tibetan-update-check".into())
        .spawn(move || {
            let result = latest();
            let _ = tx.send(result);
            ctx.request_repaint();
        })
        .map_err(|e| e.to_string())?;
    Ok(rx)
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn release(tag: &str, suffix: &str) -> Value {
        let version = tag.strip_prefix('v').unwrap_or(tag);
        let name = format!("tibetan-design-{version}-{suffix}");
        json!({"draft":false,"prerelease":false,"tag_name":tag,"html_url":format!("{REPO}/releases/tag/{tag}"),
            "assets":[{"name":name,"browser_download_url":format!("{REPO}/releases/download/{tag}/{name}")}]})
    }
    #[test]
    fn update_versions_compare_numerically_and_require_platform_assets() {
        let body = release("v0.10.0", "windows-x64.msi").to_string();
        assert_eq!(parse_release(&body, "0.9.9", "windows-x64.msi").unwrap().unwrap().version, "0.10.0");
        assert!(parse_release(&body, "0.10.0", "windows-x64.msi").unwrap().is_none());
        assert!(parse_release(&body, "1.0.0", "windows-x64.msi").unwrap().is_none());
        assert!(parse_release(&body, "0.9.0", "macos-aarch64.dmg").unwrap().is_none());
    }
    #[test]
    fn drafts_prereleases_invalid_json_and_other_hosts_are_rejected() {
        let mut body = release("v1.0.0", "macos-x86_64.dmg");
        body["draft"] = json!(true);
        assert!(parse_release(&body.to_string(), "0.8.3", "macos-x86_64.dmg").unwrap().is_none());
        body["draft"] = json!(false);
        body["prerelease"] = json!(true);
        assert!(parse_release(&body.to_string(), "0.8.3", "macos-x86_64.dmg").unwrap().is_none());
        body["prerelease"] = json!(false);
        body["html_url"] = json!("https://example.com/update");
        assert!(parse_release(&body.to_string(), "0.8.3", "macos-x86_64.dmg").is_err());
        assert!(parse_release("not json", "0.8.3", "windows-x64.msi").is_err());
        assert!(parse_release(&release("v1.0.0-beta.1", "windows-x64.msi").to_string(), "0.8.3", "windows-x64.msi").unwrap().is_none());
    }
}
