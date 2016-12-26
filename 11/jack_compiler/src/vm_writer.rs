use std::fs::File;

pub struct VMWriter {
    out_file: File
}

impl VMWriter {
    pub fn new(out_file: File) -> VMWriter {
        VMWriter{
            out_file: out_file
        }
    }

    pub fn write_push() {
        unimplemented!()
    }

    pub fn write_pop() {
        unimplemented!()
    }

    pub fn write_arithmetic() {
        unimplemented!()
    }

    pub fn write_label() {
        unimplemented!()
    }

    pub fn write_goto() {
        unimplemented!()
    }

    pub fn write_if() {
        unimplemented!()
    }

    pub fn write_call() {
        unimplemented!()
    }

    pub fn write_function() {
        unimplemented!()
    }

    pub fn write_return() {
        unimplemented!()
    }
}