// C. Wyatt Polasek + Zach Breene
// rUM Registers and Program Counter
// rUM - registers.rs

/// Represents the general-purpose registers and program counter for the Universal Machine.
pub struct Registers {
    general: [u32; 8],
    program_counter: u32,
}

impl Registers {
    /// Creates a new Registers instance with all general-purpose registers and the program counter initialized to zero.
    pub fn new() -> Self {
        Registers {
            general: [0; 8],
            program_counter: 0,
        }
    }

    /// Retrieves the value stored in a specific register.
    pub fn get(&self, index: usize) -> u32 {
        self.general[index]
    }

    /// Sets the value of a specific register.
    pub fn set(&mut self, index: usize, value: u32) {
        self.general[index] = value;
    }

    /// Performs a conditional move operation on registers. 
    /// If the value in register C is non-zero, the value of register B is copied into register A.
    pub fn conditional_move(&mut self, a: usize, b: usize, c: usize) {
        if self.general[c] != 0 {
            self.general[a] = self.general[b];
        }
    }

    /// Performs an addition operation on registers. 
    /// The sum of the values in registers B and C is stored in register A, wrapping around on overflow.
    pub fn addition(&mut self, a: usize, b: usize, c: usize) {
        self.general[a] = self.general[b].wrapping_add(self.general[c]);
    }

    /// Performs a multiplication operation on registers. 
    /// The product of the values in registers B and C is stored in register A, wrapping around on overflow.
    pub fn multiplication(&mut self, a: usize, b: usize, c: usize) {
        self.general[a] = self.general[b].wrapping_mul(self.general[c]);
    }

    /// Performs a division operation on registers. 
    /// The quotient of the values in registers B divided by C is stored in register A.
    /// Division by zero should be handled externally.
    pub fn division(&mut self, a: usize, b: usize, c: usize) {
        self.general[a] = self.general[b] / self.general[c]; //Division by zero needs to be handled
    }

    /// Performs a bitwise NOT AND operation on registers. 
    /// The result of NOT (B AND C) is stored in register A.
    pub fn not_and(&mut self, a: usize, b: usize, c: usize) {
        self.general[a] = !(self.general[b] & self.general[c]);
    }

    /// Loads a value directly into a register, bypassing the normal instruction format.
    pub fn load_value(&mut self, a: usize, value: u32) {
        self.general[a] = value;
    }

    /// Sets the value of the program counter, which points to the next instruction to be executed.
    pub fn set_program_counter(&mut self, value: u32) {
        self.program_counter = value;
    }

    /// Retrieves the current value of the program counter.
    pub fn get_program_counter(&self) -> u32 {
        self.program_counter
    }

    /// Increments the program counter, moving to the next instruction.
    pub fn increment_pc(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

}

/// Unit Tests for Registers
#[cfg(test)]
mod register_tests {
    use super::Registers;

    #[test]
    fn test_register_set_and_get() {
        let mut registers = Registers::new();
        registers.set(0, 42);
        assert_eq!(registers.get(0), 42);
    }

    #[test]
    fn test_conditional_move() {
        let mut registers = Registers::new();
        registers.set(0, 10);
        registers.set(1, 20);
        registers.set(2, 1); //Non-zero value for conditional move
        registers.conditional_move(0, 1, 2);
        assert_eq!(registers.get(0), 20);
    }

    #[test]
    fn test_conditional_move_no_action() {
        let mut registers = Registers::new();
        registers.set(0, 10);
        registers.set(1, 20);
        registers.set(2, 0); //Zero value, conditional move should not occur
        registers.conditional_move(0, 1, 2);
        assert_eq!(registers.get(0), 10);
    }

    #[test]
    fn test_register_addition() {
        let mut registers = Registers::new();
        registers.set(1, 10);
        registers.set(2, 32);
        registers.addition(0, 1, 2);
        assert_eq!(registers.get(0), 42);
    }

    #[test]
    fn test_multiplication() {
        let mut registers = Registers::new();
        registers.set(1, 5);
        registers.set(2, 6);
        registers.multiplication(0, 1, 2);
        assert_eq!(registers.get(0), 30);
    }

    #[test]
    fn test_division() {
        let mut registers = Registers::new();
        registers.set(1, 20);
        registers.set(2, 4);
        registers.division(0, 1, 2);
        assert_eq!(registers.get(0), 5);
    }

    #[test]
    fn test_not_and() {
        let mut registers = Registers::new();
        registers.set(1, 0b1100);
        registers.set(2, 0b1010);
        registers.not_and(0, 1, 2);
        assert_eq!(registers.get(0), !(0b1100 & 0b1010));
    }

    #[test]
    fn test_load_value() {
        let mut registers = Registers::new();
        registers.load_value(0, 1234);
        assert_eq!(registers.get(0), 1234);
    }

    #[test]
    fn test_program_counter_operations() {
        let mut registers = Registers::new();
        assert_eq!(registers.get_program_counter(), 0);

        registers.set_program_counter(5);
        assert_eq!(registers.get_program_counter(), 5);

        registers.increment_pc();
        assert_eq!(registers.get_program_counter(), 6);
    }
}