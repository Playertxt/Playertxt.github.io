//! HTML for every page, written with `maud` so the markup is checked by the
//! compiler. This module holds the page shell and the pieces pages share.

mod home;
mod not_found;
mod project;

pub use home::home;
pub use not_found::not_found;
pub use project::project;

use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{
    content::{Palette, Project, PROFILE},
    site::Site,
};

const FONTS: &str = "https://fonts.googleapis.com/css2?\
family=Bricolage+Grotesque:opsz,wght@12..96,300..800&\
family=Geist:wght@300..700&family=Geist+Mono:wght@400..600&display=swap";

/// Runs before the first paint so the page never flashes the wrong theme.
const THEME_BOOT: &str = "(function(){var d=document.documentElement,t;\
try{t=localStorage.getItem('theme')}catch(e){}\
if(t!=='light'&&t!=='dark'){t=matchMedia('(prefers-color-scheme: light)').matches?'light':'dark'}\
d.dataset.theme=t;d.classList.add('js')})();";

const ICON_ARROW: &str = r#"<svg class="icon" viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 8h10M9 4l4 4-4 4"/></svg>"#;
const ICON_OUT: &str = r#"<svg class="icon" viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M5 11l6-6M6 5h5v5"/></svg>"#;
const ICON_THEME: &str = r#"<svg class="icon" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true"><circle cx="8" cy="8" r="6.25" fill="none" stroke="currentColor" stroke-width="1.5"/><path d="M8 1.75a6.25 6.25 0 0 1 0 12.5z" fill="currentColor"/></svg>"#;

pub(crate) fn arrow() -> Markup {
    PreEscaped(ICON_ARROW.to_owned())
}

pub(crate) fn arrow_out() -> Markup {
    PreEscaped(ICON_OUT.to_owned())
}

/// What the shell needs to know about the page it wraps.
pub(crate) struct Page<'a> {
    /// Text for the browser tab. The site name is appended automatically.
    pub title: Option<&'a str>,
    pub description: &'a str,
    /// Colours for a page that belongs to one project.
    pub palette: Option<&'a Palette>,
}

/// Wraps page content in the document shell: head, navigation and footer.
pub(crate) fn layout(site: &Site, page: &Page, content: Markup) -> Markup {
    let title = match page.title {
        Some(title) => format!("{title} · {}", PROFILE.name),
        None => format!("{} · Software developer", PROFILE.name),
    };

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                meta name="description" content=(page.description);
                meta name="author" content=(PROFILE.name);
                meta name="color-scheme" content="dark light";
                meta property="og:type" content="website";
                meta property="og:title" content=(title);
                meta property="og:description" content=(page.description);
                meta name="twitter:card" content="summary";
                link rel="icon" href=(site.asset("favicon.svg")) type="image/svg+xml";
                script { (PreEscaped(THEME_BOOT)) }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link rel="stylesheet" href=(FONTS);
                link rel="stylesheet" href=(site.versioned_asset("css/site.css"));
                script src=(site.versioned_asset("js/site.js")) defer {}
            }
            body style=[page.palette.map(palette_style)] {
                a class="skip-link" href="#main" { "Skip to content" }
                (nav(site))
                main id="main" { (content) }
                (footer(site))
            }
        }
    }
}

fn nav(site: &Site) -> Markup {
    html! {
        header class="nav" data-nav {
            div class="wrap nav__inner" {
                a class="nav__brand" href=(site.url("/")) {
                    span class="nav__mark" aria-hidden="true" {}
                    (PROFILE.name)
                }
                nav class="nav__links" aria-label="Main" {
                    a href=(site.url("/#work")) { "Work" }
                    a href=(site.url("/#approach")) { "Approach" }
                    a href=(site.url("/#contact")) { "Contact" }
                }
                button class="nav__theme" type="button" data-theme-toggle
                    aria-label="Switch between light and dark" {
                    (PreEscaped(ICON_THEME))
                }
            }
        }
    }
}

fn footer(site: &Site) -> Markup {
    html! {
        footer class="footer" {
            div class="wrap footer__inner" {
                p { "© 2026 " (PROFILE.name) }
                p class="footer__colophon" {
                    "Written in Rust with axum and maud. No trackers, no cookies."
                }
                a class="footer__top" href=(site.url("/#top")) { "Back to top" }
            }
        }
    }
}

/// Inline custom properties that tint a block with a project's colours.
pub(crate) fn palette_style(palette: &Palette) -> String {
    format!(
        "--accent:{};--accent-2:{}",
        palette.accent, palette.accent_2
    )
}

/// The large visual for a project: its screenshot if it has one, otherwise a
/// poster drawn from its icon, name and colours.
pub(crate) fn poster(site: &Site, project: &Project, eager: bool) -> Markup {
    let loading = if eager { "eager" } else { "lazy" };
    html! {
        div class="poster" style=(format!("view-transition-name:poster-{}", project.slug)) {
            @if let Some(cover) = &project.cover {
                img class="poster__shot" src=(site.asset(cover.src)) alt=(cover.alt)
                    width="1280" height="720" loading=(loading) decoding="async";
            } @else {
                div class="poster__art" aria-hidden="true" {
                    span class="poster__word" { (project.name) }
                    span class="poster__rings" {}
                    img class="poster__icon" src=(site.asset(project.icon)) alt=""
                        width="320" height="320" loading=(loading) decoding="async";
                    span class="poster__label" { (project.kind) }
                    span class="poster__label poster__label--end" { "v" (short_version(project)) }
                }
            }
        }
    }
}

/// The first version number in a project's version text: `"1.2.0"` from
/// `"Windows 1.2.0 · Android 1.1.0"`.
fn short_version(project: &Project) -> &'static str {
    project
        .version
        .split_whitespace()
        .find(|word| word.starts_with(|c: char| c.is_ascii_digit()))
        .unwrap_or(project.version)
}

/// A row of small labels, used for stacks and platforms.
pub(crate) fn tags(items: &[&str]) -> Markup {
    html! {
        ul class="tags" {
            @for item in items {
                li { (item) }
            }
        }
    }
}

/// The small numbered label above a section title.
pub(crate) fn eyebrow(number: &str, label: &str) -> Markup {
    html! {
        p class="eyebrow" {
            span class="eyebrow__num" { (number) }
            (label)
        }
    }
}
