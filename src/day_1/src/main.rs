static DATA: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

fn main() {
    //let mut elfs: Vec<i32> = vec![];


    //let workspace_root =     
    let mut data_path: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    data_path.push_str("/data/day_1/data1");
    let file: String = String::from_utf8(std::fs::read(data_path).unwrap()).unwrap();

    let mut data = normalize_data(&file);

    let max = find_max(&mut data);
    
    let (first, second, third) = find_top_three(&mut data);

    println!("{max}");

    println!("{}", first + second + third);
}


fn normalize_data(input: &str) -> Vec<i32> {
    let elf_entry: Vec<&str> = input.split("\n\n").into_iter().collect();

    elf_entry 
        .iter()
        .map(|elf| {
            elf.lines()
                .filter_map(|line| line.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>()
}

fn find_max(input: &mut Vec<i32>) -> i32 {
    input.sort();

    input[input.len() - 1]
}


fn find_top_three(input: &mut Vec<i32>) -> (i32, i32, i32) {
    input.sort();
    
    let length = input.len();

    (input[length -1], input[length - 2], input[length - 3])
}
