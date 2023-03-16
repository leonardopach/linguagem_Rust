fn tipos_de_dados() {
    let guess: u32 = "42".parse().expect("Não é um número");
    println!("{}", guess);
}
fn main() {
    tipos_de_dados();
}
