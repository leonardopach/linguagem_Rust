use std::io;

pub fn criptografada() {
    println!("Digite a mensagem que deseja criptografar:");
    let mut mensagem = String::new();
    io::stdin()
        .read_line(&mut mensagem)
        .expect("Erro ao ler a entrada");

    let mensagem_criptografada = criptografar(&mensagem);
    println!("Mensagem criptografada: {}", mensagem_criptografada);
}

fn criptografar(mensagem: &str) -> String {
    let alfabeto = "abcdefghijklmnopqrstuvwxyz";
    let mut mensagem_criptografada = String::new();

    for letra in mensagem.chars() {
        if letra.is_ascii_alphabetic() {
            let letra_lower = letra.to_ascii_lowercase();
            let indice_letra = alfabeto.chars().position(|l| l == letra_lower).unwrap();
            let indice_letra_criptografada = (indice_letra - 3 as usize) % 26;
            let letra_criptografada = alfabeto.chars().nth(indice_letra_criptografada).unwrap();

            mensagem_criptografada.push(letra_criptografada);
        } else {
            mensagem_criptografada.push(letra);
        }
    }

    mensagem_criptografada
}
