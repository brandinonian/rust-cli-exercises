mod fn1;
mod fn2;
mod fn3;

fn main() {
    let number = 3;

    let sum = fn1::fn1(number) + fn2::fn2(number) + fn3::fn3(number);

    println!("{}", sum);
}
