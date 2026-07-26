// C. Wyatt Polasek + Zach Breene
// rUM Execution Module
// rUM - execution.rs

use crate::memory::Memory;
use crate::registers::Registers;
use crate::instructions::{Instruction, decode_instruction, Opcode};
use std::result;
use std::io::Read;


/// Custom error type for execution-related errors.
/// 
/// This enum defines the different kinds of errors that can occur during the execution of instructions.
#[derive(Debug)]
pub enum ExecutionError {
    MemoryError(String),
    InvalidInstruction(u32),
    Other(String),
}

/// A type alias for a result in the execution module.
/// 
/// This alias is used to simplify the handling of results from execution functions.
pub type ExecutionResult = result::Result<(), ExecutionError>;

/// Struct representing the execution state of the Universal Machine (UM).
/// 
/// This struct holds the memory and registers state, and is responsible for executing instructions.
pub struct UMExecution {
    memory: Memory,
    registers: Registers,
}

impl UMExecution {
    /// Creates a new `UMExecution` instance.
    /// 
    /// Initializes a new `UMExecution` with fresh memory and register states.
    pub fn new() -> Self {
        UMExecution {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    /// Initializes the `UMExecution` with the provided program.
    /// 
    /// Loads a program into memory and sets the program counter to 0.
    ///
    /// # Arguments
    ///
    /// * `program` - A vector of 32-bit words representing the program.
    pub fn initialize(&mut self, program: Vec<u32>) {
        self.memory.initialize_with_program(program);
        self.registers.set_program_counter(0);
    }

    /// Starts the execution cycle of the UM.
    /// 
    /// This method enters a loop, fetching, decoding, and executing instructions until a halt is encountered.
    pub fn run(&mut self) -> ExecutionResult {
        loop {
            //Fetch, decode, and execute instructions
            let pc = self.registers.get_program_counter();
            let instruction_word = self.fetch_instruction(pc)?;
            let instruction = self.decode_instruction(instruction_word);

            self.execute_instruction(instruction)?;
        }
    }

    /// Fetches the next instruction from memory.
    /// 
    /// Given a program counter, it fetches the corresponding instruction word from memory.
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter indicating where to fetch the instruction from.
    ///
    /// # Returns
    ///
    /// Returns the fetched instruction word or an error if fetching fails.
    fn fetch_instruction(&self, pc: u32) -> Result<u32, ExecutionError> {
        match self.memory.load(0, pc as usize) {
            Ok(instruction) => Ok(instruction),
            Err(e) => Err(ExecutionError::MemoryError(e)),
        }
    }

    /// Decodes a given instruction word into an `Instruction` struct.
    /// 
    /// This method decodes a raw 32-bit word into a structured `Instruction`.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The 32-bit word representing the instruction.
    ///
    /// # Returns
    ///
    /// Returns an `Instruction` decoded from the input word.
    fn decode_instruction(&self, instruction: u32) -> Instruction {
        decode_instruction(instruction)
    }

    /// Extracts and unwraps register indices from an instruction.
    /// 
    /// Given an array of optional register indices, this function extracts and unwraps them, ensuring they are valid.
    ///
    /// # Arguments
    ///
    /// * `registers` - An array of optional register indices.
    ///
    /// # Returns
    ///
    /// Returns a tuple of unwrapped register indices.
    fn extract_registers(registers: &[Option<usize>; 3]) -> Result<[usize; 3], ExecutionError> {
        let a = registers[0].ok_or(ExecutionError::InvalidInstruction(0))?;
        let b = registers[1].ok_or(ExecutionError::InvalidInstruction(0))?;
        let c = registers[2].ok_or(ExecutionError::InvalidInstruction(0))?;
        Ok([a, b, c])
    }

    /// Executes a given instruction.
    /// 
    /// This method performs the action defined by the instruction, modifying the machine's state accordingly.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The instruction to execute.
    ///
    /// # Returns
    ///
    /// Returns an `Ok` result if execution is successful, or an `ExecutionError` otherwise.
    fn execute_instruction(&mut self, instruction: Instruction) -> ExecutionResult {
        let registers = instruction.registers();
        match instruction.opcode() {
            
            Opcode::Halt => self.halt(),
            
            Opcode::ConditionalMove => {
                let [a, b, c] = Self::extract_registers(registers)?;
                self.registers.conditional_move(a, b, c);
            },
            
            Opcode::SegmentedLoad => {
                let [a, b, c] = Self::extract_registers(registers)?;
                let value = self.memory.load(self.registers.get(b) as usize, self.registers.get(c) as usize)
                    .map_err(ExecutionError::MemoryError)?;
                self.registers.set(a, value);
            },
            
            Opcode::SegmentedStore => {
                let [a, b, c] = Self::extract_registers(registers)?;
                self.memory.store(self.registers.get(a) as usize, self.registers.get(b) as usize, self.registers.get(c))
                    .map_err(ExecutionError::MemoryError)?;
            },
            
            Opcode::Addition => {
                let [a, b, c] = Self::extract_registers(registers)?;
                self.registers.addition(a, b, c);
            },
            
            Opcode::Multiplication => {
                let [a, b, c] = Self::extract_registers(registers)?;
                self.registers.multiplication(a, b, c);
            },
            
            Opcode::Division => {
                let [a, b, c] = Self::extract_registers(registers)?;
                self.registers.division(a, b, c);
            },
            
            Opcode::NotAnd => {
                let [a, b, c] = Self::extract_registers(registers)?;
                self.registers.not_and(a, b, c);
            },
            
            Opcode::MapSegment => {
                let c = registers[2].unwrap();
                let segment_id = self.memory.map_segment(self.registers.get(c));
                self.registers.set(registers[1].unwrap(), segment_id as u32);
            },
            
            Opcode::UnmapSegment => {
                let c = registers[2].unwrap();
                self.memory.unmap_segment(self.registers.get(c) as usize)
                    .map_err(ExecutionError::MemoryError)?;
            },
            Opcode::Output => self.output_instruction(registers[2].unwrap())?,
            
            Opcode::Input => self.input_instruction(registers[2].unwrap())?,

            Opcode::LoadProgram => {
                let b = registers[1].unwrap();
                let c = registers[2].unwrap();
                self.memory.load_program(self.registers.get(b) as usize)
                    .map_err(ExecutionError::MemoryError)?;
    
                //Set the program counter to the value in register C
                self.registers.set_program_counter(self.registers.get(c));
            },
            
            Opcode::LoadValue => {
                let a = registers[0].unwrap();
                let value = instruction.value().unwrap();
                self.registers.load_value(a, value);
            },
        }
        let is_load_program = matches!(instruction.opcode(), Opcode::LoadProgram);
        //Increment the program counter unless the executed instruction was LoadProgram
        if !is_load_program {
            self.registers.increment_pc();
        }
        Ok(())
    }

    /// Handles the halting of the UM execution.
    /// 
    /// This method is called when a halt instruction is executed, terminating the program.
    fn halt(&mut self) {
        std::process::exit(0);
    }

    //Implements the Output instruction.
    fn output_instruction(&self, register_index: usize) -> ExecutionResult {
        let value = self.registers.get(register_index);
        print!("{}", std::char::from_u32(value).ok_or(ExecutionError::Other("Invalid Unicode value".to_string()))?);
        Ok(())
    }

    //Implements the Input instruction.
    fn input_instruction(&mut self, register_index: usize) -> ExecutionResult {
        let input = std::io::stdin()
            .bytes()
            .next()
            .transpose()
            .map_err(|_| ExecutionError::Other("Input error".to_string()))?;

        match input {
            Some(value) => self.registers.set(register_index, value as u32),
            None => self.registers.set(register_index, u32::MAX), //0xFFFFFFFF for EOF
        }

        Ok(())
    }
}


/// Unit tests for the UMExecution struct.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_segment() {
        let mut um_execution = UMExecution::new();
        let segment_size = 100; //Example size

        //Set the size in a register (e.g., register 2)
        um_execution.registers.set(2, segment_size);

        // Construct the MapSegment instruction
        // Ensure that the array for registers has correct values. 
        // For example, [None, None, Some(2)] assumes that only register C (the third one) is used.
        let map_segment_instruction = Instruction::new(
            Opcode::MapSegment,
            [Some(0), None, Some(2)], //Setting register A to 0, as it will store the result
            None
        );

        //Execute MapSegment
        assert!(um_execution.execute_instruction(map_segment_instruction).is_ok(), "MapSegment execution failed");

        //Check if the segment ID is stored in register A (register 0)
        let segment_id = um_execution.registers.get(0);
        assert!(segment_id > 0, "Segment ID should be greater than 0");
    }

