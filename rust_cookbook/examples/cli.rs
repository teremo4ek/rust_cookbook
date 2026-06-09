use ansi_term::Colour;
use ansi_term::Style;
use clap::{Arg, Command};

fn main() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );

    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );

    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );

    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .author("YB")
        .about("Command line argument parsing")
        // файл
        .arg(
            Arg::new("file")
                // короткий флаг -f
                .short('f')
                // длинный флаг --file
                .long("file")
                .help("Файл"),
        )
        // число
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .help("Ваше любимое число"),
        )
        .get_matches();

    let myfile = matches.get_one::<String>("file").unwrap();
    println!("Файл: {}", myfile);

    let num_str = matches.get_one::<String>("num");
    match num_str {
        None => println!("Ваше любимое число неизвестно"),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Ваше любимое число: {}", n),
            Err(_) => println!("Это не число: {}", s),
        },
    }
}
