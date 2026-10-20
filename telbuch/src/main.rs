use std::io;
use std::io::Write;
use std::fs;
use std::sync::Mutex;
use std::fs::OpenOptions;


// von input_loop.rs entnommen
#[derive(Debug)]
struct Cmd {
    args: Vec<String>,
}

// von input_loop.rs entnommen
fn input(prompt: &[u8]) -> Cmd {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut cmd = String::new();
    loop {
        let e = handle.write_all(prompt);
        match e {
            Ok(_) => (),
            Err(err) => panic!("error {:?}", err.kind()),
        }
        let e = handle.flush();
        match e {
            Ok(_) => (),
            Err(err) => panic!("error {:?}", err.kind()),
        }
        cmd.clear();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        let v: Vec<&str> = cmd.trim().split(' ').collect();
        match v[0] {
            "" => continue,
            _ => {
                let mut v2: Vec<String> = vec![];
                for e in v {
                    v2.push(e.to_string());
                }
                return Cmd { args: v2 };
            }
        }
    }
}

static ENTRIES: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

fn main() {
    
    load();
    println!("Enter 'h' or 'help' for usage.");

    let mut c: Cmd;
    loop {
        c = input(b"Prompt> ");
        if c.args[0] == "." {
            break;
        }
        else if c.args[0] == "h" || c.args[0] == "help" {
            help();
        }
        else if c.args[0] == "!" {
            let mut entries = ENTRIES.lock().unwrap();
            entries.push((c.args[1].clone(), c.args[2].clone()));
            save(&c.args[1].clone(), &c.args[2].clone());
        }
        else if c.args[0] == "?" {
            let entries = ENTRIES.lock().unwrap();
            let name = &c.args[1];
            for entry in entries.iter() {
                if entry.0 == *name {
                    let number = entry.1.clone();
                    println!("number for {:?} is: {:?}", name, number);
                }
            }
        }
    }
}

fn help() {
    println!("usage:
                    ! name number   # make new entry
                    ? name          # ask for number
                    .               # quit");
}

fn save(name: &str, number: &str) {
    let mut file = OpenOptions::new().append(true).open("../numbers.txt").expect("could not open file.");
    let _ = write!(file, "\n{} {}", name, number);
}

fn load() {
    let mut entries = ENTRIES.lock().unwrap();
    let file = "../numbers.txt";
    let content = fs::read_to_string(file).expect("could not read file.");
    for line in content.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        entries.push((parts[0].to_string(), parts[1].to_string()));
    }
}