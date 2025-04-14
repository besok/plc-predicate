# PLC Function Block Diagram (FBD) Blocks and Functions Reference

## Common Applications of Standard Function Blocks

| Function Block Category | Industrial Applications | Examples |
|-------------------------|-------------------------|----------|
| **Basic Logic** (AND, OR, NOT) | • Safety interlocks<br>• Conditional operations<br>• Alarm logic<br>• Sequence control | • Motor start permission (multiple conditions must be TRUE)<br>• Batch process decision trees<br>• Emergency stop circuits |
| **Edge Detection** (R_TRIG, F_TRIG) | • Pulse generation<br>• Event counting<br>• Transition detection<br>• One-shot actions | • Counter triggering<br>• First scan detection<br>• Alarm triggering on state changes<br>• Detecting product passing a sensor |
| **Timers** (TON, TOF, TP) | • Process delays<br>• Cycle timing<br>• Monitoring timeouts<br>• Pulsed operations | • Conveyor delay timers<br>• Mixing time control<br>• Purge cycle timers<br>• Machine lubrication cycles |
| **Counters** (CTU, CTD, CTUD) | • Part counting<br>• Batch processing<br>• Cycle counting<br>• Position tracking | • Packaging operations<br>• Tool wear monitoring<br>• Production counting<br>• Multi-step sequence control |
| **Mathematical Functions** | • Process calculations<br>• Scaling operations<br>• Process control<br>• Data processing | • Flow totalization<br>• Temperature compensation<br>• Ratio control<br>• Energy calculations |
| **Comparison Functions** | • Threshold monitoring<br>• Range checking<br>• Process supervision<br>• Quality control | • Temperature limit alarms<br>• Tank level monitoring<br>• Speed range validation<br>• Material property checking |
| **Data Selection** (MUX, SEL) | • Recipe management<br>• Mode selection<br>• Multi-path operations<br>• Data routing | • Product type selection<br>• Manual/Auto mode switching<br>• Ingredient selection systems<br>• Multi-station transfer systems |
| **Type Conversion** | • Interfacing different systems<br>• Signal processing<br>• HMI communications<br>• Data logging | • Analog-to-digital conversions<br>• Sensor data formatting<br>• Protocol conversions<br>• Database interactions |
| **String Functions** | • HMI communications<br>• Barcode/RFID processing<br>• Logging operations<br>• Recipe handling | • Product code validation<br>• Operator message generation<br>• Batch ID processing<br>• Report generation |
| **PID Control** | • Temperature control<br>• Pressure regulation<br>• Flow control<br>• Level control | • Heat treatment furnaces<br>• Boiler pressure control<br>• Water flow regulation<br>• Tank level management |
| **Communication** | • Network data exchange<br>• Device integration<br>• Remote monitoring<br>• Distributed control | • SCADA integration<br>• Fieldbus device control<br>• Robot integration<br>• Database connectivity |
| **Program Flow Control** | • State machine implementation<br>• Conditional execution<br>• Processing optimization<br>• Multi-path logic | • Batch sequencing<br>• Machine operation modes<br>• Error handling routines<br>• Startup/shutdown sequences |
| **Bit Manipulation** | • Device register control<br>• Status word handling<br>• Protocol implementation<br>• Data packing/unpacking | • Drive parameter setting<br>• Error code processing<br>• Modbus communications<br>• Status reporting |
| **Special Functions** | • Process visualization<br>• Signal conditioning<br>• Operator interfaces<br>• Specialized controls | • Analog signal scaling<br>• Motor control<br>• Specialized machine functions<br>• Equipment calibration |


## Basic Logic Function Blocks

### AND
**Description:** Performs logical AND operation on all inputs. Output is TRUE only when all inputs are TRUE.  
**Inputs:** IN1, IN2, ..., INn (BOOL) - Two or more Boolean inputs  
**Outputs:** OUT (BOOL) - Result of AND operation

### OR
**Description:** Performs logical OR operation on all inputs. Output is TRUE if at least one input is TRUE.  
**Inputs:** IN1, IN2, ..., INn (BOOL) - Two or more Boolean inputs  
**Outputs:** OUT (BOOL) - Result of OR operation

### NOT
**Description:** Inverts the input signal (logical negation).  
**Inputs:** IN (BOOL) - Boolean input to invert  
**Outputs:** OUT (BOOL) - Inverted result

