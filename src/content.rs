//! Everything the site says. Pages are built from these values, so adding a
//! project or changing a sentence happens here and nowhere else.

/// The person the portfolio belongs to.
pub struct Profile {
    pub name: &'static str,
    pub email: &'static str,
    pub github_handle: &'static str,
    pub github_url: &'static str,
    /// One sentence for search engines and link previews.
    pub description: &'static str,
}

pub const PROFILE: Profile = Profile {
    name: "Kabir Shinde",
    email: "kabirshinde90@gmail.com",
    github_handle: "Playertxt",
    github_url: "https://github.com/Playertxt",
    description: "Kabir Shinde builds complete software for Windows, Linux and Android: \
                  a GPU-accelerated remote desktop and system tools that show their work.",
};

/// A number with the thing it counts, shown large.
pub struct Stat {
    pub value: &'static str,
    pub label: &'static str,
}

/// A titled paragraph. Used for feature highlights, engineering notes and
/// the principles on the home page.
pub struct Note {
    pub title: &'static str,
    pub body: &'static str,
}

/// A screenshot, stored under `assets/img/`.
pub struct Shot {
    /// Path below `assets/`.
    pub src: &'static str,
    pub alt: &'static str,
    pub caption: &'static str,
}

pub struct Link {
    pub label: &'static str,
    pub href: &'static str,
}

/// The two colours that give a project its own look.
pub struct Palette {
    pub accent: &'static str,
    pub accent_2: &'static str,
}

pub struct Project {
    /// URL segment: `/work/<slug>/`.
    pub slug: &'static str,
    pub name: &'static str,
    /// What kind of thing it is, in two or three words.
    pub kind: &'static str,
    pub tagline: &'static str,
    /// Short description for the home page.
    pub summary: &'static str,
    /// Longer description for the project page, one paragraph per entry.
    pub overview: &'static [&'static str],
    pub year: &'static str,
    pub version: &'static str,
    pub platforms: &'static [&'static str],
    pub stack: &'static [&'static str],
    pub palette: Palette,
    /// App icon, as a path below `assets/`.
    pub icon: &'static str,
    /// Main screenshot. Projects without one get a drawn poster instead.
    pub cover: Option<Shot>,
    pub stats: &'static [Stat],
    pub highlights: &'static [Note],
    pub internals: &'static [Note],
    pub gallery: &'static [Shot],
    pub links: &'static [Link],
}

impl Project {
    pub fn path(&self) -> String {
        format!("/work/{}/", self.slug)
    }
}

pub fn find_project(slug: &str) -> Option<&'static Project> {
    PROJECTS.iter().find(|project| project.slug == slug)
}

/// The project shown after `current` on its page, wrapping around at the end.
pub fn next_project(current: &Project) -> &'static Project {
    let index = PROJECTS
        .iter()
        .position(|project| project.slug == current.slug)
        .unwrap_or(0);
    &PROJECTS[(index + 1) % PROJECTS.len()]
}

