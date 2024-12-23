use flux_rs::*;

pub struct BigInteger {
    pub data: BitVector
}

impl BigInteger {
    pub fn new(number: u128) -> BigInteger {
        let mut data = Vec::with_capacity(128);
        for bit in 0..128 {
            data.push((number >> bit) & 1 != 0)
        }
        let data = BitVector::new_from_bools(&data);
        BigInteger {
            data
        }
    }

    pub fn new_from_string(literal: &str) -> BigInteger {
        unimplemented!()
    }
}

#[opaque]
#[refined_by(len: int)]
#[invariant(0 <= len)]
pub struct BitVector {
    pub data: Vec<u8>,
    length: usize
}

impl BitVector {
    #[trusted]
    #[sig(fn() -> BitVector[0])]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            length: 0
        }
    }

    #[trusted]
    #[sig(fn(&[bool][@len]) -> BitVector[(len + 7) / 8])]
    pub fn new_from_bools(data: &[bool]) -> Self {
        let length = data.len().div_ceil(8);
        let mut data_u8: Vec<u8> = Vec::with_capacity(length);

        for byte_index in 0..length {
            let mut byte = 0u8;
            for bit_index in 0..8 {
                if byte_index * 8 + bit_index == data.len() {
                    break;
                }
                byte += (data[byte_index * 8 + bit_index] as u8) << bit_index;
            }
            data_u8.push(byte);
        }

        Self {
            data: data_u8,
            length: data.len()
        }
    }

    #[trusted]
    #[sig(fn(&BitVector[@len]) -> Vec<bool>)]
    pub fn get_data(&self) -> Vec<bool> {
        let mut data = Vec::with_capacity(self.length);
        for index in 0..self.length {
            let byte_index = index / 8;
            let bit_index = index % 8;
            data.push(self.data[byte_index] & (1 << bit_index) != 0);
        }
        data
    }
}