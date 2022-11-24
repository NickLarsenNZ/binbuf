use crate::{error::BufferError, macros::into_buffer_and_writeable_impl, Endianness};

pub type WriteBufferResult = Result<usize, BufferError>;

pub trait ToWriteBuffer {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push(&mut self, b: u8);
    fn write_slice(&mut self, s: &[u8]) -> WriteBufferResult;
    fn write_vec(&mut self, v: &mut Vec<u8>) -> WriteBufferResult;
    fn bytes(&self) -> &[u8];
}

pub struct WriteBuffer {
    buf: Vec<u8>,
}

impl ToWriteBuffer for WriteBuffer {
    fn new() -> Self {
        WriteBuffer { buf: Vec::new() }
    }

    fn len(&self) -> usize {
        return self.buf.len();
    }

    fn is_empty(&self) -> bool {
        return self.buf.len() == 0;
    }

    fn push(&mut self, b: u8) {
        self.buf.push(b);
    }

    fn write_slice(&mut self, s: &[u8]) -> WriteBufferResult {
        self.buf.extend_from_slice(s);
        Ok(s.len())
    }

    fn write_vec(&mut self, v: &mut Vec<u8>) -> WriteBufferResult {
        self.buf.append(v);
        Ok(v.len())
    }

    fn bytes(&self) -> &[u8] {
        return self.buf.as_slice();
    }
}

pub trait IntoBuffer: Sized {
    const SIZE: usize;

    fn as_be(&self, buf: &mut WriteBuffer) -> WriteBufferResult;
    fn as_le(&self, buf: &mut WriteBuffer) -> WriteBufferResult;
}

pub trait Writeable: Sized {
    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> WriteBufferResult;
}

into_buffer_and_writeable_impl!(u8);
into_buffer_and_writeable_impl!(u16);
into_buffer_and_writeable_impl!(u32);
into_buffer_and_writeable_impl!(u64);
into_buffer_and_writeable_impl!(u128);