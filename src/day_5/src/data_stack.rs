#![allow(dead_code)]

use std::collections::HashMap;

/// Field in this exact order
/// 0. Number to move
/// 1. From
/// 2. To
#[derive(Debug, Clone)]
pub struct Move(usize, usize, usize);

#[derive(Debug, Clone)]
pub struct StackBoard {
    map: HashMap<usize, Vec<char>>,
    steps: Vec<Move>,
}

impl StackBoard {
    pub fn from_str(input: &str) -> Self {
        let splits: Vec<&str> = input.splitn(2, "\n\n").collect();

        let mut board_data: Vec<&str> = splits[0].lines().collect();
        let (mut map, max) = init_map(board_data.last().unwrap());

        // throw away last data becaused it has been processed in `init_map`
        let _ = board_data.pop();
        insert_map_data(&mut board_data, &mut map, max);

        // reverse all stack
        map.iter_mut().for_each(|stack| stack.1.reverse());

        let steps = init_step(splits[1]);
        //dbg!("{steps}");

        StackBoard { map, steps }
    }

    /// Move one item per move
    pub fn start_move_one(&mut self) {
        let map: &mut HashMap<usize, Vec<char>> = &mut self.map;
        let steps: &Vec<Move> = &self.steps;

        steps.iter().for_each(|step| {
            (0..step.0).for_each(|_| {
                let temp = map.get_mut(&step.1).unwrap().pop().unwrap();
                map.get_mut(&step.2).unwrap().push(temp);
            });
        });
    }

    /// Move stack per move
    pub fn start_move_stack(&mut self) {
        let map: &mut HashMap<usize, Vec<char>> = &mut self.map;
        let steps: &Vec<Move> = &self.steps;

        // rUsT Is A MeMOrY sAFe LaNgUAGe
        steps.iter().for_each(|step| {
            let from = map.get_mut(&step.1).unwrap();
            let index_from = from.len() - step.0;
            let mut moved_temp = from.drain(index_from..).collect::<Vec<char>>();
            map.get_mut(&step.2).unwrap().append(&mut moved_temp);
        });
    }

    pub fn print_top(&self) {
        self.map.iter().for_each(|stack| {
            println!("Key:{} Top:{}", stack.0, stack.1.last().unwrap());
        });
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
            let index = ikey + (3 * ikey) + 1; // rUsT Is A MeMOrY sAFe LaNgUAGe

            let data = chars.get(index);
            if let Some(letter) = data {
                if *letter != ' ' {
                    map.get_mut(&key).unwrap().push(*letter);
                }
            }
        });
    }
}

fn init_step(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut token = line.split(' ');

            // ignore 'move'
            let _ = token.next();
            let one = token.next().unwrap().parse::<usize>().unwrap();

            // ignore 'from'
            let _ = token.next();
            let two = token.next().unwrap().parse::<usize>().unwrap();

            // ignore 'to'
            let _ = token.next();
            let three = token.next().unwrap().parse::<usize>().unwrap();

            Move(one, two, three)
        })
        .collect::<Vec<Move>>()
}