    #[test]
    fn test_unmap_segment() {
        let mut um_execution = UMExecution::new();
        let segment_size = 100; //Example size

        //First map a segment
        let segment_id = um_execution.memory.map_segment(segment_size);

        //Set the segment ID in a register (e.g., register 2)
        um_execution.registers.set(2, segment_id as u32);

        //Execute UnmapSegment
        let unmap_segment_instruction = Instruction::new(
            Opcode::UnmapSegment,
            [None, None, Some(2)],
            None
        );
        um_execution.execute_instruction(unmap_segment_instruction).unwrap();

        //Attempt to access the unmapped segment and expect an error
        let access_result = um_execution.memory.load(segment_id, 0);
        assert!(access_result.is_err(), "Accessing an unmapped segment should result in an error");
    }

    /// Tests the Output instruction execution.
    ///
    /// This test validates the Output instruction by setting a register value to the ASCII value for 'A' 
    /// and executing the Output instruction. It checks for successful execution of the instruction.
    #[test]
    fn test_output_instruction() {
        let mut um = UMExecution::new();
        um.registers.set(2, 65); //ASCII value for 'A'
        let instruction = Instruction::new(Opcode::Output, [None, None, Some(2)], None);
        assert!(um.execute_instruction(instruction).is_ok());
    }
}