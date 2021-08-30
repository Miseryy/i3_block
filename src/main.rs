use std::process::Command;
use std::env;

fn main() {
    // Filesystem Size Used Avail Use% Mounted on
    let args: Vec<String> = env::args().collect();
    let system = &args[1];
    let command = "df -H -t ".to_string() + system;
    let mut spl: Vec<&str> = command.split(" ").collect();

    let output = Command::new(spl[0])
        .args(&spl[1..])
        .output()
        .expect("fail");

    let out = output.stdout;
    let out_str = String::from_utf8_lossy(&out);
    spl = out_str.split("\n").collect();

    spl.remove(0);
    spl.remove(spl.len() - 1);
    let mut string_data: Vec<String> = Vec::new();

    for disk_str in spl {
        let mut ss : String = String::from("");
        let mut before:char = 'a';

        for s in disk_str.chars() {
            if s != ' ' {
                if before == ' ' {
                    ss.push(' ');

                }
                ss.push(s);
            }

            before = s;

        }
        string_data.push(ss);
    }

    println!("{:?}", string_data);

}
