# Siemens PLC Function Blocks with Inputs and Outputs

## S7 Extended Timer Blocks

### S_PULSE (Pulse Timer)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| S | BOOL | Start input: triggers the pulse |
| TV | S5TIME/TIME | Time duration for the pulse |
| R | BOOL | Reset input: terminates the pulse |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Q | BOOL | Output: TRUE for the duration of the pulse |
| BI | WORD | Internal timer value (0-999) |

### S_PEXT (Extended Pulse Timer)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| S | BOOL | Start input: triggers the pulse |
| TV | S5TIME/TIME | Time duration for the pulse |
| R | BOOL | Reset input: terminates the pulse |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Q | BOOL | Output: TRUE for the duration of the pulse (extends with S) |
| BI | WORD | Internal timer value (0-999) |

### S_ODT (On-Delay Timer)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| S | BOOL | Start input: begins the delay timing |
| TV | S5TIME/TIME | Time duration for delay |
| R | BOOL | Reset input: resets timer and output |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Q | BOOL | Output: TRUE after delay expires |
| BI | WORD | Internal timer value (0-999) |

### S_ODTS (On-Delay Timer with Retrigger)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| S | BOOL | Start input: begins/retriggers the delay |
| TV | S5TIME/TIME | Time duration for delay |
| R | BOOL | Reset input: resets timer and output |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Q | BOOL | Output: TRUE after delay expires |
| BI | WORD | Internal timer value (0-999) |

### S_OFFDT (Off-Delay Timer)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| S | BOOL | Control input: begins timing when FALSE |
| TV | S5TIME/TIME | Time duration for delay |
| R | BOOL | Reset input: resets timer and output |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Q | BOOL | Output: delayed FALSE after S goes FALSE |
| BI | WORD | Internal timer value (0-999) |

## S7 IEC Counter Extensions

### S_CUD (Up-Down Counter)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| CU | BOOL | Count up input (rising edge) |
| CD | BOOL | Count down input (rising edge) |
| S | BOOL | Set counter to PV value |
| PV | INT | Preset value |
| R | BOOL | Reset counter to zero |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Q | BOOL | Output: TRUE when CV ≥ PV |
| CV | INT | Current counter value |

### S_CU (Up Counter)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| CU | BOOL | Count up input (rising edge) |
| S | BOOL | Set counter to PV value |
| PV | INT | Preset value |
| R | BOOL | Reset counter to zero |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Q | BOOL | Output: TRUE when CV ≥ PV |
| CV | INT | Current counter value |

### S_CD (Down Counter)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| CD | BOOL | Count down input (rising edge) |
| S | BOOL | Load counter with PV value |
| PV | INT | Preset value |
| R | BOOL | Reset counter to zero |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Q | BOOL | Output: TRUE when CV ≤ 0 |
| CV | INT | Current counter value |

## Siemens Communication Blocks

### TSEND_C (TCP Send Connection)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| REQ | BOOL | Initiates data transmission on rising edge |
| CONT | BOOL | Controls connection establishment |
| CONNECT | TCON_Param | Connection parameters (IP address, port, etc.) |
| DATA | VARIANT | Pointer to data to be sent |
| LEN | UINT | Length of data to send in bytes |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| DONE | BOOL | Job completed successfully |
| BUSY | BOOL | Job is in progress |
| ERROR | BOOL | Error occurred |
| STATUS | WORD | Status/error code |

### TRCV_C (TCP Receive Connection)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| EN_R | BOOL | Enable receiving |
| CONT | BOOL | Controls connection establishment |
| CONNECT | TCON_Param | Connection parameters (IP address, port, etc.) |
| LEN | UINT | Maximum length to receive in bytes |
| DATA | VARIANT | Pointer to receive buffer |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| DONE | BOOL | Job completed successfully |
| BUSY | BOOL | Job is in progress |
| ERROR | BOOL | Error occurred |
| STATUS | WORD | Status/error code |
| RCVD_LEN | UINT | Amount of data actually received |

