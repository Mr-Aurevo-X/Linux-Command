use serde::{Deserialize, Serialize};
use std::process::Command;

const PRODUCT_REPO: &str = "Mr-Aurevo-X/Linux-Command";
const LATEST_API: &str = "https://api.github.com/repos/Mr-Aurevo-X/Linux-Command/releases/latest";
pub const RELEASES_PAGE: &str = "https://github.com/Mr-Aurevo-X/Linux-Command/releases/latest";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestCheck {
    pub ok: bool,
    pub skipped: bool,
    pub newer: bool,
    pub local: String,
    pub remote: String,
    pub html_url: String,
    pub repo: String,
}

pub fn check_latest(local_version: &str, enabled: bool) -> Result<LatestCheck, String> {
    let local = normalize_version(local_version);
    if !enabled {
        return Ok(LatestCheck {
            ok: true,
            skipped: true,
            newer: false,
            local,
            remote: String::new(),
            html_url: RELEASES_PAGE.to_string(),
            repo: PRODUCT_REPO.to_string(),
        });
    }

    let body = fetch_latest_json()?;
    parse_latest_body(&body, &local)
}

fn parse_latest_body(body: &str, local: &str) -> Result<LatestCheck, String> {
    let local = normalize_version(local);
    if body.trim().is_empty() {
        return Ok(no_remote_release(local));
    }
    let value: serde_json::Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(err) => {
            if body.contains("404") || body.contains("Not Found") {
                return Ok(no_remote_release(local));
            }
            return Err(format!("github latest json: {err}"));
        }
    };
    let tag = value
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if tag.is_empty() {
        return Ok(no_remote_release(local));
    }
    let html = value
        .get("html_url")
        .and_then(|v| v.as_str())
        .filter(|url| url.starts_with("https://github.com/Mr-Aurevo-X/Linux-Command"))
        .unwrap_or(RELEASES_PAGE)
        .to_string();
    let remote = normalize_version(&tag);

    Ok(LatestCheck {
        ok: true,
        skipped: false,
        newer: is_remote_newer(&remote, &local),
        local,
        remote,
        html_url: html,
        repo: PRODUCT_REPO.to_string(),
    })
}

fn no_remote_release(local: String) -> LatestCheck {
    LatestCheck {
        ok: true,
        skipped: false,
        newer: false,
        local,
        remote: String::new(),
        html_url: RELEASES_PAGE.to_string(),
        repo: PRODUCT_REPO.to_string(),
    }
}

fn fetch_latest_json() -> Result<String, String> {
    let output = Command::new("curl")
        .args([
            "-fsSL",
            "-H",
            "Accept: application/vnd.github+json",
            "-H",
            "User-Agent: Linux-Command",
            LATEST_API,
        ])
        .output()
        .map_err(|err| format!("curl: {err}"))?;
    if !output.status.success() {
        return Ok(String::new());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn allowlisted_release_url(raw: &str) -> Result<String, String> {
    let url = raw.trim();
    let rest = url
        .strip_prefix("https://github.com/Mr-Aurevo-X/")
        .ok_or_else(|| "release url not allowlisted".to_string())?;
    if rest.is_empty() || rest.contains("..") || rest.contains('\\') || rest.contains(' ') {
        return Err("release url not allowlisted".into());
    }
    Ok(url.to_string())
}

fn normalize_version(raw: &str) -> String {
    raw.trim()
        .trim_start_matches('v')
        .trim_start_matches('V')
        .to_string()
}

fn is_remote_newer(remote: &str, local: &str) -> bool {
    let remote_parts = parse_version_parts(remote);
    let local_parts = parse_version_parts(local);
    for i in 0..3 {
        let r = remote_parts.get(i).copied().unwrap_or(0);
        let l = local_parts.get(i).copied().unwrap_or(0);
        if r > l {
            return true;
        }
        if r < l {
            return false;
        }
    }
    false
}

fn parse_version_parts(raw: &str) -> Vec<u32> {
    raw.split(|c| c == '.' || c == '-')
        .filter_map(|part| part.parse::<u32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::allowlisted_release_url;

    #[test]
    fn allows_repo_site_and_releases() {
        assert!(
            allowlisted_release_url("https://github.com/Mr-Aurevo-X/Hub-Systeme-Linux").is_ok()
        );
        assert!(allowlisted_release_url(
            "https://github.com/Mr-Aurevo-X/Linux-Command/releases/latest"
        )
        .is_ok());
        assert!(allowlisted_release_url("https://evil.example/foo").is_err());
        assert!(allowlisted_release_url("https://github.com/Mr-Aurevo-X/../etc").is_err());
    }
}