### XOR
**Description:** Exclusive OR. Output is TRUE when odd number of inputs are TRUE.  
**Inputs:** IN1, IN2, ..., INn (BOOL) - Two or more Boolean inputs  
**Outputs:** OUT (BOOL) - Result of XOR operation

### NAND
**Description:** NOT AND operation. Output is FALSE only when all inputs are TRUE.  
**Inputs:** IN1, IN2, ..., INn (BOOL) - Two or more Boolean inputs  
**Outputs:** OUT (BOOL) - Result of NAND operation

### NOR
**Description:** NOT OR operation. Output is TRUE only when all inputs are FALSE.  
**Inputs:** IN1, IN2, ..., INn (BOOL) - Two or more Boolean inputs  
**Outputs:** OUT (BOOL) - Result of NOR operation

### RS (Set-Reset flip-flop)
**Description:** Dominant reset flip-flop. SET input sets output to TRUE, RESET input resets output to FALSE. RESET has priority.  
**Inputs:**
- S (BOOL) - Set input
- R1 (BOOL) - Reset input (dominant)

**Outputs:** Q1 (BOOL) - Output state

### SR (Reset-Set flip-flop)
**Description:** Dominant set flip-flop. SET input sets output to TRUE, RESET input resets output to FALSE. SET has priority.  
**Inputs:**
- S1 (BOOL) - Set input (dominant)
- R (BOOL) - Reset input

**Outputs:** Q1 (BOOL) - Output state

## Edge Detection

### R_TRIG (Rising Edge)
**Description:** Detects rising edge (transition from FALSE to TRUE) of input signal.  
**Inputs:** CLK (BOOL) - Input signal to monitor  
**Outputs:** Q (BOOL) - Pulses TRUE for one cycle when rising edge is detected

### F_TRIG (Falling Edge)
**Description:** Detects falling edge (transition from TRUE to FALSE) of input signal.  
**Inputs:** CLK (BOOL) - Input signal to monitor  
**Outputs:** Q (BOOL) - Pulses TRUE for one cycle when falling edge is detected

## Timers

### TON (Timer On-Delay)
**Description:** Delays activation. Output turns TRUE after input has been TRUE for preset time.  
**Inputs:**
- IN (BOOL) - Timer enable
- PT (TIME) - Preset time

**Outputs:**
- Q (BOOL) - Timer output
- ET (TIME) - Elapsed time

### TOF (Timer Off-Delay)
**Description:** Delays deactivation. Output turns FALSE after input has been FALSE for preset time.  
**Inputs:**
- IN (BOOL) - Timer enable
- PT (TIME) - Preset time

**Outputs:**
- Q (BOOL) - Timer output
- ET (TIME) - Elapsed time

### TP (Pulse Timer)
**Description:** Generates fixed-duration pulse. Output turns TRUE when input goes TRUE and stays TRUE for preset time.  
**Inputs:**
- IN (BOOL) - Trigger input
- PT (TIME) - Pulse duration

**Outputs:**
- Q (BOOL) - Timer output
- ET (TIME) - Elapsed time

### TONR (Retentive On-Delay Timer)
**Description:** Accumulating timer that retains elapsed time even when input becomes FALSE.  
**Inputs:**
- IN (BOOL) - Timer enable
- R (BOOL) - Reset input
- PT (TIME) - Preset time

**Outputs:**
- Q (BOOL) - Timer output
- ET (TIME) - Accumulated elapsed time

## Counters

### CTU (Count Up)
**Description:** Increments counter on rising edge of CU input. Output Q is TRUE when CV ≥ PV.  
**Inputs:**
- CU (BOOL) - Count up input
- R (BOOL) - Reset input
- PV (INT) - Preset value

**Outputs:**
- Q (BOOL) - Counter output (CV ≥ PV)
- CV (INT) - Current count value

### CTD (Count Down)
**Description:** Decrements counter on rising edge of CD input. Output Q is TRUE when CV ≤ 0.  
**Inputs:**
- CD (BOOL) - Count down input
- LD (BOOL) - Load input
- PV (INT) - Preset value

**Outputs:**
- Q (BOOL) - Counter output (CV ≤ 0)
- CV (INT) - Current count value

### CTUD (Count Up-Down)
**Description:** Bi-directional counter that increments on CU rising edge and decrements on CD rising edge.  
**Inputs:**
- CU (BOOL) - Count up input
- CD (BOOL) - Count down input
- R (BOOL) - Reset input
- LD (BOOL) - Load input
- PV (INT) - Preset value

