fn outra_funcao() {
    println!("Outra função");
}

fn outra_funcao1(x: i32) {
    println!("O valor de x é : {}", x);
}

fn outra_funcao3(x: i32, y: f64) {
    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
}

fn cinco() -> i32 {
    5
}
pub fn principal() {
    outra_funcao();
    outra_funcao1(12);
    println!();
    outra_funcao3(14, 23.1);
    let x = cinco();

    println!("{}", x);
}
