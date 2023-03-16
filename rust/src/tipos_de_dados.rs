pub fn tipos_de_dados() {
    let guess: u32 = "42".parse().expect("Não é um número");
    println!("{}", guess);

    let x = 2.4;

    let y: f32 = 3.1;
    println!("{} {}", x, y);

    // adição
    let soma = 5 + 10;

    // subtração
    let diferenca = 95.5 - 4.3;

    // multiplicação
    let produto = 4 * 30;

    // divisão
    let quociente = 56.7 / 32.2;

    // resto
    let resto = 43 % 5;

    println!(
        "soma: {}, diferenca: {}, produto: {}, quociente: {}, resto: {}",
        soma, diferenca, produto, quociente, resto
    );

    let t = true;

    let f: bool = false;

    println!("{}, {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{:?}", tup);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("O valor do y é: {} x: {}, z: {}", y, x, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let quinhentos = x.0;
    println!("quinhentos: {}", quinhentos);

    let seis_ponto_quatro = x.1;
    println!("{}", seis_ponto_quatro);

    let um = x.2;
    println!("{}", um);

    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let primeiro = a[0];
    println!("{}", primeiro);
}
