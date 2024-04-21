use std::fs::read_to_string;

fn main() {

    let filename: &str = "input.txt";

    let file_content = read_to_string(filename).unwrap();
    let mut sum = 0;

    for inp in file_content.lines() {
        let mut num_str = String::new();
        num_str.push_str(&ret_first_num_in_string(inp));
        let rev_str = inp.chars().rev().collect::<String>();
        num_str.push_str(&ret_first_num_in_string(&rev_str));

        sum += num_str.parse::<u32>().unwrap();
    }

    println!("Sum: {sum}");
}

fn ret_first_num_in_string(input:&str) -> String{
    let mut s = String::new();
    for c in input.trim().chars() {
        match (c.to_string()).parse::<u32>() {
            Err(_) => continue,
            Ok(_) => {
                s.push(c);
                break;
            },
        }
    }
    s
}
