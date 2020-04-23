/// A wrapper for &mut String, which will go back to initial length on drop
///
/// Intended usage is to only append to the buffer after creating a StringStack.
pub struct StringStack<'a> {
    prev_len: usize,
    pub buffer: &'a mut String,
}

impl<'a> StringStack<'a> {
    pub fn new(buffer: &'a mut String) -> Self {
        StringStack {
            prev_len: buffer.len(),
            buffer,
        }
    }
}

impl Drop for StringStack<'_> {
    fn drop(&mut self) {
        self.buffer.truncate(self.prev_len);
    }
}

use std::collections::VecDeque;
use std::io;
use std::io::{BufRead, Read};

/// A wrapper for Read that allows rewinding to a last known place
pub struct RewindBuffer<R> {
    reader: R,
    past: Vec<u8>,
    future: VecDeque<u8>,
}

impl<R: BufRead> RewindBuffer<R> {
    pub fn new(reader: R) -> Self {
        RewindBuffer {
            reader,
            past: Vec::new(),
            future: VecDeque::new(),
        }
    }

    pub fn unread(&mut self, future: &[u8]) {
        for &x in future.iter().rev() {
            self.future.push_front(x);
        }
    }

    pub fn forget_past(&mut self) {
        self.past.clear();
    }

    /// Rewinds to a state from last call to "forget_past"
    pub fn rewind(&mut self) {
        for &x in self.past.iter().rev() {
            self.future.push_front(x);
        }
    }

    // Don't want to implement full BufRead interface, thus providing only read_line
    pub fn read_line(&mut self, output: &mut String) -> io::Result<usize> {
        let (a, b) = self.future.as_slices();
        let n = io::Cursor::new(a)
            .chain(io::Cursor::new(b))
            .chain(&mut self.reader)
            .read_line(output)?;
        if n < self.future.len() {
            self.future.drain(..n);
        } else {
            self.future.clear();
        }
        self.past.extend(&output.as_bytes()[output.len() - n..]);
        Ok(n)
    }
}

impl<R: Read> Read for RewindBuffer<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let (a, b) = self.future.as_slices();
        let n = io::Cursor::new(a)
            .chain(io::Cursor::new(b))
            .chain(&mut self.reader)
            .read(buf)?;
        if n < self.future.len() {
            self.future.drain(..n);
        } else {
            self.future.clear();
        }
        self.past.extend(&buf[..n]);
        Ok(n)
    }
}
