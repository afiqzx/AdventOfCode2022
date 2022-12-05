static DATA: &str = r#"A Y
B X
C Z
"#;

fn main() {
    // Get data
    let mut data_path: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    data_path.push_str("/data/day_2/data");
    let file: String = String::from_utf8(std::fs::read(data_path).unwrap()).unwrap();

    let (lhs, rhs) = to_vec(&file);

    // round 1
    println!("{}", get_score_list(&lhs, &rhs).iter().sum::<i32>());

    // round 2
    println!("{}", get_pretend_score_list(&lhs, &rhs).iter().sum::<i32>());
}

fn to_vec(file_data: &str) -> (Vec<Hand>, Vec<Hand>) {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();

    let _ = file_data
        .lines()
        .map(|line| {
            let mut splitted = line.split(" ");

            lhs.push(Hand::from(splitted.next().unwrap()));
            rhs.push(Hand::from(splitted.next().unwrap()));
        })
        .collect::<()>();

    (lhs, rhs)
}

fn get_score_list(lhs: &Vec<Hand>, rhs: &Vec<Hand>) -> Vec<i32> {
    lhs.iter().zip(rhs)
        .map(|(lhs,rhs)| {
            lhs.challenge(rhs)
        })
        .collect()
}

fn get_pretend_score_list(lhs: &Vec<Hand>, rhs: &Vec<Hand>) -> Vec<i32> {
    lhs.iter().zip(rhs)
        .map(|(lhs,rhs)| {
            lhs.pretend_strats(rhs)
        })
        .collect()
}

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    // round 1
    pub fn challenge(&self, rhs: &Hand) -> i32 {
        if self == rhs {
            3 + i32::from(rhs)
        }
        else {
            if self.isWin(rhs) {
                6 + i32::from(rhs)
            }
            else {
                0 + i32::from(rhs)

            }
        }
    }

    // round 2
    pub fn pretend_strats(&self, rhs: &Hand) -> i32 {
        match rhs {
            // lose
            Hand::Rock => {
                let losing_hand = {
                    if !self.isWin(&Hand::Rock) && self != &Hand::Rock {
                        Hand::Rock
                    }
                    else if !self.isWin(&Hand::Paper) && self != &Hand::Paper{
                        Hand::Paper
                    }
                    else {
                        Hand::Scissor
                    }
                };

                0 + i32::from(&losing_hand)
            },
            // draw
            Hand::Paper => {
                3 + i32::from(self)
            },
            // win
            Hand::Scissor => {
                let winning_hand = {
                    if self.isWin(&Hand::Rock) {
                        Hand::Rock
                    }
                    else if self.isWin(&Hand::Paper) {
                        Hand::Paper
                    }
                    else {
                        Hand::Scissor
                    }
                };

                6 + i32::from(&winning_hand)
            },
        }
    }

    fn isWin(&self, rhs: &Hand) -> bool {
        match self {
            Hand::Paper => {
                if *rhs == Hand::Scissor {
                    true
                }
                else {
                    false
                }
            },
            Hand::Scissor => {
                if *rhs == Hand::Rock {
                    true
                }
                else {
                    false
                }
            },
            Hand::Rock => {
                if *rhs == Hand::Paper {
                    true
                }
                else {
                    false
                }
            },
        }
    }
}


impl From<&str> for Hand {
    fn from(input: &str) -> Hand {
        match input {
            "A" => Hand::Rock,
            "X" => Hand::Rock,
            "B" => Hand::Paper,
            "Y" => Hand::Paper,
            "C" => Hand::Scissor,
            "Z" => Hand::Scissor,
            _ => unreachable!(),
        }
    }
}

// helper to get what every hand worth
impl From<&Hand> for i32 {
    fn from(input: &Hand) -> i32 {
        match input {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }
}

