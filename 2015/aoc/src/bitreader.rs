use std::io::{self, Seek, SeekFrom};

use bitvec::prelude::*;
use funty::Integral;

pub struct BitReader<T: BitStore = u8> {
    bits: BitVec<T, Msb0>,
    index: usize,
}

impl<T: BitStore> BitReader<T> {
    pub fn new(bits: BitVec<T, Msb0>) -> Self {
        Self { bits, index: 0 }
    }

    pub fn read<U: Integral>(&mut self, count: usize) -> U {
        let result: U = self.bits[self.index..(self.index + count)].load_be();
        self.index += count;
        result
    }

    pub fn read_bool(&mut self) -> bool {
        let result = self.bits[self.index];
        self.index += 1;
        result
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }
}

impl<T: BitStore> Seek for BitReader<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::Current(pos) => (self.index as i64 + pos)
                .try_into()
                .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?,
            SeekFrom::End(pos) => (self.len() as i64 + pos)
                .try_into()
                .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?,
        };

        if new_pos >= self.len() as u64 {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }

        self.index = new_pos.try_into().unwrap();
        Ok(new_pos)
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        Ok(self.index as u64)
    }
}
