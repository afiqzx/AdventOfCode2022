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
    // Get data
    let mut data_path: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    data_path.push_str("/data/day_1/data1");
    let file: String = String::from_utf8(std::fs::read(data_path).unwrap()).unwrap();

    // Mut to enable sorting
    let mut data = parse_data_to_vec(&file);

    let max = find_highest(&mut data);
    
    let (first, second, third) = find_top_three(&mut data);

    println!("{max}");

    println!("{}", first + second + third);
}

fn parse_data_to_vec(input: &str) -> Vec<i32> {
    // Some split magic
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

fn find_highest(input: &mut Vec<i32>) -> i32 {
    input.sort();

    input[input.len() - 1]
}


fn find_top_three(input: &mut Vec<i32>) -> (i32, i32, i32) {
    // YES I KNOW WE'LL SORT TWO TIME. JUST IN CASE
    input.sort();
    
    let length = input.len();

    (input[length -1], input[length - 2], input[length - 3])
}
