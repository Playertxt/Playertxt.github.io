//! Site-wide settings that every page needs: where the site is mounted and
//! which version of the stylesheet and script to ask the browser for.

use std::{fs, path::Path};

/// Settings shared by the server and the static exporter.
#[derive(Debug, Clone)]
pub struct Site {
    /// URL prefix the site lives under: `""` at a domain root, or something
    /// like `"/porti"` on a project page. Never ends with a slash.
    base: String,
    /// Short fingerprint of the stylesheet and script, appended to their URLs
    /// so browsers fetch new copies after a change.
    version: String,
}

impl Site {
    pub fn new(base: &str, assets_dir: &Path) -> Self {
        Self {
            base: normalize_base(base),
            version: fingerprint(assets_dir),
        }
    }

    /// Link to a page. `path` starts with a slash: `"/"`, `"/work/lookout/"`.
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base, path)
    }

    /// Link to a file under `assets/` that never changes in place (images).
    pub fn asset(&self, path: &str) -> String {
        format!("{}/assets/{}", self.base, path)
    }

    /// Link to a file under `assets/` that does change (CSS, JS), with the
    /// fingerprint attached.
    pub fn versioned_asset(&self, path: &str) -> String {
        format!("{}/assets/{}?v={}", self.base, path, self.version)
    }
}

/// Turns `"porti"`, `"/porti/"` and `"/porti"` into `"/porti"`, and `"/"` or
/// `""` into `""`.
fn normalize_base(base: &str) -> String {
    let trimmed = base.trim().trim_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("/{trimmed}")
    }
}

/// FNV-1a hash of the files whose URLs carry the fingerprint.
fn fingerprint(assets_dir: &Path) -> String {
    const FILES: [&str; 2] = ["css/site.css", "js/site.js"];
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for file in FILES {
        // A missing file just leaves the hash unchanged; the page itself will
        // show the problem soon enough.
        for byte in fs::read(assets_dir.join(file)).unwrap_or_default() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    format!("{:08x}", hash as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_is_normalized() {
        assert_eq!(normalize_base(""), "");
        assert_eq!(normalize_base("/"), "");
        assert_eq!(normalize_base("porti"), "/porti");
        assert_eq!(normalize_base("/porti/"), "/porti");
    }

    #[test]
    fn urls_carry_the_base() {
        let site = Site {
            base: "/porti".into(),
            version: "abc".into(),
        };
        assert_eq!(site.url("/"), "/porti/");
        assert_eq!(site.asset("img/a.webp"), "/porti/assets/img/a.webp");
        assert_eq!(
            site.versioned_asset("css/site.css"),
            "/porti/assets/css/site.css?v=abc"
        );
    }
}