### MODBUS_MASTER
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| REQ | BOOL | Starts request on rising edge |
| MB_ADDR | USINT | Modbus address of slave (1-247) |
| MODE | USINT | Request type (read/write registers/bits) |
| DATA_ADDR | UDINT | Start address in slave device |
| DATA_LEN | UINT | Number of data elements to access |
| DATA_PTR | VARIANT | Pointer to data buffer for read/write |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| DONE | BOOL | Job completed successfully |
| BUSY | BOOL | Job is in progress |
| ERROR | BOOL | Error occurred |
| STATUS | WORD | Status/error code |

### MODBUS_SLAVE
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| MB_ADDR | USINT | Modbus station address (1-247) |
| MB_HOLD_REG | VARIANT | Pointer to holding register data area |
| NDR_RD | BOOL | New read data ready indication |
| DR_WR | BOOL | Data write indication |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| ERROR | BOOL | Error occurred |
| STATUS | WORD | Status/error code |
| BUSY | BOOL | Slave is busy processing a request |

### USS_DRIVE
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| REQ | BOOL | Start request on rising edge |
| DRIVE | USINT | Drive address (1-16) |
| PZD_LEN | USINT | Process data length (2-8 bytes) |
| SPEED_SP | REAL | Speed setpoint |
| CTRL3 | WORD | Control word to drive |
| PARAM | VARIANT | Parameter transfer structure |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| DONE | BOOL | Job completed successfully |
| ERROR | BOOL | Error occurred |
| STATUS | WORD | Status/error code |
| RUN_EN | BOOL | Drive running bit |
| D_DIR | BOOL | Drive direction |
| INHIBIT | BOOL | Drive fault bit |
| FAULT | BOOL | Drive fault bit |
| SPEED | REAL | Actual drive speed (scaled) |
| STATUS1 | WORD | Status word from drive |

### DPRD_DAT / DPWR_DAT (PROFIBUS Data Read/Write)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| LADDR | WORD | Logical address of module/DP slave |
| RECORD | ANY | Pointer to data area (for DPWR_DAT) |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |
| RECORD | ANY | Pointer to data area (for DPRD_DAT) |

### RDREC / WRREC (Read/Write Data Record)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| REQ | BOOL | Start job with rising edge |
| ID | HW_IO | Hardware ID of module |
| INDEX | INT | Data record number |
| MLEN | INT | Maximum length in bytes to be read/written |
| RECORD | VARIANT | Target/source buffer for data record |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| VALID | BOOL | New data record received (RDREC) |
| BUSY | BOOL | Job in progress |
| ERROR | BOOL | Error occurred |
| STATUS | DWORD | Status/error code |
| LEN | INT | Length of read data in bytes (RDREC) |

### GET / PUT (S7 Connection Communication)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| REQ | BOOL | Start job with rising edge |
| ID | WORD | Connection identifier |
| NDR | BOOL | New data ready (GET) |
| ADDR_1 | ANY | Remote source/destination address |
| SD_1 | ANY | Send data buffer (PUT) |
| RD_1 | ANY | Receive data buffer (GET) |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| DONE | BOOL | Job completed successfully |
| ERROR | BOOL | Error occurred |
| STATUS | WORD | Status/error code |

### RALRM (Receive Alarm)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| MODE | INT | Operating mode |
| F_ID | INT | Start event ID |
| MLEN | INT | Maximum length of alarm data to receive |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| NEW | BOOL | New alarm arrived |
| STATUS | DWORD | Status/error code |
| ID | INT | Alarm identifier |
| LEN | INT | Length of received alarm data |
| TINFO | VARIANT | Task information |
| AINFO | VARIANT | Alarm information |

## Siemens PID Control Blocks

### PID_Compact
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Setpoint | REAL | Desired process value |
| Input | REAL | Process value input |
| Input_PER | INT | Analog input value (hardware) |
| ManualEnable | BOOL | Enable manual mode |
| ManualValue | REAL | Manual output value |
| Reset | BOOL | Restart controller |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Output | REAL | Output value |
| Output_PER | INT | Analog output value (hardware) |
| Output_PWM | BOOL | Pulse-width modulated output |
| State | INT | Current controller state |
| Error | BOOL | Error occurred |
| ErrorBits | DWORD | Error type bits |

