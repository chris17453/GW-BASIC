10 REM Mandelbrot Set - High Resolution
20 PRINT "=== Mandelbrot Set (High-Res) ==="
30 PRINT "This may take a while..."
40 INPUT "Max iterations (4-64, faster at 8-16)"; MAXITER
50 IF MAXITER < 4 THEN MAXITER = 4
60 IF MAXITER > 64 THEN MAXITER = 64
70 SCREEN 1
80 CLS
90 REM Screen dimensions for graphics mode
100 LET SW = 320
110 LET SH = 200
120 REM Mandelbrot parameters (aspect-correct for 320x200)
130 LET XMIN = -2.5
140 LET XMAX = 1.0
150 REM Keep center at 0, shrink Y range to match 1.6:1 screen aspect
160 LET YMIN = -1.1
170 LET YMAX = 1.1
180 PRINT "Rendering (fast mode)..."
190 REM Fast render: sample every 2x2 block
200 FOR SY = 0 TO SH - 1 STEP 2
210   FOR SX = 0 TO SW - 1 STEP 2
220     REM Map screen coords to complex plane
230     LET X0 = XMIN + (XMAX - XMIN) * SX / SW
240     LET Y0 = YMIN + (YMAX - YMIN) * SY / SH
250     REM Initialize iteration variables
260     LET X = 0
270     LET Y = 0
280     LET ITER = 0
290     REM Iterate
300     LET X2 = X * X
310     LET Y2 = Y * Y
320     IF X2 + Y2 > 4 THEN GOTO 380
330     IF ITER >= MAXITER THEN GOTO 380
340     LET YTEMP = 2 * X * Y + Y0
350     LET X = X2 - Y2 + X0
360     LET Y = YTEMP
370     LET ITER = ITER + 1: GOTO 300
380     REM Draw pixel if in set
390     IF ITER >= MAXITER THEN C = 0
391     IF ITER < MAXITER THEN C = (ITER MOD 15) + 1
395     PSET (SX, SY), C
396     IF SX + 1 < SW THEN PSET (SX + 1, SY), C
397     IF SY + 1 < SH THEN PSET (SX, SY + 1), C
398     IF SX + 1 < SW THEN IF SY + 1 < SH THEN PSET (SX + 1, SY + 1), C
400   NEXT SX
410   REM Progress indicator
420   IF SY MOD 10 = 0 THEN LOCATE 1, 1: PRINT INT(SY * 100 / SH); "%"
430 NEXT SY
440 LOCATE 1, 1
450 PRINT "Done!    "
460 LOCATE 23, 1
470 PRINT "Press any key..."
480 A$ = INPUT$(1)
490 SCREEN 0: WIDTH 80: CLS
500 END