**Outputs:**
- QU (BOOL) - Up counting output (CV ≥ PV)
- QD (BOOL) - Down counting output (CV ≤ 0)
- CV (INT) - Current count value

## Mathematical Functions

### ADD
**Description:** Adds two or more numerical values.  
**Inputs:** IN1, IN2, ..., INn (ANY_NUM) - Two or more numerical inputs  
**Outputs:** OUT (ANY_NUM) - Sum result

### SUB
**Description:** Subtracts second value from first value.  
**Inputs:**
- IN1 (ANY_NUM) - Minuend
- IN2 (ANY_NUM) - Subtrahend

**Outputs:** OUT (ANY_NUM) - Difference result

### MUL
**Description:** Multiplies two or more numerical values.  
**Inputs:** IN1, IN2, ..., INn (ANY_NUM) - Two or more numerical inputs  
**Outputs:** OUT (ANY_NUM) - Product result

### DIV
**Description:** Divides first value by second value.  
**Inputs:**
- IN1 (ANY_NUM) - Dividend
- IN2 (ANY_NUM) - Divisor

**Outputs:** OUT (ANY_NUM) - Quotient result

### MOD
**Description:** Calculates remainder after division.  
**Inputs:**
- IN1 (ANY_INT) - Dividend
- IN2 (ANY_INT) - Divisor

**Outputs:** OUT (ANY_INT) - Remainder result

### SQRT
**Description:** Calculates square root of input value.  
**Inputs:** IN (ANY_REAL) - Value to calculate square root  
**Outputs:** OUT (ANY_REAL) - Square root result

### ABS
**Description:** Returns absolute value (magnitude) of input.  
**Inputs:** IN (ANY_NUM) - Input value  
**Outputs:** OUT (ANY_NUM) - Absolute value result

### EXP
**Description:** Calculates e raised to the power of input value.  
**Inputs:** IN (ANY_REAL) - Exponent value  
**Outputs:** OUT (ANY_REAL) - Result of e^IN

### LN
**Description:** Calculates natural logarithm (base e) of input value.  
**Inputs:** IN (ANY_REAL) - Input value (must be positive)  
**Outputs:** OUT (ANY_REAL) - Natural logarithm result

### LOG
**Description:** Calculates logarithm to base 10 of input value.  
**Inputs:** IN (ANY_REAL) - Input value (must be positive)  
**Outputs:** OUT (ANY_REAL) - Base-10 logarithm result

### SIN
**Description:** Calculates sine of input angle (in radians).  
**Inputs:** IN (ANY_REAL) - Angle in radians  
**Outputs:** OUT (ANY_REAL) - Sine value (-1 to 1)

### COS
**Description:** Calculates cosine of input angle (in radians).  
**Inputs:** IN (ANY_REAL) - Angle in radians  
**Outputs:** OUT (ANY_REAL) - Cosine value (-1 to 1)

### TAN
**Description:** Calculates tangent of input angle (in radians).  
**Inputs:** IN (ANY_REAL) - Angle in radians  
**Outputs:** OUT (ANY_REAL) - Tangent value

### ASIN
**Description:** Calculates arcsine (inverse sine) of input value.  
**Inputs:** IN (ANY_REAL) - Value between -1 and 1  
**Outputs:** OUT (ANY_REAL) - Angle in radians

### ACOS
**Description:** Calculates arccosine (inverse cosine) of input value.  
**Inputs:** IN (ANY_REAL) - Value between -1 and 1  
**Outputs:** OUT (ANY_REAL) - Angle in radians

### ATAN
**Description:** Calculates arctangent (inverse tangent) of input value.  
**Inputs:** IN (ANY_REAL) - Input value  
**Outputs:** OUT (ANY_REAL) - Angle in radians

## Comparison Functions

### GT (Greater Than)
**Description:** Compares if first input is greater than second input.  
**Inputs:**
- IN1 (ANY_ELEMENTARY) - First value
- IN2 (ANY_ELEMENTARY) - Second value

**Outputs:** OUT (BOOL) - TRUE if IN1 > IN2

### GE (Greater Than or Equal)
**Description:** Compares if first input is greater than or equal to second input.  
**Inputs:**
- IN1 (ANY_ELEMENTARY) - First value
- IN2 (ANY_ELEMENTARY) - Second value

