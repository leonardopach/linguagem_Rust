#[derive(Debug)]
enum VersaoIp {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum EnderecoIp {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
}

pub fn print_enum() {
    let local = VersaoIp::V4(String::from("127.0.0.1"));

    let loopback = VersaoIp::V6(String::from("::1"));

    println!("{:?}", local);
    println!("{:?}", loopback);

    let local = EnderecoIp::V4(127, 0, 0, 1);

    let loopback = EnderecoIp::V6(String::from("::1"));

    println!("{:?}", local);
    println!("{:?}", loopback);
    let algum_numero = Some(4);
    let algum_texto = Some("um texto");

    println!("{:?} {:?}", algum_numero, algum_texto);

    let numero_ausente: Option<i32> = None;
    println!("{:?}", numero_ausente);
}
