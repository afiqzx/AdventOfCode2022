static DATA: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

fn main() {
    // Get data
    let mut data_path: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    data_path.push_str("/data/day_3/data");
    let file: String = String::from_utf8(std::fs::read(data_path).unwrap()).unwrap();
    //let file: String = DATA.to_string();

    let priorities = to_vec_half(&file);
    let priorities_3_lines = to_vec_3_lines(&file);

    // round 1
    println!("{}", priorities.iter().sum::<i32>());

    // round 2
    println!("{}", priorities_3_lines.iter().sum::<i32>());
}

fn to_vec_half(data: &str) -> Vec<i32> {
    data.lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let mut set = std::collections::HashSet::new();
            let _: () = left
                .chars()
                .map(|letter| {
                    set.insert(letter);
                })
                .collect();

            for letter in right.chars() {
                if set.contains(&letter) {
                    return get_priority(&letter);
                }
            }

            0
        })
        .collect()
}

fn to_vec_3_lines(data: &str) -> Vec<i32> {
    let vec = data.split("\n").into_iter().collect::<Vec<&str>>();

    // USE CHUNKS NOT WINDOWS
    vec.chunks(3)
        .map(|window| {
            //let (one, two, three) = (line[0], line[1], line[2]);
            //println!("A set start");
            //println!("{:?}", window);
            //println!("A set end");

            // Hashset for each first and second line
            let mut vec_set = vec![
                std::collections::HashSet::new(),
                std::collections::HashSet::new(),
            ];

            let mut priority = 0;

            for (index, line) in window.into_iter().enumerate() {
                let mut chars = line.chars();

                // first 2 lines
                if index < 2 {
                    let _ = chars
                        .map(|letter| vec_set[index].insert(letter))
                        .collect::<Vec<_>>();
                }
                // other final line
                else {
                    loop {
                        let letter_opt = chars.next();

                        if let Some(letter) = letter_opt {
                            if vec_set[0].contains(&letter) && vec_set[1].contains(&letter) {
                                //println!("Get same: {letter}");
                                priority = get_priority(&letter);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }

            priority
        })
        .collect()
}

fn get_priority(letter: &char) -> i32 {
    match letter {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}
