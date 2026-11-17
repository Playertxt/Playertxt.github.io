//! Kabir Shinde's portfolio.
//!
//! One program, two jobs:
//!
//! * `porti serve` runs the site as a web server.
//! * `porti export` writes the same pages to a folder of static files.

mod content;
mod export;
mod server;
mod site;
mod views;

use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    process::ExitCode,
    sync::Arc,
};

use site::Site;

const HELP: &str = "\
porti: the portfolio site

Usage:
  porti [serve] [--host <ip>] [--port <number>] [--assets <dir>]
  porti export  [--out <dir>] [--base <prefix>] [--assets <dir>]

Commands:
  serve     Run the web server (the default)
  export    Write the site to a folder of static files

Options:
  --host <ip>        Address to listen on            [default: 127.0.0.1]
  --port <number>    Port to listen on               [default: 4000, or $PORT]
  --out <dir>        Folder to export into           [default: dist]
  --base <prefix>    URL prefix the export lives at  [default: none]
                     Use --base /my-repo for a GitHub project page.
  --assets <dir>     Folder holding css, js and img  [default: ./assets]
  -h, --help         Show this help
";

enum Command {
    Serve { addr: SocketAddr },
    Export { out_dir: PathBuf, base: String },
    Help,
}

struct Args {
    command: Command,
    assets_dir: PathBuf,
}

fn main() -> ExitCode {
    let args = match parse_args(env::args().skip(1)) {
        Ok(args) => args,
        Err(message) => {
            eprintln!("error: {message}\n\n{HELP}");
            return ExitCode::from(2);
        }
    };

    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Args) -> std::io::Result<()> {
    match args.command {
        Command::Help => {
            print!("{HELP}");
            Ok(())
        }
        Command::Serve { addr } => {
            let site = Arc::new(Site::new("", &args.assets_dir));
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(server::serve(site, &args.assets_dir, addr))
        }
        Command::Export { out_dir, base } => {
            let site = Site::new(&base, &args.assets_dir);
            let report = export::export(&site, &args.assets_dir, &out_dir)?;
            println!(
                "Exported {} pages and {} asset files to {}",
                report.pages,
                report.assets,
                report.out_dir.display()
            );
            Ok(())
        }
    }
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Args, String> {
    let mut command_name: Option<String> = None;
    let mut host = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let mut port: u16 = match env::var("PORT") {
        Ok(value) => value
            .parse()
            .map_err(|_| format!("PORT is not a port number: {value}"))?,
        Err(_) => 4000,
    };
    let mut out_dir = PathBuf::from("dist");
    let mut base = String::new();
    let mut assets_dir: Option<PathBuf> = None;

    let mut args = args;
    while let Some(arg) = args.next() {
        let mut value = |name: &str| {
            args.next()
                .ok_or_else(|| format!("{name} needs a value after it"))
        };
        match arg.as_str() {
            "-h" | "--help" => {
                command_name = Some("help".into());
            }
            "--host" => {
                let text = value("--host")?;
                host = text
                    .parse()
                    .map_err(|_| format!("--host is not an IP address: {text}"))?;
            }
            "--port" => {
                let text = value("--port")?;
                port = text
                    .parse()
                    .map_err(|_| format!("--port is not a port number: {text}"))?;
            }
            "--out" => out_dir = PathBuf::from(value("--out")?),
            "--base" => base = value("--base")?,
            "--assets" => assets_dir = Some(PathBuf::from(value("--assets")?)),
            name if !name.starts_with('-') && command_name.is_none() => {
                command_name = Some(name.to_owned());
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let command = match command_name.as_deref() {
        None | Some("serve") => Command::Serve {
            addr: SocketAddr::new(host, port),
        },
        Some("export") => Command::Export { out_dir, base },
        Some("help") => Command::Help,
        Some(other) => return Err(format!("unknown command: {other}")),
    };

    Ok(Args {
        command,
        assets_dir: assets_dir.unwrap_or_else(default_assets_dir),
    })
}

/// `./assets` when the program is started from the project folder, otherwise
/// the folder the program was built from.
fn default_assets_dir() -> PathBuf {
    let local = PathBuf::from("assets");
    if local.is_dir() {
        local
    } else {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Args, String> {
        parse_args(args.iter().map(|arg| arg.to_string()))
    }

    #[test]
    fn serve_is_the_default_command() {
        let args = parse(&["--port", "8123"]).unwrap();
        match args.command {
            Command::Serve { addr } => {
                assert_eq!(addr.port(), 8123);
                assert!(addr.ip().is_loopback());
            }
            _ => panic!("expected serve"),
        }
    }

    #[test]
    fn export_takes_out_and_base() {
        let args = parse(&["export", "--out", "public", "--base", "/porti"]).unwrap();
        match args.command {
            Command::Export { out_dir, base } => {
                assert_eq!(out_dir, PathBuf::from("public"));
                assert_eq!(base, "/porti");
            }
            _ => panic!("expected export"),
        }
    }

    #[test]
    fn bad_input_is_reported() {
        assert!(parse(&["--port"]).is_err());
        assert!(parse(&["--port", "many"]).is_err());
        assert!(parse(&["--frobnicate"]).is_err());
        assert!(parse(&["deploy"]).is_err());
    }
}
