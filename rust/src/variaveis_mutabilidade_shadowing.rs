const VALOR: u32 = 45;
pub fn variavies() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("O valor de y é: {}", y);

    let espacos = "   ";
    let espacos = espacos.len();

    println!("A quantidade de espaços é: {}", espacos);
}
