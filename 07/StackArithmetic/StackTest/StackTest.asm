@17
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@17
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE0 
D;JEQ 
@SP 
A=M-1 
M=0 
@CONTINUE0 
0;JMP 
(FALSE0) 
@SP 
A=M-1 
M=-1 
(CONTINUE0)
@17
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@16
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE1 
D;JEQ 
@SP 
A=M-1 
M=0 
@CONTINUE1 
0;JMP 
(FALSE1) 
@SP 
A=M-1 
M=-1 
(CONTINUE1)
@16
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@17
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE2 
D;JEQ 
@SP 
A=M-1 
M=0 
@CONTINUE2 
0;JMP 
(FALSE2) 
@SP 
A=M-1 
M=-1 
(CONTINUE2)
@892
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@891
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE3 
D;JLT 
@SP 
A=M-1 
M=0 
@CONTINUE3 
0;JMP 
(FALSE3) 
@SP 
A=M-1 
M=-1 
(CONTINUE3)
@891
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@892
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE4 
D;JLT 
@SP 
A=M-1 
M=0 
@CONTINUE4 
0;JMP 
(FALSE4) 
@SP 
A=M-1 
M=-1 
(CONTINUE4)
@891
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@891
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE5 
D;JLT 
@SP 
A=M-1 
M=0 
@CONTINUE5 
0;JMP 
(FALSE5) 
@SP 
A=M-1 
M=-1 
(CONTINUE5)
@32767
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@32766
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE6 
D;JGT 
@SP 
A=M-1 
M=0 
@CONTINUE6 
0;JMP 
(FALSE6) 
@SP 
A=M-1 
M=-1 
(CONTINUE6)
@32766
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@32767
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE7 
D;JGT 
@SP 
A=M-1 
M=0 
@CONTINUE7 
0;JMP 
(FALSE7) 
@SP 
A=M-1 
M=-1 
(CONTINUE7)
@32766
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@32766
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
D=M-D 
@FALSE8 
D;JGT 
@SP 
A=M-1 
M=0 
@CONTINUE8 
0;JMP 
(FALSE8) 
@SP 
A=M-1 
M=-1 
(CONTINUE8)
@57
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@31
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@53
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
M=D+M
@112
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
M=M-D
@SP 
A=M-1 
M=-M
@SP 
AM=M-1 
D=M 
A=A-1 
M=D&M
@82
D=A
@SP 
A=M 
M=D 
@SP 
M=M+1
@SP 
AM=M-1 
D=M 
A=A-1 
M=D|M
@SP 
A=M-1 
M=!M
(END) 
@END 
0;JMP