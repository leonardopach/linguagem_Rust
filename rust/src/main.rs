use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn main() {
    println!("Digite um valor: ");
    let mut valor = String::new();
    io::stdin()
        .read_line(&mut valor)
        .expect("erro ao ler o input");
    convert_to_int(&valor);

    println!("{valor}");
}
