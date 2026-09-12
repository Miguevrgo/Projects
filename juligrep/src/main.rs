use regex::bytes::Regex;
use std::{
    ffi::OsStr,
    fs::DirEntry,
    io::{self},
    path::Path,
};

fn is_ignored_dir(name: &OsStr) -> bool {
    matches!(
        name.to_str(),
        Some("target" | ".git" | "node_modules" | ".cargo" | ".rustup")
    )
}

fn is_binary(bytes: &[u8]) -> bool {
    bytes[..bytes.len().min(1024)].contains(&b'\0')
}

fn visit_dirs<F: FnMut(&DirEntry, &Regex)>(
    root: &Path,
    mut callback: F,
    regex: &Regex,
) -> std::io::Result<()> {
    let mut stack = vec![root.to_path_buf()];

    while let Some(current_dir) = stack.pop() {
        let entries = match std::fs::read_dir(&current_dir) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("{}: {err}", current_dir.display());
                continue;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("Error reading entry: {err}");
                    continue;
                }
            };

            if entry.path().is_dir() && !is_ignored_dir(entry.path().as_os_str()) {
                stack.push(entry.path());
            } else if entry.path().is_file() {
                callback(&entry, regex);
            }
        }
    }

    Ok(())
}

fn grep(entry: &DirEntry, regex: &Regex) {
    let Ok(content) = std::fs::read(entry.path()) else {
        return;
    };

    if is_binary(&content) || !regex.is_match(&content) {
        return;
    }

    for (line, line_bytes) in content.split(|&b| b == b'\n').enumerate() {
        if regex.is_match(line_bytes) {
            let content = String::from_utf8_lossy(line_bytes);
            println!("\x1b[34m{}", entry.path().display());
            println!("\x1b[31m{line}: \x1b[0m{content}");
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        return Err(io::Error::other("Usage: <regex> [path]"));
    }

    let regex = Regex::new(args.get(1).unwrap()).unwrap();

    std::thread::scope(|s| {
        if let Ok(entries) = std::fs::read_dir(args.get(2).map(String::as_str).unwrap_or(".")) {
            for entry in entries.flatten() {
                if entry.path().is_dir() && !is_ignored_dir(entry.path().as_os_str()) {
                    let r = &regex;
                    s.spawn(move || {
                        let _ = visit_dirs(&entry.path(), &grep, r);
                    });
                } else if entry.path().is_file() {
                    grep(&entry, &regex);
                }
            }
        }
    });

    Ok(())
}
