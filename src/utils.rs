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

use std::io;
use std::io::{Read, BufRead};
use std::collections::VecDeque;

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
        self.future.extend(self.past.drain(..));
    }

    // Don't want to implement full BufRead interface, thus providing only read_line
    pub fn read_line(&mut self, output: &mut String) -> io::Result<usize> {
        match self.future.iter().copied().position(|x| x == b'\n') {
            Some(position) => {
                // TODO don't create this tmp-vec. Vec just makes easier to utf8-validate.
                let mut vec = Vec::new();
                vec.extend(self.future.drain(..position + 1));
                output.push_str(std::str::from_utf8(&vec).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "UTF validation failure"))?);
                Ok(position+1)
            }
            None => {
                let (a, b) = self.future.as_slices();
                io::Cursor::new(a).chain(io::Cursor::new(b)).read_to_string(output)?;
                self.future.clear();
                self.reader.read_line(output)
            }
        }
    }
}

impl<R: Read> Read for RewindBuffer<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.future.is_empty() {
            let n = std::cmp::min(buf.len(), self.future.len());
            buf.iter_mut().zip(self.future.drain(..n)).for_each(|(dest, src)| *dest = src);
            Ok(n)
        } else {
            let n = self.reader.read(buf)?;
            self.past.extend(&buf[..n]);
            Ok(n)
        }
    }
}
