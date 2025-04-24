# IEC 61508 SCL Programming Guidelines for PLCs

## Introduction

This document provides comprehensive programming guidelines for Structured Control Language (SCL) in Programmable Logic Controllers (PLCs) when developing safety-critical applications that must comply with IEC 61508. While there is no direct equivalent to MISRA C for SCL, these guidelines incorporate safety principles from various standards including IEC 61508, IEC 61131-3, and industry best practices.

## 1. General Principles

### 1.1 Code Structure
- Organize code into logical, functional modules
- Maintain a consistent programming style throughout the project
- Implement a clear separation between safety-critical and non-safety-critical code
- Keep functions and function blocks small (preferably less than 100 lines)
- Follow a top-down design approach for program architecture

### 1.2 Documentation
- Document the safety integrity level (SIL) requirements for each module
- Maintain comprehensive comments explaining code intent, not just mechanics
- Document all assumptions, preconditions, and postconditions
- Create detailed interface specifications for all function blocks
- Establish traceability between requirements and implementation
- Document all deviations from coding standards with justification

### 1.3 Configuration Management
- Implement strict version control for all code modules
- Use unique identifiers for all program versions
- Ensure configuration management covers all aspects of the system
- Document all changes with associated verification evidence
- Apply formal change management procedures

## 2. Variable Declaration and Management

### 2.1 Variable Declaration
- Declare all variables with explicit data types (no implicit declarations)
- Use meaningful, descriptive variable names that reflect their purpose
- Prefix variable names to indicate scope and type
- Group related variables together in variable blocks
- Document the purpose and valid range of each variable
- Explicitly specify variable scope (GLOBAL, PROGRAM, LOCAL)

### 2.2 Initialization
- Initialize all variables to a known safe state before use
- Never rely on default initialization values
- Re-initialize variables after fault conditions
- For critical variables, implement dual initialization checks
- Verify initialization success before proceeding with safety-critical operations

### 2.3 Data Types
- Use strong typing for all variables
- Avoid implicit type conversions
- Define custom data types for specific application domains
- Use enumerated types for variables with discrete states
- Use structured data types for related data elements
- Document valid ranges and states for all data types

### 2.4 Constants
- Use named constants instead of "magic numbers"
- Define constants for all configurable parameters
- Group related constants in dedicated sections
- Document the source and rationale for safety-critical constants
- Implement verification mechanisms for critical constants

## 3. Program Control Flow

### 3.1 Conditional Statements
- Limit nesting depth to a maximum of 4 levels
- Use positive logic for condition evaluation when possible
- Ensure complex conditions are clearly parenthesized
- Avoid side effects in conditional statements
- Provide explicit ELSE branches in IF-THEN-ELSE statements
- Use Boolean variables for complex conditions to improve readability

### 3.2 CASE Statements
- Always implement an ELSE branch to handle unexpected values
- Check that all possible values are covered
- Handle out-of-range values explicitly
- Use range specifications instead of listing individual values when appropriate
- Avoid overlapping case ranges

### 3.3 Loops
- Ensure all loops have a deterministic exit condition
- Implement explicit upper limits on all iterations
- Avoid modifying loop control variables within the loop body
- Document loop invariants
- Implement watchdog timers for critical loops
- Validate loop exit conditions

### 3.4 Jump Statements
- Avoid unconditional jumps (GOTO)
- If GOTO is absolutely necessary, restrict its use to error handling
- Use EXIT for premature loop termination
- Use CONTINUE with caution and document its purpose
- Avoid complex exit conditions that obscure program flow

## 4. Functions and Function Blocks

### 4.1 Design
- Limit function blocks to a single logical operation
- Keep function blocks small and focused
- Define clear input and output interfaces
- Implement input validation in all function blocks
- Use defensive programming techniques within function blocks

### 4.2 Parameters
- Document valid ranges for all input parameters
- Validate all input parameters before use
- Provide meaningful names for parameters
- Keep parameter lists short (preferably less than 7 parameters)
- Group related parameters in structured data types
- Define default values for optional parameters

### 4.3 Return Values
- Provide explicit error codes for all possible failure conditions
- Never return uninitialized values
- Document the meaning of all return values
- Implement range checking for all return values
- Verify return values are used appropriately by calling code

### 4.4 Error Handling
- Implement consistent error handling mechanisms
- Document all error conditions and responses
- Handle all anticipatable error conditions explicitly
- Provide fallback mechanisms for safety-critical functions
- Log all error conditions with appropriate severity levels
- Implement graceful degradation for non-critical failures

## 5. Memory Management

### 5.1 Memory Allocation
- Avoid dynamic memory allocation when possible
- Use static allocation for safety-critical variables
- Pre-allocate required memory during initialization
- Implement memory usage monitoring
- Validate memory integrity periodically

### 5.2 Array Handling
- Implement explicit array boundary checking
- Initialize arrays completely before use
- Document array size requirements and limitations
- Use constants to define array dimensions
- Implement buffer overflow protection mechanisms

