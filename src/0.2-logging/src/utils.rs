//! Contains various utilities and wrappers.
//!
//! Intended to simplify the code without introducing external dependencies.
//! May use slightly more advanced Rust concepts. If you're primarily trying to
//! learn about git, it's not necessary to attempt to read and understand these.

use std::fs;
use std::io;
use std::path::Path;

/// Recursively copies the contents, including subdirectories, of `from` into
/// `to`. Ignores any paths that match those contained in the `ignore` array.
pub fn copy_dir_deep(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    ignore: &[impl AsRef<Path>],
) -> Result<(), io::Error> {
    let dir_entries = fs::read_dir(&from)?;

    // Ensure the path we intend to transfer to exists.
    fs::create_dir_all(&to)?;

    for dir_entry_result in dir_entries {
        let dir_entry = dir_entry_result?;

        let entry_name = dir_entry.file_name();

        // Remember that ignore is not an array of Paths, it's an array of types
        // that implement AsRef<Path>, so using the contains method directly
        // here doesn't work.
        if ignore
            .iter()
            .any(|ignore_path| ignore_path.as_ref() == entry_name)
        {
            continue;
        }

        let final_destination = to.as_ref().join(entry_name);

        // We only check for files, and assume that anything that's not a file
        // is a directory. For our purposes, symlinks don't exist, because they
        // introduce a ton of complexity that isn't really relevant.
        if dir_entry.file_type()?.is_file() {
            fs::copy(dir_entry.path(), final_destination)?;
        } else {
            copy_dir_deep(dir_entry.path(), final_destination, ignore)?;
        }
    }

    Ok(())
}
