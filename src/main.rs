use std::io;

mod tree;
use tree::*;

fn main() {
    let mut morse = String::new();

    println!("Digite uma mensagem em morse: ", );
    io::stdin().read_line(&mut morse).unwrap();

    let morse = morse.to_ascii_uppercase();

    parser(morse);
}

fn parser(m: String) {
    let mut tree = Node::new("".to_ascii_uppercase());
    let tree = tree.into_morse_tree();

    let mut iter = m.split_whitespace();

    loop {
        let i = iter.next();

        match i {
            Some(x) => {
                print!("{}", search_morse(tree, x));
            },
            None => {
                println!("");
                break;
            }
        }

    }
}
