use regex::Regex;
use std::{
    fs::DirEntry,
    io::{self},
    path::Path,
    str::FromStr,
};

fn is_ignored_dir(name: &str) -> bool {
    name.contains("target")
        || name.contains(".git")
        || name.contains("node_modules")
        || name.contains(".cargo")
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

            if entry.path().is_dir() && !is_ignored_dir(&entry.path().to_string_lossy()) {
                stack.push(entry.path());
            } else if entry.path().is_file() {
                callback(&entry, regex);
            }
        }
    }

    Ok(())
}

fn grep(entry: &DirEntry, regex: &Regex) {
    let Ok(content) = std::fs::read_to_string(entry.path()) else {
        return;
    };

    for (line, content) in content.split('\n').enumerate() {
        if regex.is_match(content) {
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

    let regex = Regex::from_str(args.get(1).unwrap()).unwrap();

    std::thread::scope(|s| {
        if let Ok(entries) = std::fs::read_dir(args.get(2).map(String::as_str).unwrap_or(".")) {
            for entry in entries.flatten() {
                if entry.path().is_dir() && !is_ignored_dir(&entry.path().to_string_lossy()) {
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
