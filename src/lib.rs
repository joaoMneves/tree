use std::{fs, path::PathBuf};

pub fn show_dirs(path: PathBuf, show_files: bool, indent_level: u16, max_indent: Option<u16>) {
    let dir = fs::read_dir(path).unwrap();

    for entry in dir {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        let entry_path = entry.path();

        if !entry_path.is_dir() && show_files == false {
            continue;
        }

        println!("{:}/ {name}", make_tabs(indent_level));

        if !entry_path.is_dir() {
            continue;
        }

        match max_indent {
            None => {
                show_dirs(entry_path, show_files, indent_level + 1, max_indent);
            }
            Some(max) => {
                if indent_level + 1 <= max {
                    show_dirs(entry_path, show_files, indent_level + 1, max_indent);
                }
            }
        }
    }
}

fn make_tabs(indent_level: u16) -> String {
    let mut tabs = String::new();

    let mut i = 0;
    while i < indent_level {
        tabs.push_str("    ");

        i += 1;
    }
    tabs
}
