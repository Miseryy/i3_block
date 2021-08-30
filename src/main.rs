use std::process::Command;

fn main() {
    let command = "df -H -t ext4";
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
        let mut count: u8 = 0;
        let mut before:char = 'a';

        for s in disk_str.chars() {
            if s != ' ' {
                if before == ' ' {
                    ss.push(' ');

                }

                ss.push(s);

            }

            count += 1;
            before = s;

        }

        string_data.push(ss);
    }

    println!("{:?}", string_data);

}
