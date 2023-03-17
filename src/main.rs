use clap::{arg, command, Parser};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{format, row, Table};
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use walkdir::WalkDir;

fn normalize_filename(filename: &str) -> String {
    filename
        .to_lowercase()
        .replace(" ", "_")
        .replace(|c: char| !c.is_ascii_alphanumeric() && c != '_', "")
}

fn replace_links(content: &str, link_pattern: &Regex, index: &HashMap<String, String>) -> String {
    link_pattern
        .replace_all(content, |caps: &regex::Captures| {
            let link = caps.name("link").unwrap().as_str();
            let key = link.to_lowercase().replace(" ", "_");
            let relative_path = index.get(&key).unwrap_or(&key);
            let alt_text = caps
                .name("alt_text")
                .map_or(key.clone(), |m| m.as_str().to_string());
            format!("[{}]({})", alt_text, relative_path)
        })
        .into_owned()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    folder: String,
}

fn main() {
    let args = Args::parse();

    let folder = &args.folder;
    let mut index = HashMap::new();

    // Collect files
    let files: Vec<_> = WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().unwrap_or_default() == "md")
        .collect();

    let file_count = files.len();
    let pb = ProgressBar::new(file_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {msg}").unwrap()
            .progress_chars("=> "),
    );

    let mut files_changed = 0;
    let mut links_changed = 0;

    // Step 1: Create the index and standardize filenames
    for entry in &files {
        pb.inc(1);
        pb.set_message("Creating index...");

        let entry_path = entry.path();
        let file_stem = entry_path.file_stem().unwrap().to_string_lossy();
        let normalized_filename = normalize_filename(&file_stem);

        let parent = entry_path.parent().unwrap();
        let new_path = parent.join(format!("{}.md", normalized_filename));

        if entry_path != new_path {
            fs::rename(entry_path, &new_path).unwrap();
        }

        let relative_path = new_path
            .strip_prefix(folder)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let key = relative_path
            .trim_end_matches(".md")
            .replace("/", "_")
            .to_lowercase()
            .replace(" ", "_");
        index.insert(key, relative_path);
    }

    // Step 2: Replace links in files
    let link_pattern = Regex::new(r"\[\[(?P<link>[^\|\]]+)(\|(?P<alt_text>[^\]]+))?\]\]").unwrap();

    let files: Vec<_> = WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().unwrap_or_default() == "md")
        .collect();

    for entry in files {
        pb.inc(1);
        pb.set_message("Replacing links...");

        let entry_path = entry.path();
        let mut content = String::new();
        let mut file = File::open(entry_path).unwrap();
        file.read_to_string(&mut content).unwrap();

        let updated_content = replace_links(&content, &link_pattern, &index);

        let changes = link_pattern.find_iter(&content).count();

        if changes > 0 {
            files_changed += 1;
            links_changed += changes;

            let mut file = File::create(entry.path()).unwrap();
            file.write_all(updated_content.as_bytes()).unwrap();
        }
    }

    pb.finish_with_message("Done");

    // Display the summary with a table and colors
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Metric", "Value"]);
    table.add_row(row!["Total files in the folder", file_count]);
    table.add_row(row!["Files with links changed", files_changed]);
    table.add_row(row!["Total number of link changes", links_changed]);

    println!("\n{}", "Summary:".bold().green());
    table.printstd();
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_normalize_filename() {
        assert_eq!(normalize_filename("File Name"), "file_name");
        assert_eq!(normalize_filename("File-Name"), "filename");
        assert_eq!(normalize_filename("File_Name"), "file_name");
        assert_eq!(normalize_filename("FileName"), "filename");
    }

    #[test]
    fn test_replace_links() {
        let mut index = HashMap::new();
        index.insert("file1".to_string(), "file1.md".to_string());
        index.insert("file2".to_string(), "file2.md".to_string());

        let link_pattern =
            Regex::new(r"\[\[(?P<link>[^\|\]]+)(\|(?P<alt_text>[^\]]+))?\]\]").unwrap();

        let input = "This is a link to [[File1]] and [[File2|custom text]].";
        let expected_output = "This is a link to [file1](file1.md) and [custom text](file2.md).";
        let output = replace_links(input, &link_pattern, &index);
        assert_eq!(output, expected_output);
    }
}
