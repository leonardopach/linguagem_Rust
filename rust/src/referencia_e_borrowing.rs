fn referencia_print() {
    let s1 = String::from("hello world");

    let tamanho = calcula_tamanho(&s1);

    println!("O tamanho de {s1} é {tamanho}");
    let mut s = String::from("texto");

    modifica(&mut s);

    let mut s = String::from("texto");

    {
        let r1 = &mut s;
        println!("{r1}");
    } // aqui r1 sai de escopo, então já podemos criar uma nova referência sem
      // problema nenhum.

    let r2 = &mut s;

    println!("{r2}");
}
fn modifica(uma_string: &mut String) {
    uma_string.push_str(" longo");
    println!("{uma_string}");
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
}
