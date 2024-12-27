use big_integer::BigInteger;

fn main() {
    let num = BigInteger::new(127);
    let num2 = BigInteger::new(1);
    let num3 = num + num2;

    println!("{:#?}", num3.data);
}