mod fn1 {
    pub fn fn1(number: i32) -> i32 {
        return number;
    }
}


mod fn2 {
    pub fn fn2(number: i32) -> i32 {
        return number + 1;
    }
}


mod fn3 {
    pub fn fn3(number: i32) -> i32 {
        return number + 2;
    }
}


fn main() {
    
    let number = 4;

    let sum = fn1::fn1(number) + fn2::fn2(number) + fn3::fn3(number);

    println!("{}", sum);
}
