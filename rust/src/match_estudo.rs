#[allow(dead_code)]
#[derive(Debug)]
pub enum Estado {
    Alabama,
    Alaska,
    // ... etc
}
#[allow(dead_code)]
pub enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter(Estado),
}

pub fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => {
            println!("Moeda da sorte!");
            1
        }
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter(estado) => {
            println!("Quarter do estado {:?}", estado);
            25
        }
    }
}
fn mais_um(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn print_match() {
    let estados = Estado::Alabama;
    let moeda = Moeda::Quarter(estados);
    println!("{:?}", valor_em_cents(moeda));

    let cinco = Some(5);
    let seis = mais_um(cinco);
    let nenhum = mais_um(None);

    println!("{:?} {:?}", seis, nenhum);

    let algum_valor_u8 = 3u8;
    match algum_valor_u8 {
        1 => println!("um"),
        3 => println!("três"),
        5 => println!("cinco"),
        7 => println!("sete"),
        _ => (),
    }

    let algum_valor_u8 = Some(3u8);

    if let Some(3) = algum_valor_u8 {
        println!("três");
    }
}
