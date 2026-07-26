// C. Wyatt Polasek + Zach Breene
// rUM Loading Module
// rUM - loading.rs

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Loads a UM (Universal Machine) program from a file.
///
/// Reads a file containing a UM program and converts its contents into a `Vec<u32>`, representing the program.
///
/// # Arguments
///
/// * `program_path` - A reference to the path where the UM program file is located.
///
/// # Returns
///
/// Returns a `Result` which is `Ok` with a vector of 32-bit words if loading succeeds, or an `Error` if it fails.
pub fn load_um_program<P: AsRef<Path>>(program_path: P) -> io::Result<Vec<u32>> {
    let mut file = File::open(program_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    //Convert the contents into a Vec<u32>. This assumes the file is big-endian.
    let mut program = Vec::new();
    for chunk in contents.chunks_exact(4) {
        let instruction = u32::from_be_bytes(chunk.try_into().unwrap());
        program.push(instruction);
    }

    if contents.len() % 4 != 0 {
        //Handle the error if the program file does not consist of a series of 32-bit words.
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Program file size is not a multiple of 4 bytes."));
    }

    Ok(program)
}

// Unit tests for the loading module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests loading of a valid UM program file.
    ///
    /// This test attempts to load a UM program from a known valid file and checks for successful loading.
    #[test]
    fn test_load_um_program_valid() {
        let test_file_path = "cat.um"; //Adjust path as necessary
        let program = load_um_program(test_file_path);
        assert!(program.is_ok(), "Failed to load UM program: {:?}", program.err().unwrap());
        assert!(!program.unwrap().is_empty(), "UM program is empty");
    }

    /// Tests loading of an invalid UM program file.
    ///
    /// This test attempts to load a UM program from an invalid file path to test error handling.
    #[test]
    fn test_load_um_program_invalid() {
        //Use an invalid file path to test error handling.
        let test_file_path = "path/to/invalid/um_program.um";
        let program = load_um_program(test_file_path);
        assert!(program.is_err());
    }
}