use std::error::Error;
use std::fs;
use std::process::Command;
use std::{env, io};

mod utils;

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

            let number = commit(&message)?;

            format!("Created commit number {number}.")
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
    fs::write(format!("{RAT_NEST}/HEAD"), "-1")?;

    Ok(())
}

/// Commits the contents of the current directory to the nest.
fn commit(message: &str) -> Result<i32, Box<dyn Error>> {
    let head_file = format!("{RAT_NEST}/HEAD");

    let head_string = fs::read_to_string(&head_file)?;
    let head_number: i32 = head_string.parse()?;

    let new_head_number = head_number + 1;

    // Create a new directory for the new commit inside the nest.
    let commit_dir = format!("{RAT_NEST}/commit-{new_head_number}");
    fs::create_dir(&commit_dir)?;

    // Copy the current working directory into the commit directory, ignoring
    // the nest itself.
    utils::copy_dir_deep(env::current_dir()?, &commit_dir, &[RAT_NEST])?;

    // Write a message into the .message file in that directory
    fs::write(format!("{commit_dir}/.message"), message)?;

    // Update the HEAD file with the new commit that we just created.
    fs::write(head_file, new_head_number.to_string())?;

    Ok(new_head_number)
}

fn log() -> Result<String, Box<dyn Error>> {
    // First we obtain the current head pointer. We wrap it in an Option because
    // we're going to be digging into its parents and need a way to bail out
    // once we get to the root.
    let head_file = format!("{RAT_NEST}/HEAD");
    let current_head: i32 = fs::read_to_string(head_file)?.parse()?;

    // We initialize the string we're eventually going to return.
    let mut logs = String::new();

    for commit_num in (0..=current_head).rev() {
        // This is the header, which is simply the commit number itself
        logs.push_str(&format!("commit {commit_num}\n\n"));

        // We retrieve the message string from the .message file
        let message = fs::read_to_string(format!("{RAT_NEST}/commit-{commit_num}/.message"))?;

        // for each line
        // prepend 4 spaces to that line
        let indented_message = message
            .lines()
            .map(|s| format!("    {s}\n"))
            .collect::<String>();

        // We append the message to our logs
        logs.push_str(&indented_message);

        // We check this condition down here because we don't want these
        // separators to be printed after the last commit.
        if commit_num != 0 {
            logs.push_str("\n\n");
        }
    }

    Ok(logs)
}
