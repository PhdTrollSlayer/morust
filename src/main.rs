use std::io;

mod tree;
use tree::*;

fn main() {
    let mut morse = String::new();

    let mut opt = String::new();
    println!("(1) Morse \n(2)Alfanumérico");
    io::stdin().read_line(&mut opt).unwrap();
    let opt: i32 = opt.trim().parse().unwrap();

    match opt {
        1 => {
            println!("Digite uma mensagem em morse: ", );
            io::stdin().read_line(&mut morse).unwrap();

            let morse = morse.to_ascii_uppercase();

            morse_parser(morse);
        },
        2 => {
            println!("Digite uma mensagem em morse: ", );
            io::stdin().read_line(&mut morse).unwrap();

            let morse = morse.to_ascii_uppercase();

            alfa_parser(morse);
        },
        _ => {}
    }

}

fn morse_parser(m: String) {
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

fn alfa_parser(m: String) {
    let mut tree = Node::new("".to_ascii_uppercase());
    let tree = tree.into_morse_tree();

    let mut iter = m.into_bytes();

    loop {
        if iter.is_empty() {
            break
        }
        let i = iter.remove(0);

        print!("{} ", search_alfa(tree, &String::from_utf8_lossy(&[i]).to_ascii_uppercase()));
    }
    println!("");
}
