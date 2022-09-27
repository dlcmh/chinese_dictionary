fn main() {
    let raw = "dlc";
    println!("{}", &raw[0..]); // dlc
    println!("{}", &raw[1..]); // lc
    println!("{}", &raw[2..]); // c

    let mut chars = raw.chars();
    println!("{:?}", chars); // Chars(['d', 'l', 'c'])

    chars.next();
    println!("{:?}", chars); // Chars(['l', 'c'])

    chars.next();
    println!("{:?}", chars); // Chars(['c'])

    chars.next();
    println!("{:?}", chars); // Chars([])

    chars.next();
    println!("{:?}", chars); // Chars([])
}