### PID_3Step
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Setpoint | REAL | Desired process value |
| Input | REAL | Process value input |
| ManualEnable | BOOL | Enable manual mode |
| ManualValue | REAL | Manual output value |
| Actuator_H | BOOL | High limit switch feedback |
| Actuator_L | BOOL | Low limit switch feedback |
| Reset | BOOL | Restart controller |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Output_UP | BOOL | Open actuator command |
| Output_DN | BOOL | Close actuator command |
| State | INT | Current controller state |
| Error | BOOL | Error occurred |
| ErrorBits | DWORD | Error type bits |

### PID_Temp
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Setpoint | REAL | Desired temperature |
| Input | REAL | Process temperature input |
| Input_PER | INT | Analog input value (hardware) |
| ManualEnable | BOOL | Enable manual mode |
| ManualValue | REAL | Manual output value |
| CoolPerOn | BOOL | Activate cooling |
| Reset | BOOL | Restart controller |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Output | REAL | Output value |
| Output_PER | INT | Analog output value (hardware) |
| Output_PWM | BOOL | Pulse-width modulated output |
| OutputHeat | REAL | Heat output value |
| OutputCool | REAL | Cool output value |
| State | INT | Current controller state |
| Error | BOOL | Error occurred |
| ErrorBits | DWORD | Error type bits |

### CONT_C (Continuous Control)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| COM_RST | BOOL | Complete restart |
| MAN_ON | BOOL | Manual mode selection |
| PVPER_ON | BOOL | Process variable via I/O |
| P_SEL | BOOL | Enable P component |
| I_SEL | BOOL | Enable I component |
| D_SEL | BOOL | Enable D component |
| CYCLE | TIME | Sampling time |
| SP_INT | REAL | Internal setpoint |
| PV_IN | REAL | Process variable input |
| PV_PER | WORD | Process variable I/O |
| MAN | REAL | Manual value |
| GAIN | REAL | Proportional gain |
| TI | TIME | Integral time |
| TD | TIME | Derivative time |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| LMN | REAL | Control output |
| LMN_PER | WORD | Control output for I/O |
| QLMN_HLM | BOOL | High limit reached |
| QLMN_LLM | BOOL | Low limit reached |
| PV | REAL | Process variable |
| ER | REAL | Error signal |

### CONT_S (Step Controller)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| COM_RST | BOOL | Complete restart |
| MAN_ON | BOOL | Manual mode selection |
| PVPER_ON | BOOL | Process variable via I/O |
| P_SEL | BOOL | Enable P component |
| I_SEL | BOOL | Enable I component |
| D_SEL | BOOL | Enable D component |
| CYCLE | TIME | Sampling time |
| SP_INT | REAL | Internal setpoint |
| PV_IN | REAL | Process variable input |
| PV_PER | WORD | Process variable I/O |
| GAIN | REAL | Proportional gain |
| TI | TIME | Integral time |
| TD | TIME | Derivative time |
| MTR_TM | TIME | Motor actuating time |
| LMNR_HS | BOOL | High end position of actuator |
| LMNR_LS | BOOL | Low end position of actuator |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| LMNUP | BOOL | Output for up signal |
| LMNDN | BOOL | Output for down signal |
| PV | REAL | Process variable |
| ER | REAL | Error signal |
| QPOS_P | BOOL | Pulse positioning active |

### PULSEGEN (Pulse Generator)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| INV | BOOL | Invert control output |
| PER_TM | TIME | Period time |
| P_B_TM | TIME | Minimum pulse/break time |
| RATIOFAC | REAL | Ratio factor |
| STEP3_ON | BOOL | Three-step control on |
| ST2BI_ON | BOOL | Two/three-step control |
| MAN_ON | BOOL | Manual mode on |
| POS_P_ON | BOOL | Positive pulse on |
| NEG_P_ON | BOOL | Negative pulse on |
| SYN_ON | BOOL | Synchronization on |
| COM_RST | BOOL | Complete restart |
| CYCLE | TIME | Sampling time |
| INP | REAL | Input value |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| QPOS_P | BOOL | Positive pulse output |
| QNEG_P | BOOL | Negative pulse output |

## Siemens Technology Blocks

