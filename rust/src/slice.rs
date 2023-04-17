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

    println!("{}, {}", nome, sobrenome);
    let s = String::from("Leonard pacheco");
    println!("{}", primeira_palavra(&s));

    let texto = &s[0..7];
    let longo = &s[7..];
    println!("{texto}");
    println!("{longo}");

    let a = [1, 2, 3, 4, 5];

    let minha_string = String::from("texto longo");

    let palavra = primeira_palavra(&minha_string[..]);
    println!("{palavra}");
    let minha_string_literal = "texto longo";

    let palavra = primeira_palavra(&minha_string_literal[..]);
    println!("{palavra}");

    let palavra = primeira_palavra(minha_string_literal);
    println!("{palavra}");

    let slice = &a[1..3];
    println!("{:?}", slice);
}
