use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes_read: usize,
    operations: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats { reader: _wrapped, bytes_read: 0, operations: 0 }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.reader.read(buf)?;
        self.bytes_read += bytes;
        self.operations += 1;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes_wrote: usize,
    operations: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        return WriteStats { writer: _wrapped, bytes_wrote: 0, operations: 0 }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_wrote
    }

    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.writer.write(buf)?;
        self.bytes_wrote += bytes;
        self.operations += 1;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