### 5.3 Pointers
- Avoid pointer usage where possible
- If pointers must be used, implement strict validation
- Never use pointer arithmetic
- Initialize pointers to NULL if not immediately assigned
- Check pointers for NULL before dereferencing
- Limit the scope of pointer variables

## 6. Arithmetic Operations

### 6.1 General Guidelines
- Check for potential overflow/underflow conditions
- Implement range checking for all arithmetic operations
- Document assumptions about numeric ranges
- Use appropriate data types for expected value ranges
- Handle division-by-zero exceptions explicitly

### 6.2 Floating-Point Operations
- Use floating-point with caution in safety-critical code
- Never test floating-point values for exact equality
- Document precision requirements and limitations
- Implement range checking for floating-point values
- Consider using fixed-point arithmetic for critical calculations

### 6.3 Type Conversions
- Make all type conversions explicit
- Validate that conversions do not result in data loss
- Document potential precision issues in conversions
- Implement range checking before and after conversions
- Avoid mixing signed and unsigned values in expressions

## 7. Time Management

### 7.1 Timers
- Use standard timer function blocks
- Document timer resolution requirements
- Implement timeout handling for all critical operations
- Verify timer accuracy for safety-critical functions
- Consider system cycle time when configuring timers

### 7.2 Timing Constraints
- Document cycle time requirements
- Implement watchdog timers for critical functions
- Monitor execution times for safety-critical operations
- Handle timing violations explicitly
- Document worst-case execution time analysis

## 8. Input/Output Handling

### 8.1 Input Processing
- Validate all inputs before use
- Implement signal debouncing where appropriate
- Apply appropriate filtering for analog inputs
- Handle out-of-range inputs explicitly
- Implement input reasonability checks

### 8.2 Output Processing
- Verify outputs are within valid ranges
- Implement output monitoring for critical signals
- Create fail-safe output states for error conditions
- Use output forcing with extreme caution and documentation
- Implement output sequencing for safety-critical operations

### 8.3 Communication
- Validate all received data before use
- Implement timeouts for all communication operations
- Use checksums or CRC for data integrity
- Handle communication failures explicitly
- Document communication protocols and error handling

## 9. Defensive Programming Techniques

### 9.1 Redundancy
- Implement diverse redundancy for critical functions
- Use cross-checking for critical calculations
- Implement N-version programming for highest SIL levels
- Maintain independent safety channels
- Document redundancy strategies and verification

### 9.2 Fault Detection
- Implement runtime diagnostics
- Use assertions for internal consistency checking
- Create self-test routines for critical modules
- Implement plausibility checks for critical values
- Monitor system behavior for anomalies

### 9.3 Fault Response
- Define explicit responses for all detectable faults
- Implement graceful degradation where appropriate
- Document fault response strategies
- Prioritize safety over availability
- Implement safe state transitions for critical failures

## 10. Testing and Verification

### 10.1 Unit Testing
- Create comprehensive unit tests for all modules
- Document test coverage metrics
- Test all boundary conditions
- Verify error handling and fault response
- Maintain traceability between requirements and tests

### 10.2 Integration Testing
- Test module interactions thoroughly
- Verify timing constraints at the system level
- Test fault injection and recovery mechanisms
- Document integration test results
- Verify performance under maximum load conditions

### 10.3 Validation
- Validate system behavior against safety requirements
- Perform functional safety assessments
- Maintain validation evidence
- Document validation procedures and results
- Verify compliance with applicable standards

## 11. Specific SCL Guidelines

### 11.1 SCL-Specific Constructs
- Use structured programming constructs (IF, CASE, FOR, WHILE, REPEAT)
- Avoid using unconventional or obscure language features
- Implement clear block structures with proper indentation
- Use parentheses to clarify operator precedence
- Apply consistent naming conventions for all identifiers

### 11.2 SCL Functions
- Use certified library functions when available
- Document and justify the use of non-certified functions
- Implement thorough testing for custom functions
- Verify function behavior under all possible input conditions
- Document function side effects and dependencies

### 11.3 SCL Best Practices
- Use explicit data type conversions (e.g., INT_TO_DINT)
- Implement proper exception handling
- Follow structured programming principles
- Document all non-obvious code constructs
- Implement clear modularity and information hiding

## 12. Certification Considerations

### 12.1 Documentation Requirements
- Maintain complete functional safety documentation
- Document safety functions and their implementation
- Maintain evidence of verification and validation
- Document testing strategies and results
- Ensure traceability throughout the development lifecycle

### 12.2 Tool Qualification
- Document development tools and versions
- Validate tools used for safety-critical development
- Implement compensating measures for non-qualified tools
- Maintain evidence of tool validation
- Document tool-specific configuration and usage

### 12.3 Compliance Evidence
- Document compliance with IEC 61508 requirements
- Maintain evidence of systematic capability
- Document safety integrity level justification
- Prepare functional safety assessment documentation
- Maintain change management records

## 13. Conclusion

