use std::ops::{Add, BitAnd, BitOr, BitXor};

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
        unimplemented!();

        // for symb in literal.chars() {

        // }
    }
}

// impl Add for BigInteger {
//     type Output = BigInteger;

//     // fn add(self, rhs: Self) -> Self::Output {
//     //     let xor = self.data ^ rhs.data;
//     //     let and = self.data & rhs.data;
//     //     for bit in and.get_data() {

//     //     }
//     // }
// }

#[opaque]
#[refined_by(len: int)]
#[invariant(0 <= len)]
pub struct BitVector {
    data: Vec<u8>,
    length: usize
}

type bit_op = fn(&bool, &bool) -> bool;

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
        let mut data = Vec::with_capacity(self.get_length());
        for index in 0..self.get_length() {
            let byte_index = index / 8;
            let bit_index = index % 8;
            data.push(self.data[byte_index] & (1 << bit_index) != 0);
        }
        data
    }

    #[trusted]
    #[sig(fn(&BitVector[@len]) -> usize[len])]
    pub fn get_length(&self) -> usize {
        self.length
    }

    #[sig(fn(&BitVector[@m], &BitVector[@n], bit_op) -> BitVector)]
    fn bit_op(&self, rhs: &Self, func: bit_op) -> BitVector {
        let length = self.get_length().max(rhs.get_length());
        let self_data = self.get_data();
        let rhs_data = rhs.get_data();
        let mut new_data = Vec::with_capacity(length);
        for index in 0..length {
            new_data.push(func(
                self_data.get(index).unwrap_or_else(|| &false), 
                rhs_data.get(index).unwrap_or_else(|| &false)
            ));
        }
        BitVector::new_from_bools(&new_data.as_slice())
    }
}

impl BitXor for BitVector {
    type Output = BitVector;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bit_op(&rhs, |b1, b2| b1 ^ b2)
    }
}

impl BitOr for BitVector {
    type Output = BitVector;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        self.bit_op(&rhs, |b1, b2| b1 | b2)
    }    
}

impl BitAnd for BitVector {
    type Output = BitVector;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.bit_op(&rhs, |b1, b2| b1 & b2)
    }
}