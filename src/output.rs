use std::fs::File;
use std::io::{self, BufWriter, Stdout, Write};
use std::path::PathBuf;

pub enum Output {
    Console(BufWriter<Stdout>),
    File(BufWriter<File>),
    Mem(Vec<u8>),
}

impl Output {
    pub fn console() -> Output {
        Output::Console(io::BufWriter::new(std::io::stdout()))
    }

    pub fn file(path: &PathBuf) -> io::Result<Output> {
        Ok(Output::File(
            // Instead of typical 8kb buffer, we'll make it 800kb to speed up writing
            io::BufWriter::with_capacity(819200, std::fs::File::create(path)?),
        ))
    }

    pub fn mem() -> Output {
        Output::Mem(vec![])
    }
}

impl<'a> Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::Console(w) => w.write(buf),
            Output::File(w) => w.write(buf),
            Output::Mem(v) => {
                // let mut b = Vec::with_capacity(buf.len());
                // b.copy_from_slice(buf);
                let mut b = Vec::from(buf);
                v.append(&mut b);
                io::Result::Ok(buf.len())
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::Console(w) => w.flush(),
            Output::File(w) => w.flush(),
            Output::Mem(_) => io::Result::Ok(()),
        }
    }
}
