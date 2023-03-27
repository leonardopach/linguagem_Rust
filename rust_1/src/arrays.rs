pub fn arrays() {
    let error_codes: [i32; 3] = [200, 404, 500];
    let not_found = error_codes[1];

    println!("{}, {:?}", not_found, error_codes);

    let byts = [0; 8];

    println!("{:?}", byts);
}
