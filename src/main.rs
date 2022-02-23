#![deny(clippy::all, clippy::pedantic)]

#[derive(Debug)]
enum Mode {
    Print,
    Pattern(String),
    Replacement(String, String),
}

fn run(input: &str) -> Result<(), ()> {
    let mut program = input.chars().rev().collect::<String>();
    let mut mode = Mode::Print;

    while !program.is_empty() {
        match mode {
            Mode::Print => match program.pop().ok_or(())? {
                '/' => mode = Mode::Pattern(String::new()),
                '\\' => print!("{}", program.pop().ok_or(())?),
                c => print!("{}", c),
            },
            Mode::Pattern(p) => match program.pop().ok_or(())? {
                '/' => mode = Mode::Replacement(p, String::new()),
                '\\' => {
                    let mut p = p;
                    p.push(program.pop().ok_or(())?);
                    mode = Mode::Pattern(p);
                }
                c => {
                    let mut p = p;
                    p.push(c);
                    mode = Mode::Pattern(p);
                }
            },
            Mode::Replacement(p, r) => match program.pop().ok_or(())? {
                '/' => {
                    while program.contains(&p.chars().rev().collect::<String>()) {
                        program = program.replacen(
                            &p.chars().rev().collect::<String>(),
                            &r.chars().rev().collect::<String>(),
                            1,
                        );
                    }
                    mode = Mode::Print;
                }
                '\\' => {
                    let mut r = r;
                    r.push(program.pop().ok_or(())?);
                    mode = Mode::Replacement(p, r);
                }
                c => {
                    let mut r = r;
                    r.push(c);
                    mode = Mode::Replacement(p, r);
                }
            },
        }
    }

    println!();
    Ok(())
}

fn main() {
    if let Some(p) = std::env::args().nth(1) {
        match std::fs::read_to_string(&p) {
            Ok(s) => {
                if let Err(()) = run(&s) {
                    eprintln!("Error in program");
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Error reading file {}", p);
                std::process::exit(e.raw_os_error().unwrap_or(1));
            }
        }
    } else {
        eprintln!("Error: Provide a program file: slashes [file]");
        std::process::exit(1)
    }
}
