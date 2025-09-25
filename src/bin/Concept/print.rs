#![allow(unused)]

#[derive(Debug)]

struct Lang {
    language: String,
    version: String,
}

fn main() {
    let per = "World";
    let per2 = "Universe";
    println!("Hello {}", per);
    println!("Hello {1} {0} {1}", per, per2);
    println!("Hello {} {} {}", per, "&", per2);
    println!("Hello {per} & {per2}");

    let x = 2;
    println!("{0} * {0} = {1}", x, x * x);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.08".to_string(),
    };

    println!("{:?}", lang);
    println!("{:#?}", lang);
}
