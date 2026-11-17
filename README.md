# Portfolio

Kabir Shinde's portfolio site, written in Rust. One program does two jobs: it runs
the site as a web server, and it exports the same pages as static files for hosts
such as GitHub Pages.

## Run it

You need a recent stable [Rust](https://rustup.rs) toolchain.

```bash
cargo run
```

Then open <http://127.0.0.1:4000>. Press Ctrl+C to stop.

```bash
cargo run -- serve --host 0.0.0.0 --port 8080   # listen on every address
cargo run -- --help                             # all options
```

The port can also be set with the `PORT` environment variable.

## Export a static site

```bash
cargo run --release -- export
```

This writes the whole site to `dist/`. Upload that folder to any static host.

If the site will live under a path, such as `https://<user>.github.io/<repo>/`,
pass that path as the base so links and assets resolve:

```bash
cargo run --release -- export --base /<repo>
```

The export empties its output folder first, but only if the folder is new, empty,
or was created by an earlier export. It refuses any other folder.

## Change the content

Everything the site says is in [`src/content.rs`](src/content.rs): the profile,
the projects, the principles and the toolbox. To add a project, add a `Project`
entry to `PROJECTS` and put its icon and screenshots under `assets/img/`. The
home page, the project page and the export pick it up automatically.

A project with a `cover` screenshot shows it as its main visual. A project
without one gets a poster drawn from its icon, name and two palette colours.

## Deploy

Every push to `main` runs [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml),
which tests the site, exports it and publishes it to GitHub Pages.

## Layout

| Path | What's there |
| --- | --- |
| `src/main.rs` | Command line: `serve` and `export` |
| `src/content.rs` | All text and data |
| `src/views/` | Page templates ([maud](https://maud.lambda.xyz)): shell, home, project, 404 |
| `src/server.rs` | The [axum](https://github.com/tokio-rs/axum) server: routes, compression, headers |
| `src/export.rs` | The static exporter |
| `src/site.rs` | Base path and asset fingerprinting |
| `assets/` | Stylesheet, script and images, served at `/assets` |

## Test

```bash
cargo test
```

The tests check the content, the command line, every route of the server, and a
full export.
