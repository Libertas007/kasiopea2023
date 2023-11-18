use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() {
    println!("Zadejte zadání: ");

    let start = Instant::now();
    solve(read_file(&String::from("B.txt")));
    println!("Vyřešeno za {:?}", start.elapsed());
}

pub fn is_between(low: isize, mid: isize, high: isize) -> bool {
    low <= mid && mid <= high
}

pub fn solve(input: String) {
    let splitted: Vec<&str> = input.split("\n").collect();

    let problems = splitted.first().unwrap();
    println!("{}", problems);
    let problems: i32 = problems.parse().unwrap();
    let mut i = 0;
    let mut line_pointer = 1;
    let mut out = String::new();

    while problems != i {
        println!("Řeším {}. z {} problémů...", i + 1, problems);

        line_pointer += 1;

        let data: Vec<i32> = splitted[line_pointer]
            .split(" ")
            .map(|d| d.parse::<i32>().unwrap())
            .collect();

        line_pointer += 1;

        let mut j = 0;
        let mut records = 0;

        //println!("{:?}", data);

        let mut previous_max = 0;

        while data.len() > j {
            if j != 0 {
                if data[j - 1] > previous_max {
                    previous_max = data[j - 1];
                }
            }

            //println!("{}", previous);
            //println!("{}", records);

            if data[j] > previous_max {
                records += 1;
            }

            j += 1;
        }

        out += &format!("{:?}\n", records);

        i += 1;
        println!("Vyřešeno {} z {} problémů...", i, problems);
    }
    fs::write("B.out", out).unwrap();
}

pub fn read_line() -> String {
    let mut val = String::new();
    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line");
    val
}

pub fn read_file(file: &String) -> String {
    fs::read_to_string(file).expect("Failed to read the file")
}

pub fn write_file(file: &String, content: &String) {
    fs::write(file, content).unwrap();
}
