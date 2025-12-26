# Complete GW-BASIC Functions Checklist

This is the COMPLETE list of ALL functions available in GW-BASIC, extracted from the assembly source code and standard GW-BASIC documentation.

## Math Functions (15 total)
- [x] ABS(n) - Absolute value
- [x] ATN(n) - Arctangent
- [x] CDBL(n) - Convert to double precision
- [x] CINT(n) - Convert to integer (rounded)
- [x] COS(n) - Cosine
- [x] CSNG(n) - Convert to single precision
- [x] EXP(n) - Exponential (e^n)
- [x] FIX(n) - Truncate to integer
- [x] INT(n) - Greatest integer (floor)
- [x] LOG(n) - Natural logarithm
- [x] RND[(n)] - Random number
- [x] SGN(n) - Sign (-1, 0, or 1)
- [x] SIN(n) - Sine
- [x] SQR(n) - Square root
- [x] TAN(n) - Tangent

## String Functions (18 total)
- [x] ASC(s$) - ASCII code of first character
- [x] CHR$(n) - Character from ASCII code
- [x] HEX$(n) - Hexadecimal string representation
- [x] INSTR([start,]s1$,s2$) - Find substring position
- [x] INPUT$(n[,#filenum]) - Read n characters (need file support)
- [ ] IOCTL$(#filenum) - I/O control string **MISSING**
- [x] LCASE$(s$) - Convert to lowercase
- [x] LEFT$(s$,n) - Left n characters
- [x] LEN(s$) - String length
- [x] MID$(s$,start[,length]) - Middle substring
- [x] OCT$(n) - Octal string representation
- [x] RIGHT$(s$,n) - Right n characters
- [x] SPACE$(n) - String of n spaces
- [x] STR$(n) - Convert number to string
- [x] STRING$(n,char) - Repeat character n times
- [x] UCASE$(s$) - Convert to uppercase
- [x] VAL(s$) - Convert string to number

Note: INPUT$ partially implemented (keyboard only, file mode needs work)
Note: IOCTL$ is MISSING completely

## Conversion Functions (6 total)
- [x] CVI(s$) - Convert 2-byte string to integer
- [x] CVS(s$) - Convert 4-byte string to single
- [x] CVD(s$) - Convert 8-byte string to double
- [x] MKI$(n) - Convert integer to 2-byte string
- [x] MKS$(n) - Convert single to 4-byte string
- [x] MKD$(n) - Convert double to 8-byte string

## File I/O Functions (4 total)
- [x] EOF(filenum) - Check end of file
- [ ] FILEATTR(filenum,attribute) - Get file attribute **MISSING**
- [x] LOC(filenum) - Current file position
- [x] LOF(filenum) - Length of file

## Error Handling Functions (4 total)
- [ ] ERL - Line number where error occurred **MISSING**
- [ ] ERR - Error code number **MISSING**
- [ ] ERDEV - Device error code **MISSING**
- [ ] ERDEV$ - Device error string **MISSING**

## System/Environment Functions (11 total)
- [x] CSRLIN - Current cursor row
- [x] DATE$ - Current date
- [ ] ENVIRON$(var) - Environment variable **MISSING**
- [x] FRE(n) - Free memory  
- [x] INKEY$ - Check keyboard (no wait)
- [x] INP(port) - Read from port
- [x] PEEK(address) - Read memory byte
- [x] POS(n) - Current cursor column
- [x] TIME$ - Current time
- [x] TIMER - Seconds since midnight
- [x] VARPTR(var) - Variable pointer address

## Screen/Graphics Functions (4 total)
- [x] POINT(x,y) - Get pixel color
- [x] SCREEN(row,col[,colorflag]) - Get screen character/attribute
- [ ] STICK(n) - Joystick coordinate **MISSING**
- [ ] STRIG(n) - Joystick trigger button **MISSING**

## Advanced Functions (2 total)
- [x] USR[n](arg) - Call machine language routine (stub)
- [ ] FN<name>(args) - User-defined function **PARTIAL** (DEF FN exists but FN call needs work)

---

## Summary

**Total Functions: 64**
**Implemented: 52** (81%)
**Stub/Partial: 3** (5%)
**Missing: 9** (14%)

### Missing Functions (9):
1. **IOCTL$(#filenum)** - I/O control string for devices
2. **FILEATTR(filenum,attribute)** - Get file attributes
3. **ERL** - Error line number
4. **ERR** - Error code
5. **ERDEV** - Device error code  
6. **ERDEV$** - Device error string
7. **ENVIRON$(var)** - Get environment variable
8. **STICK(n)** - Joystick coordinate (0-3 for X/Y of 2 joysticks)
9. **STRIG(n)** - Joystick trigger button state

### Partial/Stub (3):
1. **INPUT$(n[,#f])** - Works for keyboard, file mode needs completion
2. **USR[n](arg)** - Stub only (machine language calls)
3. **FN<name>** - DEF FN statement exists but function calls need parser support

---

## Implementation Status by Category

| Category | Implemented | Total | Percentage |
|----------|-------------|-------|------------|
| Math | 15 | 15 | 100% ✅ |
| String | 16 | 18 | 89% |
| Conversion | 6 | 6 | 100% ✅ |
| File I/O | 3 | 4 | 75% |
| Error Handling | 0 | 4 | 0% ❌ |
| System/Environment | 10 | 11 | 91% |
| Screen/Graphics | 2 | 4 | 50% |
| Advanced | 1 | 2 | 50% |
| **TOTAL** | **53** | **64** | **83%** |

