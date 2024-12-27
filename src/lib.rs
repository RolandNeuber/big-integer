use std::{fmt::Display, ops::{Add, BitAnd, BitOr, BitXor}};

use flux_rs::*;

/// A struct that defines a natural number of *pratically* arbitrary size.
#[derive(Debug)]
pub struct BigInteger {
    pub data: BitVector
}

impl BigInteger {
    /// Creates a big integer instance from a u128.
    #[sig(fn(u128) -> BigInteger)]
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

    // pub fn new_from_string(literal: &str) -> BigInteger {
    //     // unimplemented!();

    //     for symb in literal.chars() {
    //         let digit: u8 = symb.try_into();

    //     }
    // }
}

impl Add for BigInteger {
    type Output = BigInteger;

    #[sig(fn(Self, Self) -> Self::Output)]
    fn add(self, rhs: Self) -> Self::Output {
        let xor = self.data.clone() ^ rhs.data.clone();
        let and = self.data & rhs.data;
        let mut res = BitVector::new();
        
        let mut carry = false;
        if xor.get_length() == 0 {
            return BigInteger::new(0);
        }
        res.push(xor.get_bit(0));

        for index in 0..and.get_length().max(xor.get_length()) {
            let digit_sum = 
                *xor.get_data().get(index + 1).unwrap_or_else(|| &false) as u8 +
                *and.get_data().get(index).unwrap_or_else(|| &false) as u8 + 
                carry as u8;

            res.push(digit_sum % 2 == 1);

            carry = digit_sum >= 2
        }

        BigInteger{ data: res }
    }
}

/// A struct that defines a vector of bits more efficiently encoded than a Vec of bool.
#[opaque]
#[refined_by(len: int)]
#[invariant(0 <= len)]
#[derive(Clone, Debug)]
pub struct BitVector {
    data: Vec<u8>,
    length: usize
}

/// Defines a boolean binary operation.
type BitOp = fn(&bool, &bool) -> bool;

