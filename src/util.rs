#[macro_export]
macro_rules! readln {
    () => {
        {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            String::from(input)
        }
    }
}

pub fn separator(symbol: char, length: u32) {
    let mut result = String::new();
    for _ in 0..length {
        result.push(symbol);
    };
    println!("{}", result);
}