pub const PROJECTS: &[Project] = &[
    Project {
        slug: "kaddudesk",
        name: "KadduDesk",
        kind: "Remote desktop",
        tagline: "Any device controls any device, from anywhere in the world.",
        summary: "A remote desktop for Windows, Android and Linux that streams through the \
                  GPU's hardware encoder and draws every frame the moment it lands.",
        overview: &[
            "KadduDesk is a remote desktop for Windows, Android and Linux. There is one app per \
             system with the same screens everywhere, and every copy can both control other \
             devices and share itself. Kaddu means pumpkin, hence the orange.",
            "It is built for speed. The shared screen goes through the GPU's hardware video \
             encoder, or the phone's, and is decoded in hardware on the other side. Each frame \
             is drawn the moment it arrives.",
            "Devices on the same network find each other on their own. For everything else, the \
             Anywhere tab connects through home routers, carrier-grade NAT, mobile data and \
             hotel Wi-Fi without any port forwarding.",
        ],
        year: "2026",
        version: "2.0.0",
        platforms: &["Windows", "Android", "Linux"],
        stack: &["Go", "Electron", "Java", "FFmpeg", "WebCodecs", "Tailscale"],
        palette: Palette {
            accent: "#ff8a2b",
            accent_2: "#f0503a",
        },
        icon: "img/icons/kaddudesk.webp",
        cover: None,
        stats: &[
            Stat { value: "3", label: "operating systems, one app on each" },
            Stat { value: "0", label: "ports to forward" },
            Stat { value: "H.264", label: "encoded and decoded in hardware" },
            Stat { value: "2FA", label: "required on hub accounts by default" },
        ],
        highlights: &[
            Note {
                title: "Hardware video on both ends",
                body: "Screens are encoded with NVENC, AMF, Quick Sync, VAAPI or the phone's \
                       MediaCodec, and decoded in hardware on the viewer.",
            },
            Note {
                title: "Built to stay in real time",
                body: "H.264 with no B-frames and no lookahead. There is no jitter buffer, and a \
                       slow viewer resyncs on a fresh keyframe instead of building up lag.",
            },
            Note {
                title: "Connect from anywhere",
                body: "Anywhere works through any router, CGNAT, mobile data or hotel Wi-Fi with \
                       no port forwarding. You sign in once per device.",
            },
            Note {
                title: "Android on both sides",
                body: "A phone can control a PC, and a PC can control a phone: tap, swipe, type \
                       and press Back, Home and Recents from another device.",
            },
            Note {
                title: "A hub with real accounts",
                body: "One PC can be the hub for all your devices, with Argon2id password \
                       hashing, two-factor sign-in, recovery codes, roles and a full audit log.",
            },
            Note {
                title: "Workspaces",
                body: "Group devices and watch all of them live in a grid. Click any screen to \
                       control it at full quality while the others stay live on the side.",
            },
        ],
        internals: &[
            Note {
                title: "One core, every system",
                body: "The same Go core runs everywhere. On Windows it is both the app's core \
                       and the elevated sharing host; on Linux and Android the app runs sharing \
                       inside itself.",
            },
            Note {
                title: "Capture and input per platform",
                body: "Desktop duplication on Windows, X11 or KMS capture on Linux and \
                       MediaProjection on Android. Input goes through SendInput, uinput and an \
                       accessibility service.",
            },
            Note {
                title: "Identity is pinned",
                body: "Every connection uses TLS. Each device has its own certificate, which the \
                       app pins on first connect and warns loudly about if it ever changes. \
                       Anywhere adds WireGuard end to end.",
            },
            Note {
                title: "No web surface",
                body: "Devices refuse browser access. The app's interface is served only on the \
                       device's loopback address and locked with a per-launch secret.",
            },
        ],
        gallery: &[],
        links: &[Link {
            label: "Source on GitHub",
            href: "https://github.com/Playertxt/kaddudesk",
        }],
    },
    Project {
        slug: "jigglypuff",
        name: "Jigglypuff",
        kind: "System cleaner",
        tagline: "A cleaner that shows its plan before it touches anything.",
        summary: "A careful, minimal cleaner for Windows and Android. Scanning never changes \
                  anything, and deleting runs one confirmed plan, once.",
        overview: &[
            "Jigglypuff is a cleaner for Windows 10 and 11 and for Android phones. On Windows \
             it is a single portable file that installs nothing. It clears junk and caches, \
             uninstalls apps completely, and finds leftovers, large files and duplicates.",
            "Cleaners are easy to write and hard to trust, so the whole design is about not \
             breaking things. Scanning never changes anything. Deleting starts with a dialog \
             that lists exactly what will be removed and where it goes, and the engine acts \
             on that list alone.",
            "It is also deeply customisable. Ten themes, eight shapes and \
             nine button styles can be mixed freely, and the result can be copied as a code \
             that works in both the Windows and the Android app.",
        ],
        year: "2026",
        version: "Windows 1.2.0 · Android 1.1.0",
        platforms: &["Windows", "Android"],
        stack: &["Go", "TypeScript", "Kotlin", "Jetpack Compose"],
        palette: Palette {
            accent: "#ff8fc3",
            accent_2: "#d63d7e",
        },
        icon: "img/icons/jigglypuff.svg",
        cover: None,
        stats: &[
            Stat { value: "1", label: "portable file on Windows, nothing installed" },
            Stat { value: "10", label: "themes, shared by both apps" },
            Stat { value: "SHA-256", label: "duplicates matched by content" },
            Stat { value: "7 days", label: "to restore from the trash on Android" },
        ],
        highlights: &[
            Note {
                title: "A plan, then exactly that plan",
                body: "The engine builds a deletion plan, shows it to you, and runs only that \
                       plan, once. It checks every item again just before deleting it.",
            },
            Note {
                title: "A guard that says no",
                body: "System folders, Microsoft's registry keys, GPU driver keys, Windows \
                       services and your protected folders are refused outright, whatever the \
                       interface asks for.",
            },
            Note {
                title: "Uninstalls that finish the job",
                body: "It runs the app's own uninstaller, waits for every helper process, then \
                       hunts down leftover folders, registry keys, services and scheduled tasks.",
            },
            Note {
                title: "Reversible by default",
                body: "Personal files go to the Recycle Bin. Registry keys, services and \
                       scheduled tasks are backed up before removal.",
            },
            Note {
                title: "Themes you can send as a code",
                body: "Copy code turns your whole theme into a string. Paste it on another PC \
                       or phone to get the same look.",
            },
            Note {
                title: "Android, within the rules",
                body: "A Kotlin and Jetpack Compose app that cleans caches, leftovers and \
                       duplicates, and moves photos and documents to a hidden trash instead of \
                       deleting them.",
            },
        ],
        internals: &[
            Note {
                title: "Elevated engine, ordinary window",
                body: "Only the Go engine runs with administrator rights. The interface opens \
                       in a chrome-less browser app window without them.",
            },
            Note {
                title: "Locked to one session",
                body: "The engine talks to its window over 127.0.0.1 with a random per-session \
                       key, so other programs and web pages cannot control it.",
            },
            Note {
                title: "Never follows links",
                body: "Symlinks, junctions and OneDrive placeholders are skipped, so it never \
                       deletes cloud files by accident.",
            },
            Note {
                title: "A simulated PC for testing",
                body: "A demo mode runs the real interface against a fake PC made of sparse \
                       files and an in-memory registry, on any operating system.",
            },
        ],
        gallery: &[],
        links: &[
            Link {
                label: "Source on GitHub",
                href: "https://github.com/Playertxt/Jigglypuff",
            },
            Link {
                label: "Releases",
                href: "https://github.com/Playertxt/Jigglypuff/releases",
            },
        ],
    },
    Project {
        slug: "dockyard",
        name: "Dockyard",
        kind: "Desktop taskbar",
        tagline: "A taskbar where your apps are piers and your windows are boats.",
        summary: "A playful taskbar for Windows and Linux. Pinned apps sit on piers, every open \
                  window is a little boat, and a sun crosses the sky as your clock.",
        overview: &[
            "Dockyard replaces the taskbar with a small harbour. Your pinned apps sit on piers, \
             each open window is a boat moored beside its app, and a sun crosses the sky from \
             sunrise to sunset as the clock, followed by a moon.",
            "Under the scenery it is a serious tool. Quick launch searches apps, windows and \
             commands. The harbor master controls every window and process on the machine. \
             Notifications are held at sea and delivered together, so they interrupt you at \
             most four times an hour.",
        ],
        year: "2026",
        version: "1.0.0",
        platforms: &["Windows", "Linux (X11)"],
        stack: &["JavaScript", "Electron"],
        palette: Palette {
            accent: "#ffc861",
            accent_2: "#7a5ea8",
        },
        icon: "img/icons/dockyard.webp",
        cover: None,
        stats: &[
            Stat { value: "5", label: "themes, each with matching sounds" },
            Stat { value: "4", label: "global shortcuts" },
            Stat { value: "15 min", label: "between notification deliveries" },
            Stat { value: "2", label: "desktops: Windows and Linux" },
        ],
        highlights: &[
            Note {
                title: "Piers and boats",
                body: "Click a boat to switch to its window, flick it up to bring the window \
                       forward, flick it down to minimise. A pier glows when its app has unread \
                       messages or a finished download.",
            },
            Note {
                title: "A sun for a clock",
                body: "The sun shows sunrise to sunset, then a moon takes over. Hover over it \
                       for the weather. Your next calendar event appears as a star.",
            },
            Note {
                title: "Harbor master",
                body: "Snap, pin, hide or close any window. See live CPU and memory for every \
                       process, end a task or its whole tree, pause it or change its priority.",
            },
            Note {
                title: "Message in a bottle",
                body: "Notifications collect at sea and wash ashore together every 15 minutes.",
            },
            Note {
                title: "Lighthouse and focus garden",
                body: "Each finished task adds a light to the lighthouse; fill it and you get \
                       fireworks. Plants grow while you stay focused on one app.",
            },
            Note {
                title: "Decks",
                body: "Drop one window card on another to group the windows you use together.",
            },
        ],
        internals: &[
            Note {
                title: "Shortcuts that negotiate",
                body: "If a global shortcut is already taken, Dockyard picks a free one and \
                       shows the real binding in Settings.",
            },
            Note {
                title: "Scriptable",
                body: "A local HTTP endpoint lets any script send a notification into the \
                       harbour as a bottle.",
            },
            Note {
                title: "Safe by default",
                body: "Core system processes are protected, and ending an app from quick launch \
                       takes a second press of Enter.",
            },
            Note {
                title: "Built on every tag",
                body: "Pushing a version tag builds the Windows installer and portable build \
                       and the Linux packages on GitHub Actions.",
            },
        ],
        gallery: &[],
        links: &[Link {
            label: "Source on GitHub",
            href: "https://github.com/Playertxt/dockyard",
        }],
    },
    Project {
        slug: "lookout",
        name: "Lookout",
        kind: "Task manager",
        tagline: "A task manager that tells you why your PC is slow.",
        summary: "A task manager for Windows and Linux that explains instead of listing: \
                  plain-English findings with a fix button, an hour of history, and rules that \
                  act for you.",
        overview: &[
            "The built-in task manager shows you a table and leaves the diagnosis to you. \
             Lookout does the diagnosis. It runs a check-up and reports findings in plain \
             English, such as an app that has held 30% CPU for 12 minutes or a drive that is \
             almost full, each with a button that fixes it.",
            "It also remembers. You can scroll back through the last hour, see which app spiked \
             CPU, memory, disk or network, and click any moment to see every app as it was \
             then. Rules handle the recurring cases without being asked.",
        ],
        year: "2026",
        version: "1.0.0",
        platforms: &["Windows", "Linux"],
        stack: &["JavaScript", "Electron", "C#"],
        palette: Palette {
            accent: "#2fd1b8",
            accent_2: "#f5c453",
        },
        icon: "img/icons/lookout.webp",
        cover: None,
        stats: &[
            Stat { value: "60 min", label: "of history to scroll back through" },
            Stat { value: "11", label: "things the built-in task manager can't do" },
            Stat { value: "1 click", label: "to give a game or work app priority" },
            Stat { value: "Undo", label: "for ending the wrong task" },
        ],
        highlights: &[
            Note {
                title: "Why is it slow?",
                body: "A plain-English check-up: which app is using 4 GB of memory, which one \
                       has been busy for 12 minutes, which drive is almost full. Each finding \
                       has a fix button.",
            },
            Note {
                title: "The last hour, remembered",
                body: "Scroll back to see which app spiked CPU, memory, disk or network, and \
                       click any moment to see every app at that time.",
            },
            Note {
                title: "Rules",
                body: "Tell it once: warn me when an app passes 2 GB, always run my game at \
                       high priority, lower Discord while OBS is open and put it back afterwards.",
            },
            Note {
                title: "Pause, resume, undo",
                body: "Freeze any app so it uses no CPU and carry on later. Ended the wrong \
                       thing? Start it again with the same program and command line.",
            },
            Note {
                title: "Who is using this file or port?",
                body: "Find what is locking a file or a USB drive, or what is sitting on port \
                       3000, and end it.",
            },
            Note {
                title: "Lost windows",
                body: "Finds windows that are off-screen, fully transparent, tiny or hidden, \
                       and brings them back to the middle of your screen.",
            },
        ],
        internals: &[
            Note {
                title: "Fails safe",
                body: "If Lookout closes or crashes, anything it paused is resumed and every \
                       priority it changed is put back. It will not end or pause core system \
                       processes.",
            },
            Note {
                title: "Leak detector",
                body: "Flags apps whose memory keeps growing and never comes back down.",
            },
            Note {
                title: "Native where it counts",
                body: "A small C# helper talks to Windows directly. On Linux, processes, \
                       connections and open files are read from /proc.",
            },
            Note {
                title: "A tested core",
                body: "The monitor, the rules engine, the diagnosis and the Linux parsing each \
                       have unit tests.",
            },
        ],
        gallery: &[],
        links: &[],
    },
];

