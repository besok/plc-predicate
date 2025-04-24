# IEC 61508 Sequential Function Chart Programming Guidelines for Safety-Critical PLC Applications

## 1. General Structure and Organization

### 1.1 Program Organization
- Organize SFC programs into functional modules with clear boundaries
- Place safety-critical sequences in dedicated SFCs
- Limit complexity by using hierarchical structures with child SFCs
- Document the purpose and safety criticality of each SFC
- Limit the number of steps and transitions in a single SFC (recommended: 20-30 elements)

### 1.2 Comments and Documentation
- Include header documentation for each SFC with:
    - SFC name and purpose
    - Safety Integrity Level (SIL) information
    - Input/output relationships
    - Safety requirement references
- Document each step's purpose and safety relevance
- Document all transition conditions with safety implications

```
(*
* Emergency Shutdown Sequence - SIL 3
* Purpose: Controls safe shutdown sequence after E-Stop activation
* Safety Req: SR-104 - Complete shutdown within 1500ms of E-Stop
* Inputs: E_Stop_Active, Process_Sensors
* Outputs: Shutdown_Valves, Motor_Controls
*)
```

### 1.3 Naming Conventions
- Use consistent, descriptive naming for steps and transitions
- Implement a standardized prefix system:
    - S_xxx for safety-critical steps (S_ShutdownPump)
    - T_xxx for safety-critical transitions (T_PressureNormal)
    - SI_ for safety inputs (SI_EmergencyStop)
    - SO_ for safety outputs (SO_MainContactor)
    - SF_ for safety flags (SF_ShutdownActive)
- Document the naming convention in project documentation

## 2. SFC Specific Guidelines

### 2.1 Step Design
- Define explicit entry and exit actions for each step
- Keep step actions simple and focused on a single purpose
- Implement time monitoring for steps with duration constraints
- Use step flags to indicate active steps for diagnostics
- Document the safe state of all outputs for each step

```
(* Step S_DepressurizeSystem - SIL 2 *)
STEP S_DepressurizeSystem:
  Entry: 
    SO_BleedValve := TRUE;
    ST_DepressTime(IN:=TRUE, PT:=T#5s);
  
  (* Main safety condition monitored during step *)
  SI_PressureSensor < MIN_SAFE_PRESSURE;
  
  Exit:
    ST_DepressTime(IN:=FALSE);
    SF_DepressurizationComplete := TRUE;
END_STEP
```

### 2.2 Transition Design
- Create simple, explicit transition conditions
- Avoid complex logic in transitions; use flags set in steps if needed
- Implement timeout conditions for all critical transitions
- Document the safety implications of each transition
- Ensure all transitions have clear, deterministic evaluation

```
(* Transition T_PressureSafe - SIL 2 *)
TRANSITION T_PressureSafe FROM S_DepressurizeSystem TO S_ValveShutdown
  (* Simple, clear condition *)
  SI_PressureSensor < MIN_SAFE_PRESSURE OR ST_DepressTime.Q
END_TRANSITION
```

### 2.3 Action Design
- Separate actions by type (entry, during, exit)
- Limit the complexity of actions in each step
- Document the purpose of each action
- Implement feedback verification for critical actions
- Avoid conditional actions in safety-critical SFCs

```
(* Step S_ValveShutdown with feedback monitoring - SIL 3 *)
STEP S_ValveShutdown:
  Entry:
    SO_MainValve := FALSE;
    ST_ValveFeedbackTimer(IN:=TRUE, PT:=T#200ms);
  
  During:
    (* Monitor feedback to verify valve action *)
    IF ST_ValveFeedbackTimer.Q AND SI_ValvePosition <> VALVE_CLOSED THEN
      SF_ValveFault := TRUE;
    END_IF;
  
  Exit:
    ST_ValveFeedbackTimer(IN:=FALSE);
END_STEP
```

## 3. Safety-Specific Practices

### 3.1 Redundancy and Diversity
- Implement parallel branches for critical decisions
- Use diverse transition conditions for redundant paths
- Create voting mechanisms for redundant sensors
- Document all redundancy mechanisms and rationale

```
(* 2oo3 Voting Implementation in SFC transitions *)
TRANSITION T_TemperatureHigh FROM S_NormalOperation TO S_EmergencyShutdown
  (* Transition triggers if at least 2 out of 3 sensors indicate high temp *)
  (SI_Temp1 > TEMP_LIMIT AND SI_Temp2 > TEMP_LIMIT) OR
  (SI_Temp1 > TEMP_LIMIT AND SI_Temp3 > TEMP_LIMIT) OR
  (SI_Temp2 > TEMP_LIMIT AND SI_Temp3 > TEMP_LIMIT)
END_TRANSITION
```

