use std::env;
mod font;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    match &command[..] {
        "display" => match args.len() {
            6 => display(&args[2], &args[3], &args[4], &args[5]),
            n => println!("Incorrect number of arguments for the `display` command - excepted 6, received {}.", n)
        },
        "read" => match args.len() {
            4 => read(&args[2], &args[3]),
            n => println!("Incorrect number of arguments for the `read` command - excepted 4, received {}.", n)
        },
        _ => println!("Unknown command: {}", command)
    };
}

fn display(font_file_path: &String, phrase: &String, foreground: &String, background: &String) {
    let font = match font::parse_file(font_file_path) {
        Ok(val) => val,
        Err(_) => return,
    };

    for i in 0..font.height {
        let line = phrase
            .chars()
            .into_iter()
            .map(|c| font.letters.get(&(c as usize)).unwrap()[i].clone())
            .collect::<Vec<String>>()
            .join("")
            .replace(font.foreground, foreground)
            .replace(font.background, background);

        println!("{}", line);
    }
}

fn read(font_file: &String, ascii_input_file: &String) {
    println!("EXTRA CREDIT FOR READING!");
    println!("{}", font_file);
    println!("{}", ascii_input_file);
}
