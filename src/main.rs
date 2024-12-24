use std::u128;

use flux_rs::*;
use big_integer::BigInteger;

fn main() {
    let num = BigInteger::new(10);
    let num2 = BigInteger::new(6);

    println!("{:#?}", (num.data ^ num2.data).get_data());
}