**Outputs:** OUT (BOOL) - TRUE if IN1 ≥ IN2

### EQ (Equal)
**Description:** Compares if inputs are equal.  
**Inputs:**
- IN1 (ANY_ELEMENTARY) - First value
- IN2 (ANY_ELEMENTARY) - Second value

**Outputs:** OUT (BOOL) - TRUE if IN1 = IN2

### LT (Less Than)
**Description:** Compares if first input is less than second input.  
**Inputs:**
- IN1 (ANY_ELEMENTARY) - First value
- IN2 (ANY_ELEMENTARY) - Second value

**Outputs:** OUT (BOOL) - TRUE if IN1 < IN2

### LE (Less Than or Equal)
**Description:** Compares if first input is less than or equal to second input.  
**Inputs:**
- IN1 (ANY_ELEMENTARY) - First value
- IN2 (ANY_ELEMENTARY) - Second value

**Outputs:** OUT (BOOL) - TRUE if IN1 ≤ IN2

### NE (Not Equal)
**Description:** Compares if inputs are not equal.  
**Inputs:**
- IN1 (ANY_ELEMENTARY) - First value
- IN2 (ANY_ELEMENTARY) - Second value

**Outputs:** OUT (BOOL) - TRUE if IN1 ≠ IN2

## Data Selection/Manipulation

### MUX (Multiplexer)
**Description:** Selects one of multiple input values based on selector index.  
**Inputs:**
- K (INT) - Selector index (0-based)
- IN0, IN1, ..., INn (ANY) - Input values

**Outputs:** OUT (ANY) - Selected input value

### DEMUX (Demultiplexer)
**Description:** Routes input to one of multiple outputs based on selector index.  
**Inputs:**
- IN (ANY) - Input value
- K (INT) - Selector index (0-based)

**Outputs:** OUT0, OUT1, ..., OUTn (ANY) - Output values

### SEL (Binary Selection)
**Description:** Selects between two values based on Boolean selector.  
**Inputs:**
- G (BOOL) - Selector (FALSE=IN0, TRUE=IN1)
- IN0 (ANY) - First input value
- IN1 (ANY) - Second input value

**Outputs:** OUT (ANY) - Selected value

### MAX
**Description:** Returns maximum value from inputs.  
**Inputs:** IN1, IN2, ..., INn (ANY_ELEMENTARY) - Input values  
**Outputs:** OUT (ANY_ELEMENTARY) - Maximum value

### MIN
**Description:** Returns minimum value from inputs.  
**Inputs:** IN1, IN2, ..., INn (ANY_ELEMENTARY) - Input values  
**Outputs:** OUT (ANY_ELEMENTARY) - Minimum value

### LIMIT
**Description:** Limits value to specified range.  
**Inputs:**
- MN (ANY_NUM) - Minimum limit
- IN (ANY_NUM) - Input value
- MX (ANY_NUM) - Maximum limit

**Outputs:** OUT (ANY_NUM) - Limited value

### MOVE
**Description:** Copies input value to output.  
**Inputs:** IN (ANY) - Input value  
**Outputs:** OUT (ANY) - Copy of input value

## Type Conversion

### INT_TO_REAL
**Description:** Converts integer value to real (floating-point) value.  
**Inputs:** IN (INT) - Integer input  
**Outputs:** OUT (REAL) - Real output

### REAL_TO_INT
**Description:** Converts real (floating-point) value to integer value (truncates decimal part).  
**Inputs:** IN (REAL) - Real input  
**Outputs:** OUT (INT) - Integer output

### BOOL_TO_INT
**Description:** Converts Boolean value to integer (FALSE=0, TRUE=1).  
**Inputs:** IN (BOOL) - Boolean input  
**Outputs:** OUT (INT) - Integer output

### INT_TO_BOOL
**Description:** Converts integer value to Boolean (0=FALSE, non-zero=TRUE).  
**Inputs:** IN (INT) - Integer input  
**Outputs:** OUT (BOOL) - Boolean output

## String Functions

### LEN
**Description:** Returns length of input string.  
**Inputs:** IN (STRING) - Input string  
**Outputs:** OUT (INT) - String length

### LEFT
**Description:** Extracts specified number of characters from left side of string.  
**Inputs:**
- IN (STRING) - Input string
- L (INT) - Number of characters to extract

**Outputs:** OUT (STRING) - Extracted substring

