use std::fs;
use std::{env, io};

mod utils;

// ANCHOR: nest_directory
const RAT_NEST: &str = ".rat";
// ANCHOR_END: nest_directory

// ANCHOR: main
fn main() -> Result<(), RatError> {
    // ANCHOR: main_subcommands
    let command_line_arguments: Vec<String> = env::args().collect();

    let subcommand = command_line_arguments
        .get(1)
        .ok_or(RatError::NoSubcommand)?;
    // ANCHOR_END: main_subcommands

    // ANCHOR: subcommand_match
    let output = match subcommand.as_str() {
        "init" => {
            init()?;

            "Initialized new rat nest.".to_string()
        }
        "commit" => {
            let number = commit()?;

            format!("Created commit number {number}.")
        }
        // An Err value followed by ? is effectively equivalent to an early
        // return, it simply more closely mirrors other error handling logic by
        // having a ?
        _ => Err(RatError::InvalidSubcommand)?,
    };
    // ANCHOR_END: subcommand_match

    println!("{}", output);

    // We need to explicitly return an empty Ok here, since our return value is
    // a Result, not a plain "void".
    Ok(())
}
// ANCHOR_END: main

#[derive(Debug)]
enum RatError {
    NoSubcommand,
    InvalidSubcommand,
    FileError(io::Error),
    CommitError(RatCommitError),
}

impl From<io::Error> for RatError {
    fn from(error: io::Error) -> Self {
        Self::FileError(error)
    }
}

impl From<RatCommitError> for RatError {
    fn from(error: RatCommitError) -> Self {
        Self::CommitError(error)
    }
}

// ANCHOR: init
/// Initializes a new rat nest in the current directory.
fn init() -> Result<(), io::Error> {
    fs::create_dir(RAT_NEST)?;
    fs::write(format!("{RAT_NEST}/HEAD"), "-1")?;

    Ok(())
}
// ANCHOR_END: init

/// Commits the contents of the current directory to the nest.
fn commit() -> Result<i32, RatCommitError> {
    // ANCHOR: commit_head_parse
    let head_file = format!("{RAT_NEST}/HEAD");

    // Read and parse the current HEAD file, containing a reference to the last
    // commit. The `parse` method on `String` has many possible outputs, so we
    // must clarify which one we need with an explicit type annotation.
    let head_string = fs::read_to_string(&head_file)?;
    let head_number: i32 = head_string
        .parse()
        .map_err(|_| RatCommitError::InvalidHead)?;

    let new_head_number = head_number + 1;
    // ANCHOR_END: commit_head_parse

    // ANCHOR: commit_creation
    // Create a new directory for our new commit inside the nest.
    let commit_dir = format!("{RAT_NEST}/commit-{new_head_number}");
    fs::create_dir(&commit_dir)?;

    // Copy the current working directory into the commit directory, ignoring
    // the nest itself.
    utils::copy_dir_deep(env::current_dir()?, &commit_dir, &[RAT_NEST])?;

    // Update the HEAD file with the new commit that we just created.
    fs::write(head_file, new_head_number.to_string())?;
    // ANCHOR_END: commit_creation

    Ok(new_head_number)
}

#[derive(Debug)]
enum RatCommitError {
    FileError(io::Error),
    InvalidHead,
}

impl From<io::Error> for RatCommitError {
    fn from(error: io::Error) -> Self {
        Self::FileError(error)
    }
}