Following these guidelines will help ensure that SCL code developed for PLCs in safety-critical applications meets the requirements of IEC 61508 and other relevant safety standards. This documentation should be considered a living document and updated as new best practices and lessons learned emerge.

## Appendix A: Example SCL Code Patterns

### A.1 Proper Variable Declaration and Initialization
```
// Declare and initialize variables with appropriate types
VAR
    Emergency_Stop : BOOL := FALSE;  // Emergency stop signal
    Temperature    : REAL := 0.0;    // Process temperature in Celsius
    Valve_Position : INT  := 0;      // Valve position (0-100%)
END_VAR

// Constants with meaningful names
VAR CONSTANT
    MAX_TEMPERATURE : REAL := 150.0;  // Maximum safe temperature
    MIN_TEMPERATURE : REAL := 10.0;   // Minimum operating temperature
END_VAR
```

### A.2 Input Validation
```
// Input validation function block
FUNCTION_BLOCK ValidateTemperature
VAR_INPUT
    Raw_Temperature : REAL;  // Raw temperature reading
END_VAR
VAR_OUTPUT
    Valid_Temperature : REAL;    // Validated temperature
    Is_Valid         : BOOL;     // Indicates if reading is valid
    Error_Code       : DINT;     // Error code if invalid
END_VAR
VAR
    Temp_Min : REAL := -50.0;  // Minimum valid temperature
    Temp_Max : REAL := 200.0;  // Maximum valid temperature
END_VAR

BEGIN
    // Initialize outputs
    Is_Valid := FALSE;
    Error_Code := 0;
    Valid_Temperature := 0.0;
    
    // Validate range
    IF Raw_Temperature < Temp_Min THEN
        Error_Code := 1;  // Below minimum range
    ELSIF Raw_Temperature > Temp_Max THEN
        Error_Code := 2;  // Above maximum range
    ELSE
        Is_Valid := TRUE;
        Valid_Temperature := Raw_Temperature;
    END_IF;
END_FUNCTION_BLOCK
```

### A.3 Defensive Function Implementation
```
// Function with defensive programming
FUNCTION SafeDivide : REAL
VAR_INPUT
    Numerator   : REAL;
    Denominator : REAL;
END_VAR
VAR_OUTPUT
    Error_Code : DINT;
END_VAR
VAR
    Result : REAL;
    Epsilon : REAL := 0.000001;  // Small value to check near-zero
END_VAR

BEGIN
    // Initialize outputs
    Error_Code := 0;
    Result := 0.0;
    
    // Check for division by zero (or very small value)
    IF ABS(Denominator) < Epsilon THEN
        Error_Code := 1;  // Division by zero error
        RETURN Result;
    END_IF;
    
    // Perform division
    Result := Numerator / Denominator;
    
    // Check for overflow
    IF Result > MAX_REAL THEN
        Error_Code := 2;  // Overflow error
        Result := MAX_REAL;
    END_IF;
    
    RETURN Result;
END_FUNCTION
```

### A.4 Proper Error Handling
```
// Error handling example
FUNCTION_BLOCK TemperatureControl
VAR_INPUT
    SetPoint : REAL;
    Actual   : REAL;
END_VAR
VAR_OUTPUT
    HeaterOutput : REAL;
    SystemStatus : DINT;
END_VAR
VAR
    Error        : REAL;
    SafeDivide_Error : DINT;
    Temp_Valid   : BOOL;
    Valid_Temp   : REAL;
    Temp_Validator : ValidateTemperature;
END_VAR

BEGIN
    // Initialize outputs to safe state
    HeaterOutput := 0.0;
    SystemStatus := 0;
    
    // Validate input temperature
    Temp_Validator(Raw_Temperature := Actual);
    Temp_Valid := Temp_Validator.Is_Valid;
    Valid_Temp := Temp_Validator.Valid_Temperature;
    
    IF NOT Temp_Valid THEN
        // Handle invalid temperature reading
        SystemStatus := 10 + Temp_Validator.Error_Code;
        HeaterOutput := 0.0;  // Safe output for fault condition
        RETURN;
    END_IF;
    
    // Calculate control output (with proper error handling)
    Error := SetPoint - Valid_Temp;
    HeaterOutput := CalculateOutput(Error);
    
    // Validate output is in range
    IF HeaterOutput < 0.0 THEN
        HeaterOutput := 0.0;
        SystemStatus := 20;  // Underflow condition
    ELSIF HeaterOutput > 100.0 THEN
        HeaterOutput := 100.0;
        SystemStatus := 21;  // Overflow condition
    END_IF;
END_FUNCTION_BLOCK
```

## Appendix B: References

1. IEC 61508: Functional Safety of Electrical/Electronic/Programmable Electronic Safety-related Systems
2. IEC 61131-3: Programmable Controllers - Programming Languages
3. Safety Critical Software Development Guidelines
4. MISRA C Guidelines (for conceptual reference)
5. High Integrity Systems Development Best Practices