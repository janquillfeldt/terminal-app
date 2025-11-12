mod windows {
    use std::io::{self, Write, Read};

    pub fn clear_screen() {
        print!("{}[2J", 27 as char);
        print!("{}[1;1H", 27 as char);
        io::stdout().flush().unwrap();
    }

    pub fn read_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn write_output(output: &str) {
        println!("{}", output);
    }
}