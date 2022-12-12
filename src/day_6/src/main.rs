use std::collections::HashSet;

#[allow(dead_code)]
static DATA: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

fn main() {
    // Get data
    let mut data_path: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    data_path.push_str("/data/day_6/data");
    let file: String = String::from_utf8(std::fs::read(data_path).unwrap()).unwrap();
    //let file: String = DATA.to_string();


    let index1 = get_first_packet_marker(&file, 4).unwrap();
    let index2 = get_first_packet_marker(&file, 14).unwrap();

    // round 1
    println!("{index1}");

    // round 2
    println!("{index2}");
}

fn get_first_packet_marker(input: &str, window_size: usize) -> Option<usize> {

    let chars: Vec<_> = input.chars().collect();

    for (o_index, window) in chars.windows(window_size).enumerate() {
        if o_index == 0 {
            continue;
        }

        let mut set: HashSet<char> = HashSet::new();

        for (index, letter) in window.iter().enumerate() {
            //println!("{}", letter);
            let result = set.insert(*letter);
            // if one char already inserted it's not our mark
            if !result {
                break;
            }
            // if all are unique, then it's the answer
            else if index + 1 == window_size && result {
                return Some(index + o_index + 1);
            }
        }

    }

    None
}


