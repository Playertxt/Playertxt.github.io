//! A project's own page: what it is, what it does and how it is built.

use maud::{html, Markup};

use super::{arrow, arrow_out, layout, palette_style, poster, tags, Page};
use crate::{
    content::{next_project, Note, Project},
    site::Site,
};

pub fn project(site: &Site, project: &Project) -> Markup {
    let page = Page {
        title: Some(project.name),
        description: project.summary,
        palette: Some(&project.palette),
    };
    let next = next_project(project);

    layout(
        site,
        &page,
        html! {
            article class="project" {
                (header(site, project))
                (cover(site, project))
                (stats(project))
                (overview(project))
                (notes("Highlights", "What it does", project.highlights, "notes"))
                (notes("Under the hood", "How it is built", project.internals, "notes notes--plain"))
                @if !project.gallery.is_empty() {
                    (gallery(site, project))
                }
            }
            a class="next" href=(site.url(&next.path())) style=(palette_style(&next.palette)) {
                div class="wrap next__inner" {
                    span class="next__label" { "Next project" }
                    span class="next__name" { (next.name) (arrow()) }
                    span class="next__kind" { (next.kind) }
                }
            }
        },
    )
}

fn header(site: &Site, project: &Project) -> Markup {
    html! {
        header class="project-hero" {
            div class="wrap" {
                a class="back" href=(site.url("/#work")) { (arrow()) "All work" }
                p class="project-hero__kicker" {
                    img src=(site.asset(project.icon)) alt="" width="44" height="44";
                    (project.kind)
                }
                h1 class="project-hero__title" { (project.name) }
                p class="project-hero__tagline" { (project.tagline) }
                dl class="meta" {
                    div class="meta__item" {
                        dt { "Year" }
                        dd { (project.year) }
                    }
                    div class="meta__item" {
                        dt { "Version" }
                        dd { (project.version) }
                    }
                    div class="meta__item" {
                        dt { "Platforms" }
                        dd { (project.platforms.join(", ")) }
                    }
                    div class="meta__item meta__item--wide" {
                        dt { "Built with" }
                        dd { (tags(project.stack)) }
                    }
                }
                @if !project.links.is_empty() {
                    div class="project-hero__links" {
                        @for (index, link) in project.links.iter().enumerate() {
                            a class=(if index == 0 { "btn btn--solid" } else { "btn" })
                                href=(link.href) rel="noopener" {
                                (link.label) (arrow_out())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn cover(site: &Site, project: &Project) -> Markup {
    html! {
        div class="wrap" {
            figure class="project-cover" {
                (poster(site, project, true))
                @if let Some(cover) = &project.cover {
                    figcaption { (cover.caption) }
                }
            }
        }
    }
}

fn stats(project: &Project) -> Markup {
    html! {
        div class="wrap" {
            dl class="stats" data-reveal {
                @for stat in project.stats {
                    div class="stats__item" {
                        dt { (stat.value) }
                        dd { (stat.label) }
                    }
                }
            }
        }
    }
}

fn overview(project: &Project) -> Markup {
    html! {
        section class="split" {
            div class="wrap split__inner" data-reveal {
                h2 class="split__label" { "Overview" }
                div class="split__body prose" {
                    @for paragraph in project.overview {
                        p { (paragraph) }
                    }
                }
            }
        }
    }
}

/// A labelled grid of titled paragraphs.
fn notes(label: &str, title: &str, notes: &[Note], class: &str) -> Markup {
    html! {
        section class="split" {
            div class="wrap split__inner" {
                div class="split__head" data-reveal {
                    h2 class="split__label" { (label) }
                    p class="split__title" { (title) }
                }
                ol class=(class) {
                    @for (index, note) in notes.iter().enumerate() {
                        li class="note" data-reveal {
                            span class="note__num" { (format!("{:02}", index + 1)) }
                            h3 class="note__title" { (note.title) }
                            p class="note__body" { (note.body) }
                        }
                    }
                }
            }
        }
    }
}

fn gallery(site: &Site, project: &Project) -> Markup {
    html! {
        section class="gallery" {
            div class="wrap" {
                h2 class="split__label" data-reveal { "Gallery" }
                div class="gallery__grid" {
                    @for shot in project.gallery {
                        figure class="gallery__item" data-reveal {
                            a href=(site.asset(shot.src)) data-lightbox data-caption=(shot.caption) {
                                img src=(site.asset(shot.src)) alt=(shot.alt)
                                    width="1280" height="720" loading="lazy" decoding="async";
                            }
                            figcaption { (shot.caption) }
                        }
                    }
                }
            }
            dialog class="lightbox" data-lightbox-dialog aria-label="Screenshot" {
                form method="dialog" {
                    button class="lightbox__close" type="submit" aria-label="Close" { "Close" }
                }
                img alt="";
                p class="lightbox__caption" {}
            }
        }
    }
}