impl BitVector {
    /// Creates a new empty bit vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_integer::BitVector;
    /// 
    /// BitVector::new();
    /// ```
    #[trusted]
    #[sig(fn() -> Self[0])]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            length: 0
        }
    }

    /// Creates a bit vector from a slice of boolean values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_integer::BitVector;
    /// 
    /// BitVector::new_from_bools(&[true, true, false, true]);
    /// ```
    #[trusted]
    #[sig(fn(&[bool][@len]) -> Self[len])]
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

    /// Pushes a boolean value to the end of the bit vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_integer::BitVector;
    /// 
    /// let vec1 = BitVector::new_from_bools(&[true, false, true]);
    /// let mut vec2 = BitVector::new();
    /// 
    /// vec2.push(true);
    /// vec2.push(false);
    /// vec2.push(true);
    /// 
    /// assert_eq!(vec1, vec2);
    /// ```
    #[trusted]
    #[sig(fn(self: &strg Self[@len], bool) ensures self: Self[len + 1])]
    pub fn push(&mut self, value: bool) {
        if self.get_length() % 8 != 0 {
            let bit_index = self.get_length() % 8;
            let byte = self.get_data_raw_mut().last_mut().expect("Vector length should not be zero.");
            *byte |= (value as u8) << bit_index;
        }
        else {
            self.get_data_raw_mut().push(value as u8);
        }
        *self.get_length_mut() += 1;
    }

    /// Gets the data of the bit vector as a vector of boolean values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_integer::BitVector;
    /// 
    /// let vec = BitVector::new_from_bools(&[true, false, true]);
    /// let data = vec.get_data();
    /// 
    /// assert_eq!(data, vec![true, false, true]);
    /// ```
    #[sig(fn(&Self) -> Vec<bool>)]
    pub fn get_data(&self) -> Vec<bool> {
        let mut data = Vec::with_capacity(self.get_length());
        for index in 0..self.get_length() {
            let byte_index = index / 8;
            let bit_index = index % 8;
            data.push(self.get_data_raw()[byte_index] & (1 << bit_index) != 0);
        }
        data
    }

    /// Gets an immutable reference to the raw byte data that encodes the bits.
    #[trusted]
    #[sig(fn(&Self) -> &Vec<u8>)]
    fn get_data_raw(&self) -> &Vec<u8> {
        &self.data
    }

    /// Gets a mutable reference to the raw byte data that encodes the bits.
    #[trusted]
    #[sig(fn(self: &strg Self[@len]) -> &mut Vec<u8> ensures self: Self[len])]
    fn get_data_raw_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    /// Gets the value of an individual bit of the bit vector a the specified index.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_integer::BitVector;
    /// 
    /// let vec = BitVector::new_from_bools(&[true, false, true]);
    /// let bit1 = vec.get_bit(1);
    /// let bit2 = vec.get_bit(2);
    /// 
    /// assert_eq!(bit1, false);
    /// assert_eq!(bit2, true);
    /// ```
    #[sig(fn(&Self[@len], usize{index: index < len}) -> bool)]
    pub fn get_bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = index % 8;
        let value = 1 << bit_index;
        self.get_data_raw()[byte_index] & value != 0
    }

    /// Sets the value of an individual bit of the bit vector a the specified index. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_integer::BitVector;
    /// 
    /// let mut vec = BitVector::new_from_bools(&[true, false, true]);
    /// vec.set_bit(1, true);
    /// 
    /// assert_eq!(vec.get_data(), &[true, true, true]);
    /// ```
    #[sig(fn(&mut Self[@len], usize{index: index < len}, bool))]
    pub fn set_bit(&mut self, index: usize, value: bool) {
        let byte_index = index / 8;
        let bit_index = index % 8;
        let value = (value as u8) << bit_index;
        if self.get_data_raw()[byte_index] & value != 0 {
            self.get_data_raw_mut()[byte_index] &= value;
        }
        else {
            self.get_data_raw_mut()[byte_index] |= value;
        }
    }

    /// Returns the length of the bit vector as an owned usize.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_integer::BitVector;
    /// 
    /// let vec = BitVector::new_from_bools(&[true, false, true]);
    /// 
    /// assert_eq!(vec.get_length(), 3);
    /// ```
    #[trusted]
    #[sig(fn(&Self[@len]) -> usize[len])]
    pub fn get_length(&self) -> usize {
        self.length
    }

    /// Returns the length of the bit vector as a mutable reference.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use big_integer::BitVector;
    /// 
    /// let vec = BitVector::new_from_bools(&[true, false, true]);
    /// 
    /// assert_eq!(vec.get_length(), 3);
    /// ```
    #[trusted]
    #[sig(fn(&mut Self[@len]) -> &mut usize[len])]
    fn get_length_mut(&mut self) -> &mut usize {
        &mut self.length
    }

    /// Executes a bitwise generic binary operation on two bit vectors.
    /// Returns the result as an owned bit vector.
    /// Should be used as template to implement binary operators.
    /// Should not be used standalone.
    #[trusted]
    #[sig(fn(&Self[@m], &Self[@n], BitOp) -> BitVector[if m >= n {m} else {n}])]
    fn bit_op(&self, rhs: &Self, func: BitOp) -> BitVector {
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

    #[sig(fn(Self[@m], Self[@n]) -> Self[if m >= n {m} else {n}])]
    fn bitxor(self, rhs: Self) -> Self {
        self.bit_op(&rhs, |b1, b2| b1 ^ b2)
    }
}

impl BitOr for BitVector {
    type Output = BitVector;
   
    #[sig(fn(Self[@m], Self[@n]) -> Self[if m >= n {m} else {n}])]
    fn bitor(self, rhs: Self) -> Self {
        self.bit_op(&rhs, |b1, b2| b1 | b2)
    }    
}

impl BitAnd for BitVector {
    type Output = BitVector;

    #[sig(fn(Self[@m], Self[@n]) -> Self[if m >= n {m} else {n}])]
    fn bitand(self, rhs: Self) -> Self {
        self.bit_op(&rhs, |b1, b2| b1 & b2)
    }
}

impl PartialEq for BitVector {
    #[sig(fn(&Self[@m], &Self[@n]) -> bool)] 
    fn eq(&self, other: &Self) -> bool {
        self.get_length() == other.get_length() && self.get_data() == other.get_data()
    }
}