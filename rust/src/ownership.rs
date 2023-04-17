fn print_ownership() {
    let s = String::from("hello world");
    toma_posse(s);
    // println!("{s}"); Não valido pois o valor foi movido para a função
    let x = 5;
    faz_uma_copia(x);
    println!("{x}");
    let s1 = entrega_valor();
    let s2 = String::from("texto");
    let s3 = pega_e_entrega_valor(s2);
    println!("{s1}");
    println!("{s3}");
    let s1 = String::from("texto");
    let (s2, tamanho) = calcula_tamanho(s1);
    println!("O tamanho de '{}' é {}.", s2, tamanho);
}

fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len(); // len() retorna o tamanho de uma String.
    (s, tamanho)
}
fn toma_posse(s: String) {
    println!("{s}");
}

fn faz_uma_copia(x: i32) {
    println!("{x}");
}

fn entrega_valor() -> String {
    let uma_string = String::from("olá");
    uma_string
}

fn pega_e_entrega_valor(uma_string: String) -> String {
    uma_string
}
