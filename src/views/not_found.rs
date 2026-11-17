//! The page shown for an address that does not exist.

use maud::{html, Markup};

use super::{arrow, layout, Page};
use crate::site::Site;

pub fn not_found(site: &Site) -> Markup {
    let page = Page {
        title: Some("Page not found"),
        description: "This page does not exist.",
        palette: None,
    };

    layout(
        site,
        &page,
        html! {
            section class="lost" {
                div class="wrap" {
                    p class="lost__code" { "404" }
                    h1 class="lost__title" { "There is nothing at this address." }
                    p class="lost__body" {
                        "The page may have moved, or the link may be wrong."
                    }
                    a class="btn btn--solid" href=(site.url("/")) { "Go to the home page" (arrow()) }
                }
            }
        },
    )
}
