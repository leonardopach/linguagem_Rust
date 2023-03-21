fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn slice() {
    let string = String::from("Roberto da silva");

    println!("{}", primeira_palavra(&string));
    let valor = &string.as_bytes().len();
    println!("{}", valor);

    let nome = &string[0..7];
    let sobrenome = &string[8..16];

    println!("{}, {}", nome, sobrenome)
}