/// The ideas the projects have in common, each tied to the work that shows it.
pub const PRINCIPLES: &[Note] = &[
    Note {
        title: "Fast is a feature",
        body: "KadduDesk draws each frame the moment it arrives and never buffers. I treat \
               waiting as a bug.",
    },
    Note {
        title: "Nothing destructive without a plan",
        body: "Jigglypuff lists exactly what it will delete, then deletes exactly that. Lookout \
               resumes everything it paused if it ever crashes. Software that touches your \
               machine should be able to explain itself.",
    },
    Note {
        title: "Finish the whole product",
        body: "Installers, portable builds, Android packages, in-app guides, automated tests \
               and release pipelines. A project is done when someone else can download it \
               and use it.",
    },
    Note {
        title: "Give it some character",
        body: "A taskbar can be a harbour. A cleaner can have ten themes. Tools people open \
               every day are allowed to be enjoyable.",
    },
];

/// A named group of skills for the toolbox section.
pub struct Toolbox {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const TOOLBOX: &[Toolbox] = &[
    Toolbox {
        title: "Languages",
        items: &["Go", "Rust", "JavaScript", "TypeScript", "Kotlin", "Java", "C#"],
    },
    Toolbox {
        title: "Platforms",
        items: &["Windows", "Linux", "Android"],
    },
    Toolbox {
        title: "Frameworks",
        items: &["Electron", "Jetpack Compose", "axum"],
    },
    Toolbox {
        title: "Systems",
        items: &[
            "Hardware video encoding",
            "WebCodecs",
            "TLS and certificate pinning",
            "Win32 and /proc",
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn slugs_are_unique_and_url_safe() {
        let mut seen = HashSet::new();
        for project in PROJECTS {
            assert!(seen.insert(project.slug), "duplicate slug {}", project.slug);
            assert!(
                project
                    .slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "slug {} is not URL-safe",
                project.slug
            );
        }
    }

    #[test]
    fn every_project_is_complete() {
        for project in PROJECTS {
            assert!(!project.overview.is_empty(), "{} has no overview", project.name);
            assert_eq!(project.stats.len(), 4, "{} needs four stats", project.name);
            assert!(project.highlights.len() >= 4, "{} needs highlights", project.name);
            assert!(!project.internals.is_empty(), "{} has no internals", project.name);
        }
    }

    #[test]
    fn next_project_wraps_around() {
        let last = PROJECTS.last().unwrap();
        assert_eq!(next_project(last).slug, PROJECTS[0].slug);
        assert_eq!(next_project(&PROJECTS[0]).slug, PROJECTS[1].slug);
    }
}
