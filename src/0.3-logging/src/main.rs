use std::error::Error;
use std::fs;
use std::process::Command;
use std::{env, io};

mod utils;

use sha2::{Digest, Sha256};

// Akin to the hidden .git directory, this is the directory where rat will store
// the history of the nest. The real .git directory is a bit more complicated
// than we're going to make it, but the concept is the same - everything that
// git stores is nothing magical, it's all just files stored in a directory.
const RAT_NEST: &str = ".rat";

// We're going to be using Box<dyn Error> to make some aspects of error handling
// less explicit for simplicity. It allows us to use any type that implements
// the Error trait as an error, including types known only at runtime thanks
// to "dyn". In an completely idiomatic application, you would likely create
// custom error types that you bubble up to the surface and then report in a
// user-friendly way.
fn main() -> Result<(), Box<dyn Error>> {
    let command_line_arguments: Vec<String> = env::args().collect();

    let subcommand = command_line_arguments
        .get(1)
        .ok_or_else(|| "No command provided.".to_string())?;

    let output = match subcommand.as_str() {
        "init" => {
            init()?;

            "Initialized new rat nest.".to_string()
        }
        "commit" => {
            // The user can specify the commit message either through the -m
            // option in the command itself or by opening their default editor
            // to edit a commit message.
            let message = if let Some("-m") = command_line_arguments.get(2).map(String::as_str) {
                // If they specified -m, we attempt to get the message from the
                // next argument.
                command_line_arguments
                    .get(3)
                    .ok_or_else(|| "No commit message provided.".to_string())?
                    .to_owned()
            } else {
                // Otherwise, we open their editor to a special file and use the
                // contents of that file as the commit message instead.
                let commit_file = format!("{RAT_NEST}/COMMIT_EDITMSG");

                // Empty the file first
                fs::write(&commit_file, "")?;

                // By convention, the default editor is usually in the $EDITOR
                // environment variable, but sometimes in $VISUAL.
                let editor = env::var("EDITOR")
                    .or_else(|_| env::var("VISUAL"))
                    .map_err(|_| "No editor set.".to_string())?;

                Command::new(editor)
                    // We pass in the special commit file to the editor
                    // through the Command interface.
                    .arg(&commit_file)
                    .status()?;

                fs::read_to_string(commit_file)
                    .map_err(|e| format!("Failed to read commit message: {e}"))?
            };

            if message.trim().is_empty() {
                Err("Cancelled commit.")?;
            }

            let hash = commit(&message)?;

            format!("Created commit number {hash}.")
        }
        "log" => log()?,
        _ => Err("Invalid subcommand.")?,
    };

    println!("{}", output);

    // We need to explicitly return an empty Ok here, since our return value is
    // a Result, not a plain "void".
    Ok(())
}

/// Initializes a new rat nest in the current directory.
fn init() -> Result<(), io::Error> {
    fs::create_dir(RAT_NEST)?;
    fs::write(format!("{RAT_NEST}/HEAD"), "")?;
    fs::create_dir(format!("{RAT_NEST}/commits"))?;
    fs::create_dir(format!("{RAT_NEST}/contents"))?;

    Ok(())
}

/// Commits the contents of the current directory to the nest.
fn commit(message: &str) -> Result<String, Box<dyn Error>> {
    let head_file = format!("{RAT_NEST}/HEAD");
    let working_dir = env::current_dir()?;

    let current_head = fs::read_to_string(&head_file)?;

    let metadata = format!("parent {current_head}\n\n{message}");

    // Create a Sha256 Hasher and use it to create a hash of the contents of
    // each of the files in the working directory, plus the metadata.
    let mut hasher = Sha256::new();
    hasher.update(&metadata);
    utils::hash_directory(&mut hasher, &working_dir, &[RAT_NEST])?;
    let new_commit_hash = utils::encode_hash(hasher.finalize());

    // Write the commit metadata, and only the metadata, to a file.
    fs::write(format!("{RAT_NEST}/commits/{new_commit_hash}"), metadata)?;

    // Create a new directory for the new commit inside the nest.
    let commit_dir = format!("{RAT_NEST}/contents/{new_commit_hash}");
    fs::create_dir(&commit_dir)?;

    // Copy the current working directory into the commit directory, ignoring
    // the nest itself.
    utils::copy_dir_deep(working_dir, &commit_dir, &[RAT_NEST])?;

    // Update the HEAD file with the new commit that we just created.
    fs::write(head_file, &new_commit_hash)?;

    Ok(new_commit_hash)
}

fn log() -> Result<String, Box<dyn Error>> {
    // First we obtain the current head pointer. We wrap it in an Option because
    // we're going to be digging into its parents and need a way to bail out
    // once we get to the root.
    let head_file = format!("{RAT_NEST}/HEAD");
    let mut current_head = Some(fs::read_to_string(&head_file)?);

    // We initialize the string we're eventually going to return.
    let mut logs = String::new();

    while let Some(head) = &current_head {
        // This is the header, which is simply the commit hash itself
        logs.push_str(&utils::terminal_format(
            &format!("commit {head}\n"),
            utils::TerminalFormatting {
                color: Some(utils::Color::Yellow),
                bold: true
            }
        ));

        // We retrieve the metadata from the commit file, not the contents
        let head_metadata = fs::read_to_string(format!("{RAT_NEST}/commits/{head}"))?;

        // We need to keep track of whether we're currently reading the
        // key/value metadata, or the commit message itself.
        let mut capturing_message = false;
        for metadata_line in head_metadata.lines() {
            // If we're currently in the message portion, we should indent the
            // message line, put it directly in our output, and move on to the
            // next one.
            if capturing_message {
                logs.push_str(&(" ".repeat(4) + metadata_line));
                continue;
            }

            // Our simple format is defined by key/value metadata separated
            // by a space. We split on it and match on the results.
            let (key, value) = metadata_line.split_once(' ').unwrap_or_default();

            match key {
                "parent" => {
                    // If this commit has a parent, that's the next commit we
                    // have to log, so set it as our current head. If not, we've
                    // reached the root.
                    current_head = if !value.trim().is_empty() {
                        Some(value.to_string())
                    } else {
                        None
                    }
                }
                _ => capturing_message = true,
            }
        }

        // We check this condition down here because we don't want these
        // separators to be printed after the last commit.
        if current_head.is_some() {
            logs.push_str("\n\n");
        }
    }

    Ok(logs)
}