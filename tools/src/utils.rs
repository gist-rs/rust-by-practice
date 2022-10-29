use lazy_static::lazy_static;
use regex::Regex;

use anyhow::Result;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

lazy_static! {
    pub(crate) static ref NUM_BULLET_RE: Regex = Regex::new(r"^\d{1,2}\.").unwrap();
    pub(crate) static ref CODE_BEGIN_RE: Regex = Regex::new(r"^```rust").unwrap();
    pub(crate) static ref CODE_END_RE: Regex = Regex::new(r"^```\r?$").unwrap();
    pub(crate) static ref INSERTED_RS_RE: Regex =
        Regex::new(r"^(\{\{#playground\s)(\w+\.rs\s)(answer\}\})$").unwrap();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Eq)]
pub(crate) enum ParseExpect {
    Number,
    CodeBegin,
    CodeEnd,
}

pub(crate) fn write_file_rs(
    quiz_file_name: String,
    current_num_bullet: &String,
    sub_index: i32,
    rust_content: &String,
) -> Result<(), anyhow::Error> {
    if current_num_bullet.len() == 0 {
        // bail!("expect current_num_bullet from {quiz_file_name}")
    };

    write!(
        File::create(&format!(
            "{}_{current_num_bullet}_{sub_index}.rs",
            quiz_file_name.to_lowercase().to_owned()
        ))?,
        "{rust_content}"
    )?;
    Ok(())
}

pub(crate) fn write_file_md(
    file_path: &String,
    rust_content: &String,
) -> Result<(), anyhow::Error> {
    write!(
        File::create(file_path.to_lowercase().to_owned())?,
        "{rust_content}"
    )?;
    Ok(())
}

pub(crate) fn get_filtered_files(
    dir: &Path,
    pattern_re: &Regex,
) -> Result<Vec<PathBuf>, anyhow::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|path| path.is_file())
        .filter(|path| pattern_re.is_match(path.file_name().unwrap().to_str().unwrap())) // Filter out non-file
        .collect())
}

pub(crate) fn get_folders(path_string: &String) -> Result<Vec<String>, anyhow::Error> {
    Ok(fs::read_dir(&Path::new(path_string))?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|path| path.is_dir())
        .map(|e| e.file_name().unwrap().to_owned().into_string().unwrap())
        .collect::<Vec<_>>())
}

pub(crate) fn get_rs_files(path_string: &String) -> Result<Vec<String>, anyhow::Error> {
    let rs_file_re = Regex::new(r".rs$").unwrap();
    let rs_paths = get_filtered_files(&Path::new(path_string), &rs_file_re)?;
    Ok(rs_paths
        .iter()
        .map(|e| e.file_name().unwrap().to_owned().into_string().unwrap())
        .collect::<Vec<_>>())
}

// TODO: DRY
pub(crate) fn get_md_files(path_string: &String) -> Result<Vec<String>, anyhow::Error> {
    let rs_file_re = Regex::new(r".md$").unwrap();
    let rs_paths = get_filtered_files(&Path::new(path_string), &rs_file_re)?;
    Ok(rs_paths
        .iter()
        .map(|e| e.file_name().unwrap().to_owned().into_string().unwrap())
        .collect::<Vec<_>>())
}

pub(crate) fn is_path_exists(path_name: &String) -> bool {
    Path::new(path_name).exists()
}
