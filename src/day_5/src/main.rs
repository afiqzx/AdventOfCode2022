mod data_stack;

#[allow(dead_code)]
static DATA: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

fn main() {
    // Get data
    let mut data_path: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    data_path.push_str("/data/day_5/data");
    let file: String = String::from_utf8(std::fs::read(data_path).unwrap()).unwrap();
    //let file: String = DATA.to_string();

    let board = data_stack::StackBoard::from_str(&file);

    println!("{board:?}");

    // round 1

    // round 2
}

