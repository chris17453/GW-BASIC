# GW-BASIC Complete Implementation Checklist

## Keywords & Statements

### Program Control
- [x] RUN - Execute program
- [x] LIST - List program lines
- [x] NEW - Clear program
- [ ] LOAD - Load program from disk
- [ ] SAVE - Save program to disk
- [ ] MERGE - Merge program from disk
- [ ] CHAIN - Chain to another program
- [x] STOP - Stop program execution
- [x] END - End program
- [ ] CONT - Continue after STOP
- [x] GOTO - Jump to line
- [x] GOSUB - Call subroutine
- [x] RETURN - Return from subroutine
- [ ] ON...GOTO - Computed GOTO
- [ ] ON...GOSUB - Computed GOSUB

### Program Editing
- [ ] AUTO - Automatic line numbering
- [ ] DELETE - Delete program lines
- [ ] RENUM - Renumber program lines
- [ ] EDIT - Edit program line
- [ ] TRON - Trace on
- [ ] TROFF - Trace off

### Control Flow
- [x] IF...THEN...ELSE - Conditional
- [x] FOR...TO...STEP - Loop
- [x] NEXT - End of FOR loop
- [x] WHILE...WEND - While loop

### Data I/O
- [x] PRINT - Output to screen
- [x] PRINT# - Output to file
- [ ] PRINT USING - Formatted output
- [x] INPUT - Input from keyboard
- [ ] INPUT# - Input from file
- [ ] LINE INPUT - Input entire line
- [ ] LINE INPUT# - Input line from file
- [ ] WRITE - Write to screen
- [ ] WRITE# - Write to file
- [x] READ - Read from DATA
- [x] DATA - Data storage
- [x] RESTORE - Reset DATA pointer

### Variables & Arrays
- [x] LET - Assignment
- [x] DIM - Dimension arrays
- [ ] ERASE - Erase array
- [ ] CLEAR - Clear variables
- [x] SWAP - Swap variables
- [ ] DEFSTR - Define string variables
- [ ] DEFINT - Define integer variables
- [ ] DEFSNG - Define single variables
- [ ] DEFDBL - Define double variables

### File Operations
- [x] OPEN - Open file
- [x] CLOSE - Close file
- [ ] RESET - Close all files
- [ ] KILL - Delete file
- [ ] NAME - Rename file
- [ ] FILES - List files
- [ ] FIELD - Define random file buffer
- [ ] LSET - Left-justify in field
- [ ] RSET - Right-justify in field
- [ ] GET - Read record
- [ ] PUT - Write record

### Screen/Graphics
- [x] CLS - Clear screen
- [x] LOCATE - Position cursor
- [x] COLOR - Set colors
- [x] SCREEN - Set screen mode
- [ ] WIDTH - Set screen width
- [ ] VIEW - Define viewport
- [ ] WINDOW - Define window coordinates
- [x] PSET - Set pixel
- [ ] PRESET - Reset pixel
- [x] LINE - Draw line
- [x] CIRCLE - Draw circle
- [ ] PAINT - Fill area
- [ ] DRAW - Draw complex shapes
- [ ] GET - Get graphics block
- [ ] PUT - Put graphics block

### Sound
- [x] BEEP - System beep
- [x] SOUND - Generate sound
- [ ] PLAY - Play music

### System
- [x] RANDOMIZE - Seed random generator
- [ ] KEY - Define function keys
- [ ] KEY ON/OFF/LIST - Function key control
- [ ] ON KEY - Function key trap
- [ ] ON ERROR - Error trap
- [ ] RESUME - Resume after error
- [ ] ERROR - Generate error
- [ ] WAIT - Wait for port
- [ ] OUT - Output to port
- [ ] POKE - Write to memory
- [ ] DEF FN - Define function
- [ ] DEF SEG - Define segment
- [ ] BLOAD - Binary load
- [ ] BSAVE - Binary save
- [ ] CALL - Call machine language
- [ ] USR - Call USR function

### Other
- [x] REM - Comment
- [ ] ' - Comment (apostrophe)

## Functions

### Numeric Functions
- [x] ABS(n) - Absolute value
- [x] ATN(n) - Arctangent
- [ ] CDBL(n) - Convert to double
- [x] CINT(n) - Convert to integer
- [x] COS(n) - Cosine
- [x] CSNG(n) - Convert to single
- [ ] CSRLIN - Current row
- [x] EXP(n) - Exponential
- [x] FIX(n) - Truncate
- [x] INT(n) - Integer part
- [x] LOG(n) - Natural logarithm
- [ ] POS(n) - Current column
- [x] RND(n) - Random number
- [x] SGN(n) - Sign
- [x] SIN(n) - Sine
- [x] SQR(n) - Square root
- [x] TAN(n) - Tangent

### String Functions
- [x] ASC(s$) - ASCII code
- [x] CHR$(n) - Character from code
- [x] HEX$(n) - Hexadecimal string
- [x] INSTR([n,]s1$,s2$) - Find substring
- [x] LEFT$(s$,n) - Left substring
- [x] LEN(s$) - String length
- [x] MID$(s$,n[,m]) - Middle substring
- [x] OCT$(n) - Octal string
- [x] RIGHT$(s$,n) - Right substring
- [x] SPACE$(n) - Spaces
- [x] STR$(n) - Number to string
- [x] STRING$(n,c) - Repeated character
- [x] VAL(s$) - String to number
- [ ] INPUT$(n[,#f]) - Read n characters
- [ ] LCASE$(s$) - Lowercase
- [ ] UCASE$(s$) - Uppercase

### Conversion Functions
- [ ] CVI(s$) - String to integer
- [ ] CVS(s$) - String to single
- [ ] CVD(s$) - String to double
- [ ] MKI$(n) - Integer to string
- [ ] MKS$(n) - Single to string
- [ ] MKD$(n) - Double to string

### File Functions
- [ ] EOF(n) - End of file
- [ ] LOC(n) - Current position
- [ ] LOF(n) - File length

### System Functions
- [x] PEEK(n) - Read memory
- [x] INP(n) - Read port
- [x] TIMER - Time since midnight
- [ ] FRE(n) - Free memory
- [ ] VARPTR(var) - Variable pointer
- [ ] INKEY$ - Check keyboard
- [ ] DATE$ - Current date
- [ ] TIME$ - Current time

### Screen Functions
- [ ] POINT(x,y) - Get pixel color
- [ ] SCREEN(row,col[,z]) - Get screen character

## Operators

### Arithmetic
- [x] + - Addition
- [x] - - Subtraction
- [x] * - Multiplication
- [x] / - Division
- [x] \ - Integer division
- [x] MOD - Modulo
- [x] ^ - Exponentiation

### Comparison
- [x] = - Equal
- [x] <> - Not equal
- [x] < - Less than
- [x] > - Greater than
- [x] <= - Less than or equal
- [x] >= - Greater than or equal

### Logical
- [x] AND - Logical AND
- [x] OR - Logical OR
- [x] NOT - Logical NOT
- [x] XOR - Exclusive OR
- [x] EQV - Equivalence
- [x] IMP - Implication

## Summary Statistics
- Total Keywords/Statements: ~100
- Implemented: ~40 (40%)
- Total Functions: ~60
- Implemented: ~35 (58%)
- Total Operators: 19
- Implemented: 19 (100%)

**Overall Completion: ~47%**
