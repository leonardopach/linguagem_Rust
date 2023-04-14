fn main() {
    let numero = 7;

    if numero < 5 {
        println!("Numero e menor que 5")
    } else {
        println!("Numero não e menor que 5")
    }

    if numero % 4 == 0 {
        println!("numero is divisible by 4");
    } else if numero % 3 == 0 {
        println!("numero is divisible by 3");
    } else if numero % 2 == 0 {
        println!("numero is divisible by 2");
    } else {
        println!("numero is not divisible by 4, 3, or 2");
    }
}