### 3.2 Diagnostics and Monitoring
- Implement step timeout monitoring for all critical steps
- Create diagnostic paths for fault conditions
- Monitor sequence execution for unexpected deviations
- Document diagnostic coverage and techniques
- Create sequence monitoring for critical operations

```
(* Timeout monitoring for critical step *)
STEP S_PumpStartup:
  Entry:
    SO_PumpStart := TRUE;
    ST_PumpStartTimeout(IN:=TRUE, PT:=T#3s);
  
  During:
    (* Monitor for timeout condition *)
    IF ST_PumpStartTimeout.Q AND NOT SI_PumpRunning THEN
      SF_PumpFault := TRUE;
      S_FaultHandling.X := TRUE; (* Activate fault handling step *)
    END_IF;
  
  Exit:
    ST_PumpStartTimeout(IN:=FALSE);
END_STEP
```

### 3.3 Safe State Implementation
- Define explicit safe states for all operating modes
- Create dedicated paths to safe state activation
- Implement multiple entry points to safe state sequence
- Verify safe state achievement through feedback
- Document all paths leading to safe state

```
(* Multiple paths to safe state *)
TRANSITION T_ToSafeState1 FROM S_NormalOperation TO S_SafeState
  SI_EmergencyStop OR SF_SystemFault
END_TRANSITION

TRANSITION T_ToSafeState2 FROM S_Startup TO S_SafeState
  SI_EmergencyStop OR SF_StartupFault
END_TRANSITION

TRANSITION T_ToSafeState3 FROM S_MaintenanceMode TO S_SafeState
  SI_EmergencyStop OR SF_MaintenanceFault
END_TRANSITION
```

### 3.4 Startup and Initialization
- Implement explicit startup sequences with verification steps
- Require manual reset after safety-critical faults
- Verify all safety systems before enabling operation
- Document the complete initialization sequence
- Implement power-up diagnostics as initial steps

```
(* Safe startup sequence *)
STEP S_SystemCheck:
  Entry:
    SF_AllOutputsDisabled := TRUE;
    ST_DiagnosticsTimer(IN:=TRUE, PT:=T#2s);
  
  During:
    (* Run system diagnostics *)
    SF_DiagnosticsActive := TRUE;
    
  Exit:
    ST_DiagnosticsTimer(IN:=FALSE);
    SF_DiagnosticsActive := FALSE;
END_STEP

TRANSITION T_DiagnosticsOK FROM S_SystemCheck TO S_WaitForReset
  ST_DiagnosticsTimer.Q AND NOT SF_DiagnosticsFault
END_TRANSITION
```

## 4. Variable Management

### 4.1 I/O Mapping
- Map physical I/O to symbolic variables
- Implement separate flags for diagnostics
- Document the physical significance of each I/O
- Verify I/O mapping through testing
- Use cross-reference documentation for all I/O points

```
(*
* I/O Mapping Documentation
* SI_EmergencyStop1 - Field Terminal X1:3 - NC contact from E-Stop button 1
* SI_EmergencyStop2 - Field Terminal X1:4 - NC contact from E-Stop button 2
* SO_MainContactor - Field Terminal Y2:1 - 24VDC output to main contactor K1
*)
```

### 4.2 Variable Initialization
- Initialize all variables to safe state values
- Document default (safe) values for all variables
- Implement verification of critical variable initialization
- Use initial steps to ensure proper initialization
- Document initialization dependencies

```
(* Initial step with safe initialization *)
INITIAL STEP S_Initialize:
  Entry:
    (* Initialize all outputs to safe state *)
    SO_MainValve := FALSE;
    SO_PumpControl := FALSE;
    SO_HeaterControl := FALSE;
    
    (* Initialize internal flags *)
    SF_SystemReady := FALSE;
    SF_SafetyOK := FALSE;
END_STEP
```

### 4.3 Data Exchange
- Implement data validation for interfaced data
- Create checksums for critical data exchanges
- Document data dependencies between SFCs
- Implement timeout monitoring for external data
- Validate data ranges before use in safety calculations

## 5. Testing and Verification

### 5.1 Static Analysis
- Define analysis rules specific to SFC programs
- Verify all possible execution paths
- Check for unreachable steps and transitions
- Identify and eliminate deadlocks
- Verify step and transition consistency

### 5.2 Dynamic Testing
- Create test cases for normal, boundary, and fault conditions
- Implement test harnesses for safety-critical sequences
- Verify timing behavior meets safety requirements
- Test all possible sequence transitions
- Document test coverage and results

