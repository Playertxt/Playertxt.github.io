//! The home page: introduction, the list of projects, approach and contact.

use maud::{html, Markup};

use super::{arrow, arrow_out, eyebrow, layout, palette_style, poster, tags, Page};
use crate::{
    content::{Project, PRINCIPLES, PROFILE, PROJECTS, TOOLBOX},
    site::Site,
};

pub fn home(site: &Site) -> Markup {
    let page = Page {
        title: None,
        description: PROFILE.description,
        palette: None,
    };

    layout(
        site,
        &page,
        html! {
            (hero(site))
            (work(site))
            (approach())
            (toolbox())
            (contact())
        },
    )
}

fn hero(site: &Site) -> Markup {
    // The most recently updated project gets the "now building" spot.
    let current = &PROJECTS[0];

    html! {
        section class="hero" id="top" data-glow {
            div class="hero__glow" aria-hidden="true" {}
            div class="wrap" {
                a class="hero__status" href=(site.url(&current.path())) data-reveal {
                    span class="hero__dot" aria-hidden="true" {}
                    "Now building " (current.name)
                    (arrow())
                }
                h1 class="hero__title" data-reveal {
                    "Software that’s fast, careful and a little bit "
                    em { "playful." }
                }
                div class="hero__foot" data-reveal {
                    p class="hero__lead" {
                        "I’m " (PROFILE.name) ". I design and build complete products for \
                         Windows, Linux and Android: a GPU-accelerated remote desktop and \
                         system tools that show their work."
                    }
                    div class="hero__actions" {
                        a class="btn btn--solid" href="#work" { "See the work" (arrow()) }
                        a class="btn" href=(PROFILE.github_url) rel="noopener" {
                            "GitHub" (arrow_out())
                        }
                    }
                }
                dl class="facts" data-reveal {
                    div class="facts__item" {
                        dt { (format!("{:02}", PROJECTS.len())) }
                        dd { "projects, each built from engine to installer" }
                    }
                    div class="facts__item" {
                        dt { "03" }
                        dd { "platforms: Windows, Linux and Android" }
                    }
                    div class="facts__item" {
                        dt { "07" }
                        dd { "languages, from Go and Rust to Kotlin and C#" }
                    }
                }
            }
        }
    }
}

fn work(site: &Site) -> Markup {
    html! {
        section class="section" id="work" {
            div class="wrap" {
                header class="section__head" data-reveal {
                    (eyebrow("01", "Selected work"))
                    h2 class="section__title" { "Four projects, built end to end." }
                }
                div class="work" {
                    @for (index, project) in PROJECTS.iter().enumerate() {
                        (work_card(site, project, index))
                    }
                }
            }
        }
    }
}

fn work_card(site: &Site, project: &Project, index: usize) -> Markup {
    let href = site.url(&project.path());
    let class = if index % 2 == 0 {
        "work-card"
    } else {
        "work-card work-card--flip"
    };

    html! {
        article class=(class) style=(palette_style(&project.palette)) data-reveal {
            a class="work-card__media" href=(href) tabindex="-1" aria-hidden="true" {
                (poster(site, project, index == 0))
            }
            div class="work-card__body" {
                p class="work-card__meta" {
                    span class="work-card__num" { (format!("{:02}", index + 1)) }
                    span { (project.kind) }
                    span { (project.platforms.join(" · ")) }
                }
                h3 class="work-card__title" {
                    a href=(href) { (project.name) }
                }
                p class="work-card__tagline" { (project.tagline) }
                p class="work-card__summary" { (project.summary) }
                (tags(project.stack))
                a class="link-arrow" href=(href) aria-label=(format!("Read about {}", project.name)) {
                    "Read the case study" (arrow())
                }
            }
        }
    }
}

fn approach() -> Markup {
    html! {
        section class="section" id="approach" {
            div class="wrap" {
                header class="section__head" data-reveal {
                    (eyebrow("02", "Approach"))
                    h2 class="section__title" { "What the work has in common." }
                }
                ol class="principles" {
                    @for (index, principle) in PRINCIPLES.iter().enumerate() {
                        li class="principle" data-reveal {
                            span class="principle__num" { (format!("{:02}", index + 1)) }
                            h3 class="principle__title" { (principle.title) }
                            p class="principle__body" { (principle.body) }
                        }
                    }
                }
            }
        }
    }
}

fn toolbox() -> Markup {
    html! {
        section class="section section--tight" id="toolbox" {
            div class="wrap" {
                header class="section__head" data-reveal {
                    (eyebrow("03", "Toolbox"))
                    h2 class="section__title" { "What I build with." }
                }
                div class="toolbox" data-reveal {
                    @for group in TOOLBOX {
                        div class="toolbox__group" {
                            h3 class="toolbox__title" { (group.title) }
                            ul class="toolbox__list" {
                                @for item in group.items {
                                    li { (item) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn contact() -> Markup {
    html! {
        section class="section contact" id="contact" {
            div class="wrap" {
                div data-reveal {
                    (eyebrow("04", "Contact"))
                    h2 class="contact__title" {
                        "Have something worth building? "
                        em { "Write to me." }
                    }
                    a class="contact__mail" href=(format!("mailto:{}", PROFILE.email)) {
                        (PROFILE.email)
                    }
                    div class="contact__links" {
                        a class="btn" href=(PROFILE.github_url) rel="noopener" {
                            "github.com/" (PROFILE.github_handle) (arrow_out())
                        }
                    }
                }
            }
        }
    }
}