### MC_Power
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Axis | TO_Axis_PTO/TO_Axis_Servo | Reference to axis |
| Enable | BOOL | Enable/disable axis |
| StopMode | INT | Stop mode for disable |
| EnablePositionLag | BOOL | Enable following error monitoring |
| JogPositive | BOOL | Jog in positive direction |
| JogNegative | BOOL | Jog in negative direction |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Status | BOOL | Axis enabled status |
| Busy | BOOL | Job in progress |
| Error | BOOL | Error occurred |
| ErrorID | WORD | Error ID |

### MC_Home
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Axis | TO_Axis_PTO/TO_Axis_Servo | Reference to axis |
| Execute | BOOL | Start job with rising edge |
| Position | REAL | Absolute position after homing |
| Mode | INT | Homing mode |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Done | BOOL | Job successfully completed |
| Busy | BOOL | Job in progress |
| CommandAborted | BOOL | Job aborted by another command |
| Error | BOOL | Error occurred |
| ErrorID | WORD | Error ID |

### MC_MoveAbsolute
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Axis | TO_Axis_PTO/TO_Axis_Servo | Reference to axis |
| Execute | BOOL | Start job with rising edge |
| Position | REAL | Target position |
| Velocity | REAL | Maximum velocity |
| Acceleration | REAL | Acceleration |
| Deceleration | REAL | Deceleration |
| Jerk | REAL | Jerk limit |
| Direction | INT | Direction of motion |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Done | BOOL | Target position reached |
| Busy | BOOL | Job in progress |
| CommandAborted | BOOL | Job aborted by another command |
| Error | BOOL | Error occurred |
| ErrorID | WORD | Error ID |

### MC_MoveRelative
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Axis | TO_Axis_PTO/TO_Axis_Servo | Reference to axis |
| Execute | BOOL | Start job with rising edge |
| Distance | REAL | Travel distance |
| Velocity | REAL | Maximum velocity |
| Acceleration | REAL | Acceleration |
| Deceleration | REAL | Deceleration |
| Jerk | REAL | Jerk limit |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Done | BOOL | Target position reached |
| Busy | BOOL | Job in progress |
| CommandAborted | BOOL | Job aborted by another command |
| Error | BOOL | Error occurred |
| ErrorID | WORD | Error ID |

### MC_MoveVelocity
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Axis | TO_Axis_PTO/TO_Axis_Servo | Reference to axis |
| Execute | BOOL | Start job with rising edge |
| Velocity | REAL | Target velocity |
| Acceleration | REAL | Acceleration |
| Deceleration | REAL | Deceleration |
| Jerk | REAL | Jerk limit |
| Direction | INT | Direction of motion |
| Current | BOOL | Use current velocity as start |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| InVelocity | BOOL | Target velocity reached |
| Busy | BOOL | Job in progress |
| CommandAborted | BOOL | Job aborted by another command |
| Error | BOOL | Error occurred |
| ErrorID | WORD | Error ID |

### MC_Halt
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| Axis | TO_Axis_PTO/TO_Axis_Servo | Reference to axis |
| Execute | BOOL | Start job with rising edge |
| Deceleration | REAL | Deceleration |
| Jerk | REAL | Jerk limit |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| Done | BOOL | Zero velocity reached |
| Busy | BOOL | Job in progress |
| CommandAborted | BOOL | Job aborted by another command |
| Error | BOOL | Error occurred |
| ErrorID | WORD | Error ID |

### High_Speed_Counter
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| RST | BOOL | Reset counter with rising edge |
| CV | INT | New counter value for preset |
| DIR | BOOL | Count direction (0=up, 1=down) |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT | BOOL | Counter threshold reached |
| CV | INT | Current counter value |
| QDIR | BOOL | Current counting direction |

### Modulo
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | REAL | Input value |
| BASE | REAL | Modulo base value |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT | REAL | Result (remainder) |
| FLOOR | REAL | Integer multiples of the modulo |

## Siemens S7 Math Extensions

### FC105 SCALE (Scale Integer to Real)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | INT | Input value to be scaled |
| HI_LIM | REAL | Upper limit of output range |
| LO_LIM | REAL | Lower limit of output range |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT | REAL | Output scaled value |
| RET_VAL | WORD | Return value/error code |

