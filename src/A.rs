use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() {
    println!("Zadejte zadání: ");

    let start = Instant::now();
    solve(read_file(&String::from("A.txt")));
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

        let s: &str = splitted[line_pointer];
        let s: i32 = s.parse().unwrap();
        line_pointer += 1;

        let d: &str = splitted[line_pointer];
        let d: i32 = d.parse().unwrap();
        line_pointer += 1;

        let p: &str = splitted[line_pointer];
        let p: i32 = p.parse().unwrap();
        line_pointer += 1;

        let z: &str = splitted[line_pointer];
        let z: i32 = z.parse().unwrap();
        line_pointer += 1;

        let cesta1: i32 = p + z + d;
        let cesta2: i32 = (d * 2 + s * 2) - cesta1;

        if cesta1 < cesta2 {
            out += &format!("{:?}\n", cesta1);
        } else {
            out += &format!("{:?}\n", cesta2)
        }
        i += 1;
        println!("Vyřešeno {} z {} problémů...", i, problems);
    }
    fs::write("A.out", out).unwrap();
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
