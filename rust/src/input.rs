use std::io::stdin;

pub fn input() {
    println!("Digite um valor: ");

    let mut valor1 = String::new();

    stdin().read_line(&mut valor1).expect("Erro no input");

    println!("Digite outro  valor: ");
    let mut valor2 = String::new();
    match stdin().read_line(&mut valor2) {
        Ok(_) => {}
        Err(e) => panic!("Erro no input {e}"),
    }

    let valor1 = valor1.trim().parse::<i32>().unwrap();
    let valor2: i32 = valor2.trim().parse::<i32>().unwrap();

    let resultado = valor1 + valor2;
    println!("{resultado}");
}