```
(*
* Test Case: TC-ESTOP-001 - SIL 3
* Purpose: Verify Emergency Shutdown Sequence
* Preconditions: System running in normal mode
* Test Steps:
* 1. Activate E-Stop input
* 2. Verify transition to S_EmergencyShutdown within 50ms
* 3. Verify complete shutdown sequence within 1500ms
* 4. Verify system remains in safe state after E-Stop release
* 5. Verify manual reset required before restart
* Expected Results: Safe state entered and maintained as specified
* Requirements Covered: SR-104, SR-105
*)
```

### 5.3 Fault Injection Testing
- Implement tests for identified fault conditions
- Document fault response and recovery procedures
- Test boundary conditions and timing violations
- Verify system response to step timeouts
- Test system response to power fluctuations

## 6. Time Management and Execution Control

### 6.1 Execution Cycle Control
- Document maximum allowable cycle time for SFC execution
- Implement watchdog monitoring for cycle time violations
- Design safety functions to be cycle-time independent
- Monitor step execution times
- Document timing dependencies and constraints

### 6.2 Step Time Monitoring
- Implement timeout monitoring for all safety-critical steps
- Document maximum allowed step duration
- Create fault paths for step timeout conditions
- Test worst-case response times under load
- Document the timeout handling for each critical step

```
(* Step with time monitoring - SIL 2 *)
STEP S_PressureRelease:
  Entry:
    SO_ReleaseValve := TRUE;
    ST_ReleaseTimer(IN:=TRUE, PT:=T#10s);
  
  During:
    (* Monitor for step timeout *)
    IF ST_ReleaseTimer.Q AND SI_PressureSensor > MAX_RESIDUAL_PRESSURE THEN
      SF_PressureReleaseFault := TRUE;
      S_FaultHandling.X := TRUE;
    END_IF;
  
  Exit:
    ST_ReleaseTimer(IN:=FALSE);
END_STEP
```

### 6.3 Timing Requirements
- Document response time requirements for safety functions
- Implement timing validation routines
- Test worst-case response times under load
- Create timing diagrams for critical safety sequences
- Document timing margins for safety-critical steps

```
(*
* Response Time Documentation - SIL 3
* Emergency Shutdown Sequence:
* - Required response time: 1500ms from E-Stop to safe state
* - Step breakdown:
*   - S_DetectEStop: 25ms (max)
*   - S_DisableOutputs: 50ms (max)
*   - S_DepressurizeSystem: 1000ms (max)
*   - S_VerifySafeState: 100ms (max)
* - Total worst-case: 1175ms
* - Safety margin: 325ms
*)
```

## 7. Configuration Management

### 7.1 Version Control
- Implement version control for all SFC programs
- Document changes with references to safety assessments
- Create revision history in program headers
- Document testing required after program changes
- Implement checksums for program validation

```
(*
* SFC: EmergencyShutdownSequence
* Version: 2.3.1
* Last Modified: 2024-09-15
* Author: J. Smith
* Safety Assessment: SA-ES-2024-005
*
* Revision History:
* 2.3.1 (2024-09-15) - Added feedback monitoring in S_ValveShutdown
* 2.3.0 (2024-08-22) - Enhanced diagnostic coverage
* 2.2.1 (2024-07-04) - Fixed timing issue in depressurization sequence
*)
```

### 7.2 Parameter Management
- Document all configurable parameters
- Implement range checking for safety parameters
- Create verification routines for parameter sets
- Implement access control for parameter changes
- Document parameter dependencies and constraints

```
(*
* Parameter Documentation - SIL 2
* MAX_PRESSURE_LIMIT = 10.5 bar (Range: 8.0-12.0 bar)
* MIN_PRESSURE_LIMIT = 2.0 bar (Range: 1.0-3.0 bar)
* DEPRESSURIZATION_TIME = 5s (Range: 3.0-10.0s)
*
* Constraint: DEPRESSURIZATION_TIME must allow pressure to drop
* from MAX_PRESSURE_LIMIT to MIN_PRESSURE_LIMIT under worst-case conditions
*)
```

## 8. Operational Considerations

### 8.1 Error Handling
- Create dedicated error handling steps
- Document error detection mechanisms
- Implement explicit error logging with timestamps
- Define recovery procedures for each error type
- Ensure error states lead to safe system state

```
(* Error handling step - SIL 2 *)
STEP S_ErrorHandling:
  Entry:
    (* Disable all outputs *)
    SO_AllOutputs := FALSE;
    (* Log error with timestamp *)
    ErrorLogger(ErrorCode := SF_CurrentErrorCode,
               Timestamp := System_Time);
    (* Activate fault indicator *)
    SO_FaultIndicator := TRUE;
  
  Exit:
    SF_ErrorHandled := TRUE;
END_STEP
```

### 8.2 User Interface Safety
- Implement confirmation steps for safety-critical operations
- Verify operator inputs before execution
- Create clear indication of sequence status
- Implement access control for critical functions
- Document HMI safety requirements

