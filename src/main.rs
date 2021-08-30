use std::process::Command;
use std::env;

fn get_df_data(command:String) -> String {
    let spl: Vec<&str> = command.split(" ").collect();

    let output = Command::new(spl[0])
        .args(&spl[1..])
        .output()
        .expect("fail");

    let out = output.stdout;
    let out_str = String::from_utf8_lossy(&out);

    String::from(out_str)

}

fn str_molding(mut spl: Vec<&str>) -> Vec<String> {
    spl.remove(0);
    spl.remove(spl.len() - 1);
    let mut string_vec: Vec<String> = Vec::new();

    for disk_str in spl {
        let mut ss : String = String::from("");
        let mut before:char = '\0';

        for s in disk_str.chars() {
            if s != ' ' {
                if before == ' ' {
                    ss.push(' ');

                }
                ss.push(s);
            }

            before = s;

        }
        string_vec.push(ss);
    }

    string_vec

}

fn main() {
    // Filesystem Size Used Avail Use% Mounted on
    let args: Vec<String> = env::args().collect();
    let system = &args[1];
    let command = "df -H -t ".to_string() + system;

    let out_str = get_df_data(command);
    let spl: Vec<&str> =  out_str.split("\n").collect();

    let string_vec = str_molding(spl);

    println!("{:?}", string_vec);

}
