use std::env;

fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        panic!("Too many arguments passed.");
    } else if args.len() > 2 {
        let arg1 = args[1].clone();
        let arg2 = args[2].clone();

        if arg1 == "--flag" || arg1 == "-f" {
            println!("{}", arg2);
        } else {
            println!("{}", arg1);
            println!("{}", arg2);
        }
    } else {
        let arg1 = args[1].clone();
        println!("{}", arg1);
    }
}
