>   ## C. Wyatt Polasek & Zach Breene
        A5 - Rust Universal Machine
        finalthoughts.md


> 1. Acknowledgements

        - Help from TAs
        - rum.pdf Instructions Document
        - 411 Class Notes
        - Github Copilot
        - The Rust Programming Language Online Book: https://doc.rust-lang.org/stable/book/
        - https://github.com/CSC411TA/rumdump
        - Stack Overflow Discussions

> 2. Successful Implementations
    
    The implementations for all modules have been implemented correctly.
        - main.rs
        - loading.rs
        - memory.rs
        - registers.rs
        - instructions.rs
        - execution.rs

> 3. Significant Departures from Design

        In our initial design, we had all the neccessary components but
        they were spread across too many modules. Our only significant 
        departure from our design would be the use of fewer modules than
        we had intitally planned.


> 4. Architecture of the System

        1. lib.rs: This is the root module that brings together various components of the system. It imports and makes publicly available five submodules:
        memory, registers, instructions, loading, and execution. This module acts as the entry point, orchestrating the functionalities provided by the
        other modules.
   
        2. loading.rs: This module is responsible for loading programs into the system. It reads a program from a file and converts it into a vector of
        32-bit unsigned integers (Vec<u32>), handling file I/O and error checking. Its main abstraction is translating file data into a format suitable for
        execution by the system.
   
        3. memory.rs: The memory module manages the virtual memory of the system. It abstracts memory into segments, each represented by a vector of 32-bit
        words, and manages these segments using a HashMap. Key operations include mapping/unmapping segments, loading/storing values, and duplicating
        segments for program loading. This module encapsulates the details of memory segmentation and access.
   
        4. registers.rs: This module defines a Registers structure that maintains eight general-purpose registers and a program counter. It provides basic
        operations for manipulating these registers, such as getting and setting values, performing arithmetic and logical operations, and managing the
        program counter. This module abstracts the concept of CPU registers and their operations.
   
        5. instructions.rs: This module defines an Instruction struct and an Opcode enumeration to represent the instructions that the system can execute. 
        It includes functionality to decode instruction words into Instruction objects. This module encapsulates the details of instruction decoding and
        representation.
   
        6. execution.rs: The execution module combines the functionalities of the memory, registers, and instructions modules to execute programs. It
        defines a UMExecution struct that represents the execution state of the machine, including methods for running the program and handling individual
        instructions. This module abstracts the process of program execution and orchestration of the system's components.
   
        7. main.rs: This is the entry point of the application. It integrates the loading and execution modules to load a program and execute it. This
        module handles command-line arguments, initializes the execution environment, and manages the execution lifecycle.



> 5. How long would it take our UM to execute 50 million instructions?

        When testing the midmark.um file, the execution of all instructions took about 20 seconds. 
        Given that midmark.um is 30,109 intructions we can assume it would take approximately 9.2 Hours.
        
        Since,      50 million / 30,109     = ~1,660.63  
        and         1,660.63 * 20 seconds   = 33,207.2 seconds  or  9.2 Hours


> 6. How many hours did we spend analyzing the assignment?

        Approximately 10 Hours.

> 7. How many hours did we spend preparing our design?

        Approximately 6 Hours.

> 8. How many hours did we spend solving the problems after our analysis?

        Approximately 24 Hours.