use std::fs::File;
use std::io::Write;

pub enum Segment {
    ARG,
    LOCAL,
    STATIC,
    THIS,
    THAT,
    POINTER,
    TEMP,
    CONSTANT
}

pub enum Command {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT
}

pub struct VMWriter {
    out_file: File
}

impl VMWriter {
    pub fn new(out_file: File) -> VMWriter {
        VMWriter{
            out_file: out_file
        }
    }

    fn write_segment(&mut self, segment: Segment) {
        self.out_file.write(
            match segment {
                Segment::ARG      => b"argument ",
                Segment::LOCAL    => b"local ",
                Segment::POINTER  => b"pointer ",
                Segment::STATIC   => b"static ",
                Segment::TEMP     => b"temp ",
                Segment::THIS     => b"this ",
                Segment::THAT     => b"that ",
                Segment::CONSTANT => b"constant "
        });
    }

    pub fn write_push(&mut self, segment: Segment, index: u16) {
        self.out_file.write(b"push ");
        self.write_segment(segment);
        self.out_file.write_fmt(
            format_args!("{}\n", index)
        );        
    }

    pub fn write_pop(&mut self, segment: Segment, index: u16) {
        self.out_file.write(b"pop ");
        self.write_segment(segment);
        self.out_file.write_fmt(
            format_args!("{}\n", index)
        );        
    }

    pub fn write_arithmetic(&mut self, command: Command) {
        self.out_file.write(
            match command {
                Command::ADD => b"add\n",
                Command::SUB => b"sub\n",
                Command::AND => b"and\n",
                Command::OR  => b"or\n",
                Command::EQ  => b"eq\n",
                Command::GT  => b"gt\n",
                Command::LT  => b"lt\n",
                Command::NEG => b"neg\n",
                Command::NOT => b"not\n"
            });
    }

    pub fn write_label(&mut self, label: String) {
        self.out_file.write_fmt(
            format_args!("label {}\n", label)
        );
    }

    pub fn write_goto(&mut self, label: String) {
        self.out_file.write_fmt(
            format_args!("goto {}\n", label)
        );
    }

    pub fn write_if(&mut self, label: String) {
        self.out_file.write_fmt(
            format_args!("if-goto {}\n", label)
        );
    }

    pub fn write_call(&mut self, name: String, nArgs: u16) {
        self.out_file.write_fmt(
            format_args!("call {} {}\n",
                          name,
                          nArgs));
    }

    pub fn write_function(&mut self, name: String, nLocals: u16) {
        self.out_file.write_fmt(
            format_args!("function {} {}\n",
                          name,
                          nLocals));
    }

    pub fn write_return(&mut self) {
        self.out_file.write(b"return\n");
    }
}