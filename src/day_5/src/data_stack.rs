#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct StackBoard {
    map: HashMap<usize, Vec<char>>,
}

impl StackBoard {
    pub fn from_str(input: &str) -> Self {
        let splits: Vec<&str> = input.splitn(2, "\n\n").collect();

        let mut board_data: Vec<&str> = splits[0].lines().collect();
        let (mut map, max) = StackBoard::init_map(board_data.last().unwrap());

        // throw away last data becaused it has been processed in `init_map`
        let _ = board_data.pop();
        StackBoard::insert_map_data(&mut board_data, &mut map, max);

        //println!("{map:?}");

        StackBoard {
            map
        }
    }

    // Get the number of stack needed
    fn init_map(input: &str) -> (HashMap<usize, Vec<char>>, usize) {
        let mut map = HashMap::new();

        //println!("set line {input}");

        // I don't know how to get only last so collecting to a vec it is
        let max = input
            .split(" ")
            .filter_map(|num| num.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        //println!("max {max:?}");

        max.iter().for_each(|num| {
            map.insert(num.clone(), vec![]);
        });

        (map, *max.last().unwrap())
    }

    // Insert data. (to be called after initializing stack)
    fn insert_map_data(input: &mut Vec<&str>, map: &mut HashMap<usize, Vec<char>>, max: usize) {
        // Process per line
        for line in input.iter() {
            // make it easy to access by indices
            let chars = line.chars().collect::<Vec<char>>();

            // insert data by hashmap column
            (0..max).into_iter().for_each(|ikey| {
                // some hashmap and index magic (calculation)
                let key = ikey + 1;
                let index = ikey + (3 * ikey) + 1;  // rUsT Is A MeMOrY sAFe LaNgUAGe

                let data = chars.get(index);
                if let Some(letter) = data {
                    if *letter != ' ' {
                        map.get_mut(&key).unwrap().push(*letter);
                    }
                }
            });
        }
    }
}
