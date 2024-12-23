use std::u128;

use flux_rs::*;
use big_integer::BigInteger;

fn main() {
    let num = BigInteger::new(u128::MAX);
    println!("{:#?}", num.data.get_data());
}