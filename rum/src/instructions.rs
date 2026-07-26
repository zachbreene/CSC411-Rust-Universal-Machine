// C. Wyatt Polasek + Zach Breene
// rUM Instructions Decoder Module
// rUM - instructions.rs 

use bitpack::bitpack::getu;

/// Represents the different opcodes used in the rUM architecture.
/// 
/// The opcodes define the types of operations that can be performed by the Universal Machine.
#[derive(Debug, PartialEq)]
pub enum Opcode {
    ConditionalMove,
    SegmentedLoad,
    SegmentedStore,
    Addition,
    Multiplication,
    Division,
    NotAnd,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
}

/// Represents a decoded instruction with its opcode, registers, and optional value.
/// 
/// This struct encapsulates all the necessary components of an instruction in the rUM.
#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    registers: [Option<usize>; 3],
    value: Option<u32>,
}


impl Instruction {
    /// Creates a new `Instruction` struct.
    ///
    /// # Arguments
    /// 
    /// * `opcode` - The opcode of the instruction.
    /// * `registers` - The registers involved in the instruction, if any.
    /// * `value` - An optional immediate value used by the instruction.
    pub fn new(opcode: Opcode, registers: [Option<usize>; 3], value: Option<u32>) -> Self {
        Instruction { opcode, registers, value }
    }

    /// Returns a reference to the instruction's opcode.
    pub fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    /// Returns a reference to the instruction's registers.
    pub fn registers(&self) -> &[Option<usize>; 3] {
        &self.registers
    }

    /// Returns the instruction's optional value.
    pub fn value(&self) -> Option<u32> {
        self.value
    }
}

/// Decodes a 32-bit word into an `Instruction`.
///
/// This function extracts the opcode, registers, and optional immediate value from a 32-bit word.
///
/// # Arguments
/// 
/// * `instruction_word` - The 32-bit word representing the instruction.
///
/// # Returns
/// 
/// Returns an `Instruction` decoded from the input word.
pub fn decode_instruction(instruction_word: u32) -> Instruction {
    let opcode_val = getu(instruction_word as u64, 4, 28) as usize; // Use getu to extract opcode
    let registers;
    let value;

    match opcode_val {
        //Handle LoadValue specially
        13 => {
            let register = getu(instruction_word as u64, 3, 25) as usize;
            registers = [Some(register), None, None];
            value = Some(getu(instruction_word as u64, 25, 0) as u32);
        }
        _ => {
            let a = Some(getu(instruction_word as u64, 3, 6) as usize);
            let b = Some(getu(instruction_word as u64, 3, 3) as usize);
            let c = Some(getu(instruction_word as u64, 3, 0) as usize);
            registers = [a, b, c];
            value = None;
        }
    };

    let opcode = match opcode_val {
        0 => Opcode::ConditionalMove,
        1 => Opcode::SegmentedLoad,
        2 => Opcode::SegmentedStore,
        3 => Opcode::Addition,
        4 => Opcode::Multiplication,
        5 => Opcode::Division,
        6 => Opcode::NotAnd,
        7 => Opcode::Halt,
        8 => Opcode::MapSegment,
        9 => Opcode::UnmapSegment,
        10 => Opcode::Output,
        11 => Opcode::Input,
        12 => Opcode::LoadProgram,
        13 => Opcode::LoadValue,
        _ => panic!("Unknown opcode: {}", opcode_val),
    };
    
    Instruction::new(opcode, registers, value)
}




// Unit tests for the Instructions Decoder.
#[cfg(test)]
mod instruction_tests {
    use super::*;

    //Tests the decoding of the Conditional Move instruction.
    //
    #[test]
    fn test_decode_conditional_move() {
        let instruction_word = 0b0000_0000_0000_0000_0000_0000_0000_0000; //Example binary representation
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::ConditionalMove);
    }

    //Tests the decoding of the Segmented Load instruction.
    //
    #[test]
    fn test_decode_segmented_load() {
        let instruction_word = 0b0001_0000_0001_0010_0011_0100_0000_0000; //Example for Segmented Load
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::SegmentedLoad);
    }

    //Tests the decoding of the Segmented Store instruction.
    // 
    #[test]
    fn test_decode_segmented_store() {
        let instruction_word = 0b0010_0000_0100_0101_0110_0000_0000_0000; //Example for Segmented Store
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::SegmentedStore);
    }

    //Tests the decoding of the Addition instruction.
    //
    #[test]
    fn test_decode_addition() {
        let instruction_word = 0b0011_0111_1000_1001_1010_0000_0000_0000; //Example for Addition
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::Addition);
    }

    //Tests the decoding of the Map Segment instruction.
    //
    #[test]
    fn test_decode_map_segment() {
        let instruction_word = 0b1000_1010_1011_0000_0000_0000_0000_0000; //Example for Map Segment
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::MapSegment);
    }

    //Tests the decoding of the Unmap Segment instruction.
    //
    #[test]
    fn test_decode_unmap_segment() {
        let instruction_word = 0b1001_1100_0000_0000_0000_0000_0000_0000; //Example for Unmap Segment
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::UnmapSegment);
    }

    //Tests the decoding of the Output instruction.
    //
    #[test]
    fn test_decode_output() {
        let instruction_word = 0b1010_0000_0000_0000_0000_0001_0010_0011; //Example for Output
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::Output);
    }

    //Tests the decoding of the Input instruction.
    //
    #[test]
    fn test_decode_input() {
        let instruction_word = 0b1011_0000_0000_0000_0000_0100_0101_0110; //Example for Input
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::Input);
    }

    //Tests the decoding of the Load Program instruction.
    //
    #[test]
    fn test_decode_load_program() {
        let instruction_word = 0b1100_0111_1000_0000_0000_0000_0000_0000; //Example for Load Program
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::LoadProgram);
    }

    //Tests the decoding of the Load Value instruction.
    //
    #[test]
    fn test_decode_load_value() {
        //Note: Load Value has a different format
        let instruction_word = 0b1101_0000_0000_0000_0000_0000_0000_1111; //Example for Load Value
        let instruction = decode_instruction(instruction_word);
        assert_eq!(*instruction.opcode(), Opcode::LoadValue);
        assert_eq!(instruction.registers()[0], Some(0)); //Check register A
    }
}