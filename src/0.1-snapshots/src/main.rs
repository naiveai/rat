use std::{env, io};
use std::error::Error;
use std::fs;

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
        },
        "commit" => {
            let number = commit()?;
            
            format!("Created commit number {number}.")
        }
        _ => Err("Invalid subcommand.")?
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
fn commit() -> Result<i32, Box<dyn Error>> {
    let head_file = format!("{RAT_NEST}/HEAD");

    // Read and parse the current HEAD file, containing a reference to the last
    // commit. The `parse` method on `String` has many possible outputs, so we
    // must clarify which one we need with an explicit type annotation.
    let head_string = fs::read_to_string(&head_file)?;
    let head_number: i32 = head_string.parse()?;

    let new_head_number = head_number + 1;

    // Create a new directory for our new commit inside the nest.
    let commit_dir = format!("{RAT_NEST}/commit-{new_head_number}");
    fs::create_dir(&commit_dir)?;

    // Copy the current working directory into the commit directory, ignoring
    // the nest itself.
    utils::copy_dir_deep(env::current_dir()?, &commit_dir, &[RAT_NEST])?;

    // Update the HEAD file with the new commit that we just created.
    fs::write(head_file, new_head_number.to_string())?;

    Ok(new_head_number)
}