### FC106 UNSCALE (Scale Real to Integer)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | REAL | Input value to be scaled |
| HI_LIM | REAL | Upper limit of input range |
| LO_LIM | REAL | Lower limit of input range |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT | INT | Output integer value |
| RET_VAL | WORD | Return value/error code |

### FC241 LN (Natural Logarithm)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | REAL | Input value (must be > 0) |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT | REAL | Natural logarithm of input |
| RET_VAL | WORD | Return value/error code |

### FC242 EXP (Exponential Function)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | REAL | Input value |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT | REAL | e raised to the power of input |
| RET_VAL | WORD | Return value/error code |

### FC250 SINCOS (Sine and Cosine)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | REAL | Input angle in radians |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT_SIN | REAL | Sine of input angle |
| OUT_COS | REAL | Cosine of input angle |
| RET_VAL | WORD | Return value/error code |

### FC260 INTEGRAL (Integration)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | REAL | Input value |
| RESET | BOOL | Reset integrator |
| CYCLE | TIME | Integration time interval |
| UPPER_LIM | REAL | Upper limit for result |
| LOWER_LIM | REAL | Lower limit for result |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT | REAL | Integration result |
| RET_VAL | WORD | Return value/error code |

### FC261 DERIVATIVE (Differentiation)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | REAL | Input value |
| CYCLE | TIME | Differentiation time interval |
| CYCLE_P | REAL | Time constant T_D |
| RESET | BOOL | Reset differentiator |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| OUT | REAL | Rate of change result |
| RET_VAL | WORD | Return value/error code |

## Siemens S7 Diagnostics and System Functions

### SFC51 RDSYSST (Read System Status)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| REQ | BOOL | Begin read with rising edge |
| SZL_ID | WORD | ID of system status list |
| INDEX | WORD | Index within system status list |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |
| BUSY | BOOL | Job in progress |
| SZL_HEADER | STRUCT | Header information |
| DR | ANY | Destination for read data |

### SFC52 WR_USMSG (Write User Diagnostic Message)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| SEND | BOOL | Trigger message with rising edge |
| EVT_ID | WORD | Event ID |
| INFO | ANY | Additional information |
| SD1-10 | ANY | Send parameters 1-10 |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |

### SFC20 BLKMOV (Block Move)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| SRCBLK | ANY | Source data block |
| DSTBLK | ANY | Destination data block |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |

### SFC46 STP (Stop CPU)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| None | | |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| None | | |

### SFC64 TIME_TCK (Read System Time in ms)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| None | | |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | TIME | Current system time in ms |

### SFC1 READ_CLK (Read System Clock)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| None | | |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |
| CDT | DATE_AND_TIME | Current date and time |

### SFC0 SET_CLK (Set System Clock)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| PDT | DATE_AND_TIME | New date and time |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |

## Siemens S7 Data Handling Extensions

### FC10 DI_STRNG (Convert DINT to String)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| I | DINT | DINT value to convert |
| POS | INT | String position to begin placement |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |
| S | STRING | Resulting string |

### FC21 LEN (String Length)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| S | STRING | Input string |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | String length |

### FC22 FIND (Find Substring in String)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN1 | STRING | String to search in |
| IN2 | STRING | String to search for |
| L | INT | Starting position for search |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Position of substring or 0 |

### FC26 ENCO (Encode)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | DWORD | Bit pattern to encode |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Position of first 1 from left |

### FC27 DECO (Decode)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | INT | Position to set bit (0-31) |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | DWORD | Bit pattern with bit set |

### FC23 CONCAT (Concatenate Strings)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN1 | STRING | First string |
| IN2 | STRING | Second string |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |
| OUT | STRING | Concatenated result |

### FC120 MOVE_BLK (Move Block)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| IN | Array | Source array |
| COUNT | INT | Number of elements to move |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Return value/error code |
| OUT | Array | Destination array |

### FC17 VAL_STRNG (Convert String to Value)
| Inputs | Data Type | Description |
|--------|-----------|-------------|
| S | STRING | String to convert |
| POS | INT | Position to start |

| Outputs | Data Type | Description |
|---------|-----------|-------------|
| RET_VAL | INT | Error code |
| STRG | STRING | Remaining string |
| VAL | DINT | Converted value |