### RIGHT
**Description:** Extracts specified number of characters from right side of string.  
**Inputs:**
- IN (STRING) - Input string
- L (INT) - Number of characters to extract

**Outputs:** OUT (STRING) - Extracted substring

### MID
**Description:** Extracts substring from the middle of input string.  
**Inputs:**
- IN (STRING) - Input string
- P (INT) - Starting position (1-based)
- L (INT) - Number of characters to extract

**Outputs:** OUT (STRING) - Extracted substring

### CONCAT
**Description:** Concatenates two or more strings.  
**Inputs:** IN1, IN2, ..., INn (STRING) - Input strings  
**Outputs:** OUT (STRING) - Concatenated string

### FIND
**Description:** Finds position of substring within string.  
**Inputs:**
- IN1 (STRING) - String to search in
- IN2 (STRING) - Substring to find

**Outputs:** OUT (INT) - Position (1-based) or 0 if not found

### INSERT
**Description:** Inserts string at specified position.  
**Inputs:**
- IN1 (STRING) - Original string
- IN2 (STRING) - String to insert
- P (INT) - Position to insert at (1-based)

**Outputs:** OUT (STRING) - Resulting string

### DELETE
**Description:** Deletes characters from string.  
**Inputs:**
- IN (STRING) - Input string
- P (INT) - Starting position (1-based)
- L (INT) - Number of characters to delete

**Outputs:** OUT (STRING) - Resulting string

### REPLACE
**Description:** Replaces portion of string with another string.  
**Inputs:**
- IN1 (STRING) - Original string
- IN2 (STRING) - Replacement string
- P (INT) - Starting position (1-based)
- L (INT) - Number of characters to replace

**Outputs:** OUT (STRING) - Resulting string

## PID Control

### PID
**Description:** Proportional-Integral-Derivative controller for closed-loop control.  
**Inputs:**
- SETPOINT (REAL) - Desired process value
- ACTUAL (REAL) - Current process value
- KP (REAL) - Proportional gain
- TI (TIME) - Integral time
- TD (TIME) - Derivative time
- CYCLE (TIME) - Cycle time
- MANUAL (BOOL) - Manual mode enable
- MAN_IN (REAL) - Manual input value
- RESET (BOOL) - Reset integrator

**Outputs:**
- OUT (REAL) - Control output
- ERROR (REAL) - Control error (SETPOINT - ACTUAL)

### PI
**Description:** Proportional-Integral controller (simpler PID without derivative term).  
**Inputs:**
- SETPOINT (REAL) - Desired process value
- ACTUAL (REAL) - Current process value
- KP (REAL) - Proportional gain
- TI (TIME) - Integral time
- CYCLE (TIME) - Cycle time
- MANUAL (BOOL) - Manual mode enable
- MAN_IN (REAL) - Manual input value
- RESET (BOOL) - Reset integrator

**Outputs:**
- OUT (REAL) - Control output
- ERROR (REAL) - Control error (SETPOINT - ACTUAL)

### PD
**Description:** Proportional-Derivative controller (without integrator).  
**Inputs:**
- SETPOINT (REAL) - Desired process value
- ACTUAL (REAL) - Current process value
- KP (REAL) - Proportional gain
- TD (TIME) - Derivative time
- CYCLE (TIME) - Cycle time

**Outputs:**
- OUT (REAL) - Control output
- ERROR (REAL) - Control error (SETPOINT - ACTUAL)

## Communication

### SEND
**Description:** Sends data to another device or system.  
**Inputs:**
- REQ (BOOL) - Request transmission
- ID (INT/ANY) - Connection identifier
- DATA (ANY) - Data to send

**Outputs:**
- DONE (BOOL) - Transmission complete
- BUSY (BOOL) - Transmission in progress
- ERROR (BOOL) - Transmission error
- STATUS (WORD) - Error code

### RECV
**Description:** Receives data from another device or system.  
**Inputs:**
- EN_R (BOOL) - Enable reception
- ID (INT/ANY) - Connection identifier

**Outputs:**
- NDR (BOOL) - New data received
- BUSY (BOOL) - Reception in progress
- ERROR (BOOL) - Reception error
- STATUS (WORD) - Error code
- DATA (ANY) - Received data

### READ
**Description:** Reads data from memory area or device.  
**Inputs:**
- REQ (BOOL) - Request read operation
- ADDR (ANY) - Source address
- LEN (INT) - Data length