```
(* Two-step confirmation for critical operation *)
STEP S_RequestOverride:
  Entry:
    SF_OverrideRequested := TRUE;
    ST_ConfirmTimer(IN:=TRUE, PT:=T#5s);
    
  During:
    (* Wait for confirmation within timeout period *)
    IF SI_ConfirmButton AND ST_ConfirmTimer.IN AND NOT ST_ConfirmTimer.Q THEN
      SF_OverrideConfirmed := TRUE;
    END_IF;
    
    (* Cancel request on timeout *)
    IF ST_ConfirmTimer.Q THEN
      SF_OverrideRequested := FALSE;
    END_IF;
    
  Exit:
    ST_ConfirmTimer(IN:=FALSE);
END_STEP
```

### 8.3 Maintenance and Diagnostic Access
- Create dedicated maintenance mode sequence
- Implement special access procedures for diagnostic functions
- Document maintenance mode safety limitations
- Ensure safe return to normal operation after maintenance
- Implement monitoring of maintenance mode duration

```
(* Maintenance mode step with timeout *)
STEP S_MaintenanceMode:
  Entry:
    SF_MaintenanceActive := TRUE;
    SO_MaintenanceIndicator := TRUE;
    ST_MaintenanceTimer(IN:=TRUE, PT:=T#1h);
    
  During:
    (* Monitor for timeout or exit request *)
    IF ST_MaintenanceTimer.Q OR SI_ExitMaintenance THEN
      S_ExitMaintenance.X := TRUE;
    END_IF;
    
  Exit:
    SF_MaintenanceActive := FALSE;
    SO_MaintenanceIndicator := FALSE;
    ST_MaintenanceTimer(IN:=FALSE);
END_STEP
```

## 9. SFC-Specific Safety Considerations

### 9.1 Simultaneous Branches
- Use simultaneous branches with caution in safety-critical applications
- Document synchronization points and dependencies
- Verify that all parallel branches complete before proceeding
- Implement timeout monitoring for parallel branches
- Test synchronization under worst-case conditions

```
(* Safe use of parallel branches with synchronization *)
STEP S_ParallelOperations:
  Entry:
    (* Start both operations *)
    SF_Operation1Started := TRUE;
    SF_Operation2Started := TRUE;
    ST_ParallelTimeout(IN:=TRUE, PT:=T#2s);
  
  During:
    (* Monitor for timeout *)
    IF ST_ParallelTimeout.Q AND 
       (NOT SF_Operation1Complete OR NOT SF_Operation2Complete) THEN
      SF_SynchronizationFault := TRUE;
    END_IF;
  
  Exit:
    ST_ParallelTimeout(IN:=FALSE);
END_STEP
```

### 9.2 Alternative Branches
- Document decision criteria for alternative paths
- Ensure mutually exclusive conditions for alternatives
- Implement default/fallback path for safety
- Test all alternative paths during validation
- Document the safety implications of each path

```
(* Alternative branches with safety considerations *)
STEP S_DecisionPoint:
  (* Step actions *)
END_STEP

TRANSITION T_Path1 FROM S_DecisionPoint TO S_NormalOperation
  SI_SystemHealthy AND NOT SI_MaintenanceRequired
END_TRANSITION

TRANSITION T_Path2 FROM S_DecisionPoint TO S_MaintenancePrep
  SI_SystemHealthy AND SI_MaintenanceRequired
END_TRANSITION

TRANSITION T_Path3 FROM S_DecisionPoint TO S_SafeState
  NOT SI_SystemHealthy
END_TRANSITION
```

### 9.3 SFC Hierarchy
- Document interactions between parent and child SFCs
- Implement clear handover mechanisms between SFCs
- Verify proper initialization of child SFCs
- Test hierarchical interactions under fault conditions
- Document dependencies between hierarchical levels

```
(* Parent-child SFC handover *)
STEP S_InvokeSubsequence:
  Entry:
    (* Start child SFC with parameters *)
    ChildSFC.SI_StartSequence := TRUE;
    ChildSFC.SI_Parameter1 := CurrentParameter;
    ST_ChildTimeout(IN:=TRUE, PT:=T#10s);
  
  During:
    (* Monitor child completion or timeout *)
    IF ChildSFC.SF_SequenceComplete THEN
      SF_ChildComplete := TRUE;
    END_IF;
    
    IF ST_ChildTimeout.Q AND NOT SF_ChildComplete THEN
      SF_ChildFault := TRUE;
    END_IF;
  
  Exit:
    ChildSFC.SI_StartSequence := FALSE;
    ST_ChildTimeout(IN:=FALSE);
END_STEP
```