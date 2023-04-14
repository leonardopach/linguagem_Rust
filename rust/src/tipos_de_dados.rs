use std::io::stdin;
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

    // tipos compostos
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

    // array
    let a = [1, 2, 3, 4, 5];
    println!("{:a}",);

    let primeiro = a[0];
    println!("{}", primeiro);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let converte: i32 = "42".parse().expect("Error parsing");

    println!("{converte}");
    println!("{}", converte + 1);

    let (nome, idade, profissao) = ("leo", 25, "QA");

    println!("{} {} {}", nome, idade, profissao);

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut vetor = vec![1, 2, 3, 4, 5, 6, 7];

    println!("{:?}", array);
    println!("{}", array[4]);
    vetor.push(10);
    println!("{:?}", vetor);

    let array = [1, 2, 3, 4, 5, 6, 7];

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Erro");

    let converte: usize = input.trim().parse().expect("Erro no parser");

    let element = array[converte];

    println!("{}", element);
}
