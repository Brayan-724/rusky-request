use std::{
    io::{self, Write},
    ops::{Deref, DerefMut},
};

pub struct MyTerminal<W: Write> {
    term: W,
}

impl<W: Write> From<W> for MyTerminal<W> {
    fn from(from: W) -> Self {
        MyTerminal { term: from }
    }
}

impl<W: Write> Drop for MyTerminal<W> {
    fn drop(&mut self) {
        drop(&mut self.term)
    }
}

impl<W: Write> Deref for MyTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.term
    }
}

impl<W: Write> DerefMut for MyTerminal<W> {
    fn deref_mut(&mut self) -> &mut W {
        &mut self.term
    }
}

impl<W: Write> Write for MyTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _data: Vec<&[u8]> = vec![];
        let mut curr: Vec<u8> = vec![];

        for ch in buf {
            if *ch == b'\n' {
                curr.push(b'\n');
                curr.push(b'\x1B');
                curr.push(b'[');
                curr.push(b'9');
                curr.push(b'9');
                curr.push(b'9');
                curr.push(b'9');
                curr.push(b'9');
                curr.push(b'9');
                curr.push(b'9');
                curr.push(b'9');
                curr.push(b'D');
            } else {
                curr.push(*ch);
            }
        }

        match self.term.write_all(&curr.as_slice()) {
            Ok(_) => Ok(buf.len()),
            Err(err) => Err(err),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.term.flush()
    }
}
