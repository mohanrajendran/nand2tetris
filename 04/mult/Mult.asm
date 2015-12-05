// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.
        
        // intialize values, MEMORY[@i]=1, MEMORY[@2]=0 
        @i
        M=1
        @2
        M=0
    (LOOP)
        // if (i > MEMORY[@1]) end;
        @i
        D=M
        @1
        D=D-M
        @END
        D;JGT
        // MEMORY[@2] += MEMORY[@0]
        @0
        D=M
        @2
        M=D+M
        // MEMORY[@i]++;
        @i
        M=M+1
        @LOOP
        0;JMP
    (END)
        @END
        0;JMP
