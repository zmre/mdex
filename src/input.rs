use std::fs::File;
use std::io::Cursor;
use std::io::{self, BufRead, BufReader, Read, Stdin};
use std::path::PathBuf;

pub enum Input {
    Console(BufReader<Stdin>),
    File(BufReader<File>),
    Mem(Cursor<Vec<u8>>),
}

impl Input {
    pub fn console() -> Input {
        Input::Console(io::BufReader::new(std::io::stdin()))
    }

    pub fn file(path: &PathBuf) -> io::Result<Input> {
        Ok(Input::File(
            // increase performance using a 800k buffer instead of 8k
            io::BufReader::with_capacity(819200, std::fs::File::open(path)?),
        ))
    }

    pub fn mem(bytes: Vec<u8>) -> Input {
        Input::Mem(Cursor::new(bytes))
    }
}

impl<'a> Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Input::Console(r) => r.read(buf),
            Input::File(r) => r.read(buf),
            Input::Mem(b) => b.read(buf),
        }
    }
}

impl<'a> BufRead for Input {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            Input::Console(r) => r.fill_buf(),
            Input::File(r) => r.fill_buf(),
            Input::Mem(b) => b.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            Input::Console(r) => r.consume(amt),
            Input::File(r) => r.consume(amt),
            Input::Mem(b) => b.consume(amt),
        }
    }
}
