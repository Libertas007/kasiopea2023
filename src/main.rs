use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() {
    println!("Zadejte zadání: ");

    let start = Instant::now();
    solve(read_file(&String::from("C.txt")));
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

        let pairs: &str = splitted[line_pointer];
        let pairs: u32 = pairs.parse().unwrap();
        line_pointer += 1;

        let iter = splitted[line_pointer]
            .split(" ")
            .map(|d| d.parse::<i64>().unwrap());

        line_pointer += 1;

        let mut distance: u64 = 0;
        let mut position: usize = 0;
        let mut collected_pairs: u32 = 0;

        let mut cache: HashMap<i64, Vec<usize>> = HashMap::new();
        let mut data: Vec<i64> = vec![];

        let mut j = 0;

        for e in iter {
            if cache.contains_key(&e) {
                cache.get_mut(&e).unwrap_or(&mut vec![]).push(j);
            } else {
                cache.insert(e, vec![j]);
            }
            data.push(e);
            j += 1;
        }

        let start = Instant::now();

        while collected_pairs < pairs {
            // println!("data: {:?}", data);
            let colour = data[position];

            data[position] = -colour;

            //println!("pos: {}, data: {:?}", position, data);
            //println!("colour: {}", colour);
            let next_pos = cache.get(&colour).unwrap()[1];
            /*let next_pos = data[position..]
            .iter()
            .position(|x| x == &colour)
            .unwrap_or(0)
            + position;*/

            distance += ((position as i64) - (next_pos as i64)).abs() as u64;

            //println!("next: {}", next_pos);

            data[next_pos] = -colour;

            let new_position = data[position..]
                .iter()
                .position(|x| x > &0)
                .unwrap_or(next_pos - position)
                + position;

            //println!("new: {}", new_position);

            distance += ((next_pos as i64) - (new_position as i64)).abs() as u64;

            position = new_position;

            collected_pairs += 1;

            if collected_pairs % 10000 == 0 {
                println!(
                    "problem {}, remaining: {}, ETA: {:?}",
                    i,
                    pairs - collected_pairs,
                    (start.elapsed() / collected_pairs) * (pairs - collected_pairs)
                );
            }
            //println!("distance: {}", distance);
        }

        out += &format!("{:?}\n", distance);

        i += 1;
        println!("Vyřešeno {} z {} problémů...", i, problems);
    }
    fs::write("C.out", out).unwrap();
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
