// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, the
// program clears the screen, i.e. writes "white" in every pixel.

// Put your code here.

        // Set up pointer to first pixel
        @16384
        D=A
        @screen
        M=D

    (LOOP)
        // Check if any key is pressed
        @24576
        D=M
        @CLEAR
        D;JEQ

    (BLACK)
        // blacken current pixel
        D=-1
        @screen
        A=M
        M=D
        // if (24575 != MEMORY[@SCREEN]) MEMORY[@SCREEN]++;
        @screen
        D=A
        @24575
        D=D-M
        @LOOP
        D;JEQ
        @screen
        M=M+1
        @LOOP
        0;JMP

    (CLEAR)
        // clear current pixel
        D=0
        @screen
        A=M
        M=D
        // if (16384 != MEMORY[@SCREEN]) MEMORY[@SCREEN]--;
        @screen
        D=A
        @16384
        D=D-M
        @LOOP
        D;JEQ
        @screen
        M=M-1
        @LOOP
        0;JMP
        
