#[macro_use]
extern crate lazy_static;
use literal::{set, SetLiteral};
use notify::DebouncedEvent::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::env;
use std::process::Command;
use std::str;
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch(file: &String, _event: &String, program: &String, args: &[String]) -> notify::Result<()> {
    let trigger = || {
        let resp = Command::new(program)
            .args(args)
            .output()
            .expect("failed to execute process");
        if !resp.stdout.is_empty() {
            tracing::info!("\n{}", str::from_utf8(&resp.stdout).unwrap());
        }
        if !resp.stderr.is_empty() {
            tracing::error!("\n{}", str::from_utf8(&resp.stderr).unwrap());
        }
    };

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    watcher.watch(file, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                NoticeWrite(path) => {
                    if _event == "prechange" {
                        tracing::info!("notice change {:?}", path);
                        trigger();
                    }
                }
                NoticeRemove(path) => {
                    if _event == "predelete" {
                        tracing::info!("notice delete {:?}", path);
                        trigger();
                    }
                }
                Create(path) => {
                    if _event == "create" {
                        tracing::info!("{:?} created", path);
                        trigger();
                    }
                }
                Write(path) => {
                    if _event == "change" {
                        tracing::info!("{:?} changed", path);
                        trigger();
                    }
                }
                Chmod(path) => {
                    if _event == "chmod" {
                        tracing::info!("{:?} chmod", path);
                        trigger();
                    }
                }
                Remove(path) => {
                    if _event == "delete" {
                        tracing::info!("{:?} deleted", path);
                        trigger();
                    }
                }
                Rename(source, dest) => {
                    if _event == "remove" {
                        tracing::info!("rename {:?} to {:?}", source, dest);
                        trigger();
                    }
                }
                Rescan => {
                    if _event == "rescan" {
                        tracing::info!("rescan");
                        trigger();
                    }
                }
                Error(err, path) => {
                    if _event == "error" {
                        tracing::info!("{:?} -> {:?}", err, path);
                        trigger();
                    }
                }
            },
            Err(e) => tracing::error!("{:?}", e),
        }
    }
}

lazy_static! {
    static ref USAGE: &'static str = r#"
usage:
    on path event action
    event: prechange | predelete | create | change | chmod | delete | remove | rescan | error

eg:
    on main.rs change cat main.rs 
"#
    .trim();
    static ref ALLOWS: HashSet<String> = set! {
        "chmod",
        "error",
        "create",
        "change",
        "delete",
        "remove",
        "rescan",
        "prechange",
        "predelete",
    };
}

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("{}", USAGE.to_owned());
        return;
    }
    if !ALLOWS.contains(args.get(2).unwrap()) {
        println!("{}", USAGE.to_owned());
        return;
    }
    if let Err(e) = watch(
        args.get(1).unwrap(),
        args.get(2).unwrap(),
        &args.get(3).unwrap(),
        &args[4..],
    ) {
        tracing::error!("{:?}", e);
    }
}
