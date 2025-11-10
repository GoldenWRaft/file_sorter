use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

struct FileMove {
    source: PathBuf,
    destination: PathBuf,
}

fn sort_files_in_a_folder(mypath: &Path) -> io::Result<()> {
    if !mypath.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The specified path is not a directory.",
        ));
    }

    let entries = fs::read_dir(mypath)?;

    let files: Vec<PathBuf> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_file() {
                    Some(e.path())
                } else {
                    None
                }
            })
        })
        .collect();

    let mut filetype_folder_dict: HashMap<String, PathBuf> = HashMap::new();
    let mut planned_moves: Vec<FileMove> = Vec::new();

    for file_path in &files {
        if let Some(file_name) = file_path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if let Some(file_type) = file_name_str.split('.').last() {
                    let dest_path = filetype_folder_dict
                        .entry(file_type.to_string())
                        .or_insert_with(|| mypath.join(format!("_{}_folder", file_type)))
                        .clone();

                    let mut count = 1;
                    let mut destination_file = dest_path.join(file_name);

                    while destination_file.exists() {
                        if let Some(file_stem) = file_path.file_stem() {
                            if let Some(file_stem_str) = file_stem.to_str() {
                                let new_file_name =
                                    format!("{} ({}).{}", file_stem_str, count, file_type);
                                destination_file = dest_path.join(new_file_name);
                                count += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    planned_moves.push(FileMove {
                        source: file_path.clone(),
                        destination: destination_file,
                    });
                }
            }
        }
    }

    if planned_moves.is_empty() {
        println!("No files to sort in the specified directory.");
        return Ok(());
    }

    println!("--- Preview of File Sorting ---");
    for mv in &planned_moves {
        println!("Move '{}' >>> '{}'", mv.source.display(), mv.destination.display());
    }
    println!("-----------------------------");
    print!("Proceed with sorting? (y/n): ");
    io::stdout().flush()?;

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;

    if confirmation.trim().to_lowercase() == "y" || confirmation.trim().to_lowercase() == "yes" {
        println!("Sorting files...");

        let mut folders_to_create: HashSet<PathBuf> = HashSet::new();
        for mv in &planned_moves {
            if let Some(parent) = mv.destination.parent() {
                folders_to_create.insert(parent.to_path_buf());
            }
        }
        for folder in folders_to_create {
            if !folder.exists() {
                fs::create_dir_all(&folder)?;
            }
        }

        for mv in planned_moves {
            fs::rename(&mv.source, &mv.destination)?;
            println!("Moved '{}' >>> '{}'", mv.source.display(), mv.destination.display());
        }

        println!("Sorting complete!");
    } else {
        println!("Operation cancelled by user.");
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = sort_files_in_a_folder(&args.path) {
        eprintln!("Error: {}", e);
    }
}
