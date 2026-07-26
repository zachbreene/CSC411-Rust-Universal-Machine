// C. Wyatt Polasek + Zach Breene
// rUM Memory Management Module
// rUM - memory.rs

use std::collections::HashMap;

/// Represents the memory model for the Universal Machine.
/// Manages segments of memory, each being a vector of 32-bit words.
pub struct Memory {
    segments: HashMap<usize, Vec<u32>>,
    free_segment_ids: Vec<usize>,
}

impl Memory {
    //Constructs a new Memory instance with an empty program segment (segment 0).
    pub fn new() -> Self {
        let mut segments = HashMap::new();
        segments.insert(0, Vec::new()); //Segment 0 is reserved for the program itself
        Memory {
            segments,
            free_segment_ids: Vec::new(),
        }
    }

    /// Initializes memory with the given program, placing it in segment 0.
    /// This segment is reserved for the UM program being executed.
    pub fn initialize_with_program(&mut self, program: Vec<u32>) {
        self.segments.insert(0, program);
    }

    /// Maps a new segment with the given size and returns its ID.
    /// Reuses IDs from the free segment pool if available.
    pub fn map_segment(&mut self, size: u32) -> usize {
        let id = self.free_segment_ids.pop().unwrap_or_else(|| self.segments.len());
        let new_segment = vec![0; size as usize];
        self.segments.insert(id, new_segment);
        id
    }

    /// Unmaps a segment with the given ID.
    /// Segment 0 cannot be unmapped and will result in an error.
    pub fn unmap_segment(&mut self, id: usize) -> Result<(), String> {
        if id == 0 {
            return Err("Cannot unmap segment 0".to_string());
        }
        if self.segments.remove(&id).is_some() {
            self.free_segment_ids.push(id);
            Ok(())
        } else {
            Err("Segment does not exist".to_string())
        }
    }

    /// Loads a value from a specified segment and offset.
    /// Returns an error if the segment ID or offset is invalid.
    pub fn load(&self, segment_id: usize, offset: usize) -> Result<u32, String> {
        self.segments.get(&segment_id)
            .and_then(|segment| segment.get(offset))
            .copied()
            .ok_or_else(|| "Invalid memory access".to_string())
    }

    /// Stores a value into a specified segment and offset.
    /// Returns an error if the segment ID or offset is invalid.
    pub fn store(&mut self, segment_id: usize, offset: usize, value: u32) -> Result<(), String> {
        if let Some(segment) = self.segments.get_mut(&segment_id) {
            if let Some(word) = segment.get_mut(offset) {
                *word = value;
                return Ok(());
            }
        }
        Err("Invalid memory access".to_string())
    }

    /// Duplicates a specified segment and replaces segment 0 with the duplicate.
    /// This method is used for loading a new program into the UM.
    pub fn load_program(&mut self, segment_id: usize) -> Result<(), String> {
        //Check if segment_id is valid and get the segment to duplicate
        let segment_to_duplicate = match self.segments.get(&segment_id) {
            Some(segment) => segment.clone(),
            None => return Err(format!("Attempt to load program from an unmapped segment: {}", segment_id)),
        };

        //Replace segment 0 with the duplicated segment
        self.segments.insert(0, segment_to_duplicate);        
        Ok(())
    }
}

/// Unit tests for the Memory module.
#[cfg(test)]
mod memory_tests {
    use super::Memory;

    /// Tests whether a new memory segment is correctly mapped and has the right size.
    ///
    /// This test creates a new memory segment of a specified size and verifies
    /// that it is correctly mapped in the memory and has the expected size.
    #[test]
    fn test_memory_segment_mapping() {
        let mut memory = Memory::new();
        let segment_id = memory.map_segment(10);
        assert!(memory.segments.contains_key(&segment_id));
        assert_eq!(memory.segments[&segment_id].len(), 10);
    }

    /// Ensures that a memory segment can be unmapped and is no longer accessible.
    ///
    /// This test maps a new memory segment and then unmaps it, verifying
    /// that the segment is no longer present in the memory after unmapping.
    #[test]
    fn test_memory_segment_unmapping() {
        let mut memory = Memory::new();
        let segment_id = memory.map_segment(10);
        assert!(memory.unmap_segment(segment_id).is_ok());
        assert!(!memory.segments.contains_key(&segment_id));
    }

    /// Verifies that values can be stored and later retrieved from a memory segment.
    ///
    /// This test checks the store and load functionality of the memory,
    /// ensuring that values stored in a memory segment can be correctly retrieved.
    #[test]
    fn test_memory_store_and_load() {
        let mut memory = Memory::new();
        let segment_id = memory.map_segment(10);
        let value = 123;

        assert!(memory.store(segment_id, 0, value).is_ok());
        match memory.load(segment_id, 0) {
            Ok(loaded_value) => assert_eq!(loaded_value, value),
            Err(e) => panic!("Failed to load value: {}", e),
        }
    }

    /// Tests whether the memory is correctly initialized with a given program in segment 0.
    ///
    /// This test initializes the memory with a program and verifies
    /// that segment 0 is correctly set with the given program data.
    #[test]
    fn test_memory_initialization_with_program() {
        let mut memory = Memory::new();
        let program = vec![1, 2, 3, 4, 5];
        memory.initialize_with_program(program.clone());

        assert_eq!(memory.segments[&0], program);
    }

    /// Confirms that a new program can be loaded into segment 0, replacing the existing program.
    ///
    /// This test initializes the memory with a program, then loads a new program into segment 0,
    /// and verifies that the original program is replaced by the new program.
    #[test]
    fn test_memory_load_program() {
        let mut memory = Memory::new();
        let program = vec![1, 2, 3, 4, 5];
        memory.initialize_with_program(program.clone());

        let segment_id = memory.map_segment(10);
        memory.segments.insert(segment_id, vec![6, 7, 8, 9, 10]);
        assert!(memory.load_program(segment_id).is_ok());

        assert_eq!(memory.segments[&0], vec![6, 7, 8, 9, 10]);
    }

    /// Covers cases of invalid memory access, such as storing/loading from an invalid offset or a non-existent segment.
    ///
    /// This test ensures that attempts to access memory outside of valid bounds or in non-existent segments
    /// are properly handled with errors.
    #[test]
    fn test_invalid_memory_access() {
        let mut memory = Memory::new();
        let segment_id = memory.map_segment(10);

        assert!(memory.store(segment_id, 10, 123).is_err());
        assert!(memory.load(segment_id, 10).is_err());
        assert!(memory.load(999, 0).is_err());
    }
}