**Outputs:**
- DONE (BOOL) - Read complete
- BUSY (BOOL) - Read in progress
- ERROR (BOOL) - Read error
- STATUS (WORD) - Error code
- DATA (ANY) - Read data

### WRITE
**Description:** Writes data to memory area or device.  
**Inputs:**
- REQ (BOOL) - Request write operation
- ADDR (ANY) - Destination address
- DATA (ANY) - Data to write

**Outputs:**
- DONE (BOOL) - Write complete
- BUSY (BOOL) - Write in progress
- ERROR (BOOL) - Write error
- STATUS (WORD) - Error code

## Program Flow Control

### JUMP
**Description:** Jumps to specified label within the program.  
**Inputs:** EN (BOOL) - Enable jump  
**Outputs:** None

### LABEL
**Description:** Defines target location for JUMP instruction.  
**Inputs:** None  
**Outputs:** None

### RETURN
**Description:** Returns from function block or function.  
**Inputs:** EN (BOOL) - Enable return  
**Outputs:** None

## Bit Manipulation

### SHL (Shift Left)
**Description:** Shifts bits in input value to the left by specified number of positions.  
**Inputs:**
- IN (ANY_BIT) - Input value
- N (INT) - Number of positions to shift

**Outputs:** OUT (ANY_BIT) - Shifted result

### SHR (Shift Right)
**Description:** Shifts bits in input value to the right by specified number of positions.  
**Inputs:**
- IN (ANY_BIT) - Input value
- N (INT) - Number of positions to shift

**Outputs:** OUT (ANY_BIT) - Shifted result

### ROL (Rotate Left)
**Description:** Rotates bits in input value to the left by specified number of positions.  
**Inputs:**
- IN (ANY_BIT) - Input value
- N (INT) - Number of positions to rotate

**Outputs:** OUT (ANY_BIT) - Rotated result

### ROR (Rotate Right)
**Description:** Rotates bits in input value to the right by specified number of positions.  
**Inputs:**
- IN (ANY_BIT) - Input value
- N (INT) - Number of positions to rotate

**Outputs:** OUT (ANY_BIT) - Rotated result

### BITSET
**Description:** Sets specified bit in input value to 1.  
**Inputs:**
- IN (ANY_BIT) - Input value
- BIT_POS (INT) - Bit position (0-based)

**Outputs:** OUT (ANY_BIT) - Result with bit set

### BITCLR
**Description:** Clears specified bit in input value (sets to 0).  
**Inputs:**
- IN (ANY_BIT) - Input value
- BIT_POS (INT) - Bit position (0-based)

**Outputs:** OUT (ANY_BIT) - Result with bit cleared

### BITTST
**Description:** Tests state of specified bit in input value.  
**Inputs:**
- IN (ANY_BIT) - Input value
- BIT_POS (INT) - Bit position (0-based)

**Outputs:** OUT (BOOL) - State of tested bit (0=FALSE, 1=TRUE)

## Special Functions

### BLINK
**Description:** Generates blinking signal with specified period.  
**Inputs:**
- EN (BOOL) - Enable block
- PERIOD (TIME) - Blink period

**Outputs:** OUT (BOOL) - Blinking output

### RAMP
**Description:** Generates ramp signal from MIN to MAX over specified duration.  
**Inputs:**
- EN (BOOL) - Enable block
- MIN (REAL) - Minimum output value
- MAX (REAL) - Maximum output value
- TR (TIME) - Ramp transition time
- RESET (BOOL) - Reset to minimum value

**Outputs:**
- OUT (REAL) - Ramp output value
- BUSY (BOOL) - Ramp in progress

### SCALE
**Description:** Scales input value from one range to another.  
**Inputs:**
- IN (REAL) - Input value
- IN_MIN (REAL) - Minimum input range
- IN_MAX (REAL) - Maximum input range
- OUT_MIN (REAL) - Minimum output range
- OUT_MAX (REAL) - Maximum output range

**Outputs:** OUT (REAL) - Scaled output value

### UNSCALE
**Description:** Converts scaled value back to original range.  
**Inputs:**
- IN (REAL) - Input value
- OUT_MIN (REAL) - Minimum output range
- OUT_MAX (REAL) - Maximum output range
- IN_MIN (REAL) - Minimum input range
- IN_MAX (REAL) - Maximum input range

**Outputs:** OUT (REAL) - Unscaled output value