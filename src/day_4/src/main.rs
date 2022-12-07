use std::ops::RangeInclusive;

#[allow(dead_code)]
static DATA: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

fn main() {
    // Get data
    let mut data_path: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    data_path.push_str("/data/day_4/data");
    let file: String = String::from_utf8(std::fs::read(data_path).unwrap()).unwrap();
    //let file: String = DATA.to_string();

    let (lhs, rhs) = to_ranges(&file);

    // round 1
    println!(
        "{}",
        is_fully_contains_vec(&lhs, &rhs)
            .iter()
            .filter(|x| **x)
            .map(|_x| 1) // too lazy
            .sum::<usize>()
    );

    // round 2
    println!(
        "{}",
        is_overlap_vec(&lhs, &rhs)
            .iter()
            .filter(|x| **x)
            .map(|_x| 1) // too lazy
            .sum::<usize>()
    );
}

fn to_ranges(data: &str) -> (Vec<RangeInclusive<usize>>, Vec<RangeInclusive<usize>>) {
    let mut lhs = vec![];
    let mut rhs = vec![];

    data.lines().for_each(|line| {
        let mut splits = line.split(",");

        let to_range = |range: &str| {
            let mut r = range.split("-").map(|num| num.parse::<usize>().unwrap());

            r.next().unwrap()..=r.next().unwrap()
        };

        lhs.push(to_range(splits.next().unwrap()));
        rhs.push(to_range(splits.next().unwrap()));
    });

    (lhs, rhs)
}

fn is_fully_contains_vec(
    lhs: &Vec<RangeInclusive<usize>>,
    rhs: &Vec<RangeInclusive<usize>>,
) -> Vec<bool> {
    lhs.iter()
        .zip(rhs)
        .map(|(lhs, rhs)| {
            (lhs.contains(rhs.start()) && lhs.contains(rhs.end()))
                || (rhs.contains(lhs.start()) && rhs.contains(lhs.end()))
        })
        .collect::<Vec<bool>>()
}

fn is_overlap_vec(lhs: &Vec<RangeInclusive<usize>>, rhs: &Vec<RangeInclusive<usize>>) -> Vec<bool> {
    lhs.iter()
        .zip(rhs)
        .map(|(lhs, rhs)| {
            (lhs.contains(rhs.start()) || lhs.contains(rhs.end()))
                || (rhs.contains(lhs.start()) || rhs.contains(lhs.end()))
        })
        .collect::<Vec<bool>>()
}
