use ohce::ohce;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    if args.len() == 0 {
        panic!("not enough arguments");
    }

    let name = args.collect::<Vec<String>>().join(" ");
    ohce(&name);
}

