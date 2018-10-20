#[derive(Clone, Debug)]
pub struct Node {
    pub value: String,
    pub code: Option<String>,
    pub direita: Option<Box<Node>>,
    pub esquerda: Option<Box<Node>>
}

impl Node {
    pub fn new(v: String) -> Node {
        Node {
            value: v,
            code: Some("".to_ascii_uppercase()),
            direita: None,
            esquerda: None
        }
    }

    pub fn into_morse_tree(&mut self)-> &mut Node {
        self.esquerda = Some(Box::from(Node{
            value: "E".to_ascii_uppercase(),
            code: Some(".".to_ascii_uppercase()),
            direita: Some(Box::from(Node{
                value: "A".to_ascii_uppercase(),
                code: Some(".-".to_ascii_uppercase()),
                direita: Some(Box::from(Node{
                    value: "W".to_ascii_uppercase(),
                    code: Some(".--".to_ascii_uppercase()),
                    direita: Some(Box::from(Node{
                        value: "J".to_ascii_uppercase(),
                        code: Some(".---".to_ascii_uppercase()),
                        direita: Some(Box::from(Node{
                            value: "1".to_ascii_uppercase(),
                            code: Some(".----".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        })),
                        esquerda: None
                    })),
                    esquerda: Some(Box::from(Node{
                        value: "P".to_ascii_uppercase(),
                        code: Some(".--.".to_ascii_uppercase()),
                        esquerda: None,
                        direita: None
                    })),
                })),
                esquerda: Some(Box::from(Node{
                    value: "R".to_ascii_uppercase(),
                    code: Some(".-.".to_ascii_uppercase()),
                    direita: None,
                    esquerda: Some(Box::from(Node{
                        value: "L".to_ascii_uppercase(),
                        code: Some(".-..".to_ascii_uppercase()),
                        direita: None,
                        esquerda: None
                    })),
                })),
            })),
            esquerda: Some(Box::from(Node{
                value: "I".to_ascii_uppercase(),
                code: Some("..".to_ascii_uppercase()),
                direita: Some(Box::from(Node{
                    value: "U".to_ascii_uppercase(),
                    code: Some("..-".to_ascii_uppercase()),
                    direita: Some(Box::from(Node{
                        value: " ".to_ascii_uppercase(),
                        code: Some("..--".to_ascii_uppercase()),
                        direita: Some(Box::from(Node{
                            value: "2".to_ascii_uppercase(),
                            code: Some("..---".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        })),
                        esquerda: None
                    })),
                    esquerda: Some(Box::from(Node{
                        value: "F".to_ascii_uppercase(),
                        code: Some("..-.".to_ascii_uppercase()),
                        esquerda: None,
                        direita: None
                    })),
                })),
                esquerda: Some(Box::from(Node{
                    value: "S".to_ascii_uppercase(),
                    code: Some("...".to_ascii_uppercase()),
                    direita: Some(Box::from(Node{
                        value: "V".to_ascii_uppercase(),
                        code: Some("...-".to_ascii_uppercase()),
                        direita: Some(Box::from(Node{
                            value: "3".to_ascii_uppercase(),
                            code: Some("...--".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        })),
                        esquerda: None
                    })),
                    esquerda: Some(Box::from(Node{
                        value: "H".to_ascii_uppercase(),
                        code: Some("....".to_ascii_uppercase()),
                        direita: Some(Box::from(Node{
                            value: "4".to_ascii_uppercase(),
                            code: Some("....-".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        })),
                        esquerda: Some(Box::from(Node{
                            value: "5".to_ascii_uppercase(),
                            code: Some(".....".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        }))
                    })),
                })),
            })),
        }));

        self.direita = Some(Box::from(Node{
            value: "T".to_ascii_uppercase(),
            code: Some("-".to_ascii_uppercase()),
            direita: Some(Box::from(Node{
                value: "M".to_ascii_uppercase(),
                code: Some("--".to_ascii_uppercase()),
                direita: Some(Box::from(Node{
                    value: "O".to_ascii_uppercase(),
                    code: Some("---".to_ascii_uppercase()),
                    direita: Some(Box::from(Node{
                        value: " ".to_ascii_uppercase(),
                        code: Some("----".to_ascii_uppercase()),
                        direita: Some(Box::from(Node{
                            value: "0".to_ascii_uppercase(),
                            code: Some("-----".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        })),
                        esquerda: Some(Box::from(Node{
                            value: "9".to_ascii_uppercase(),
                            code: Some("----.".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        }))
                    })),
                    esquerda: Some(Box::from(Node{
                        value: " ".to_ascii_uppercase(),
                        code: Some("---.".to_ascii_uppercase()),
                        direita: None,
                        esquerda: Some(Box::from(Node{
                            value: "8".to_ascii_uppercase(),
                            code: Some("---..".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        }))
                    }))
                })),
                esquerda: Some(Box::from(Node{
                    value: "G".to_ascii_uppercase(),
                    code: Some("--.".to_ascii_uppercase()),
                    direita: Some(Box::from(Node{
                        value: "Q".to_ascii_uppercase(),
                        code: Some("--.-".to_ascii_uppercase()),
                        direita: None,
                        esquerda: None
                    })),
                    esquerda: Some(Box::from(Node{
                        value: "Z".to_ascii_uppercase(),
                        code: Some("--..".to_ascii_uppercase()),
                        direita: None,
                        esquerda: Some(Box::from(Node{
                            value: "7".to_ascii_uppercase(),
                            code: Some("--...".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        }))
                    })),
                })),
            })),
            esquerda: Some(Box::from(Node{
                value: "N".to_ascii_uppercase(),
                code: Some("-.".to_ascii_uppercase()),
                direita: Some(Box::from(Node{
                    value: "K".to_ascii_uppercase(),
                    code: Some("-.-".to_ascii_uppercase()),
                    direita: Some(Box::from(Node{
                        value: "Y".to_ascii_uppercase(),
                        code: Some("-.--".to_ascii_uppercase()),
                        direita: None,
                        esquerda: None
                    })),
                    esquerda: Some(Box::from(Node{
                        value: "C".to_ascii_uppercase(),
                        code: Some("-.-.".to_ascii_uppercase()),
                        direita: None,
                        esquerda: None
                    })),
                })),
                esquerda:Some(Box::from(Node{
                    value: "D".to_ascii_uppercase(),
                    code: Some("-..".to_ascii_uppercase()),
                    direita: Some(Box::from(Node{
                        value: "X".to_ascii_uppercase(),
                        code: Some("-..-".to_ascii_uppercase()),
                        direita: None,
                        esquerda: None
                    })),
                    esquerda: Some(Box::from(Node{
                        value: "B".to_ascii_uppercase(),
                        code: Some("-...".to_ascii_uppercase()),
                        direita: None,
                        esquerda: Some(Box::from(Node{
                            value: "6".to_ascii_uppercase(),
                            code: Some("-....".to_ascii_uppercase()),
                            direita: None,
                            esquerda: None
                        }))
                    })),
                })),
            })),
        }));

        self
    }



}

pub fn search_morse(start: &Node, q: &str) -> String {

    if start.clone().code.is_none() {
        return "".to_ascii_uppercase()
    } else if start.clone().code.expect("linh 165") == q {
        return start.clone().value;
    }

    if !start.clone().direita.is_none() {
        if search_morse(&start.clone().direita.expect("linh 169"), q) != "".to_ascii_uppercase() {
            return search_morse(&start.clone().direita.expect("linh 170"), q);
        }
    }

    if !start.clone().esquerda.is_none() {
        if search_morse(&start.clone().esquerda.expect("linh 169"), q) != "".to_ascii_uppercase() {
            return search_morse(&start.clone().esquerda.expect("linh 170"), q);
        }
    }

    "".to_ascii_uppercase()

}
