//! Writes the whole site to a folder of plain files, ready for any static
//! host such as GitHub Pages.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{content::PROJECTS, site::Site, views};

/// Left in every export folder so a later export knows the folder is ours
/// and safe to empty.
const MARKER: &str = ".porti-export";

/// What an export produced.
pub struct Report {
    pub pages: usize,
    pub assets: usize,
    pub out_dir: PathBuf,
}

/// Renders every page into `out_dir` and copies the assets beside them.
///
/// `out_dir` is emptied first, but only if it is missing, empty, or was
/// created by an earlier export. Any other folder is left alone and reported
/// as an error, so a mistyped path cannot delete someone's files.
pub fn export(site: &Site, assets_dir: &Path, out_dir: &Path) -> io::Result<Report> {
    prepare(out_dir)?;

    let mut pages = 0;
    let mut write_page = |relative: &str, html: String| -> io::Result<()> {
        let path = out_dir.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        pages += 1;
        fs::write(path, html)
    };

    write_page("index.html", views::home(site).into_string())?;
    write_page("404.html", views::not_found(site).into_string())?;
    for project in PROJECTS {
        write_page(
            &format!("work/{}/index.html", project.slug),
            views::project(site, project).into_string(),
        )?;
    }

    let assets = copy_dir(assets_dir, &out_dir.join("assets"))?;

    // Tells GitHub Pages to publish the files as they are.
    fs::write(out_dir.join(".nojekyll"), "")?;
    fs::write(out_dir.join(MARKER), "")?;

    Ok(Report {
        pages,
        assets,
        out_dir: out_dir.to_path_buf(),
    })
}

fn prepare(out_dir: &Path) -> io::Result<()> {
    if out_dir.exists() {
        let is_ours = out_dir.join(MARKER).exists();
        let is_empty = fs::read_dir(out_dir)?.next().is_none();
        if !is_ours && !is_empty {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "{} already has files in it that were not created by an export. \
                     Choose an empty folder with --out.",
                    out_dir.display()
                ),
            ));
        }
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)
}

/// Copies a folder and everything in it, returning the number of files.
fn copy_dir(from: &Path, to: &Path) -> io::Result<usize> {
    fs::create_dir_all(to)?;
    let mut count = 0;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            count += copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assets() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("assets")
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("porti-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn export_writes_every_page_under_the_base() {
        let out = scratch("pages");
        let site = Site::new("/porti", &assets());
        let report = export(&site, &assets(), &out).unwrap();

        assert_eq!(report.pages, PROJECTS.len() + 2);
        assert!(report.assets > 0);
        assert!(out.join("404.html").exists());
        assert!(out.join("assets/css/site.css").exists());
        for project in PROJECTS {
            assert!(out.join("work").join(project.slug).join("index.html").exists());
        }

        let home = fs::read_to_string(out.join("index.html")).unwrap();
        assert!(home.contains("/porti/assets/css/site.css?v="));
        assert!(home.contains("href=\"/porti/work/kaddudesk/\""));

        // A second export into the same folder replaces the first.
        export(&site, &assets(), &out).unwrap();
        fs::remove_dir_all(&out).unwrap();
    }

    #[test]
    fn export_refuses_a_folder_it_did_not_create() {
        let out = scratch("foreign");
        fs::create_dir_all(&out).unwrap();
        fs::write(out.join("keep.txt"), "important").unwrap();

        let site = Site::new("", &assets());
        let error = export(&site, &assets(), &out).err().unwrap();
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert!(out.join("keep.txt").exists());

        fs::remove_dir_all(&out).unwrap();
    }
}
