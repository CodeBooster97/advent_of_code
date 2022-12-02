
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    
    let point_rock = 1;
    let point_paper = 2;
    let point_scissor = 3;
    
    let point_wins = 6;
    let point_draw = 3; 

    let mut points_played = 0;
    let mut points_hand = 0;

    
    // ENEMY
    // A ROCK
    // B PAPER 
    // C Scissor 

    // MY
    // X LOOSE 
    // Y DRAW 
    // Z WIN
    if let Ok(lines) = read_lines("./input") {
        for line in lines{ 
            if let Ok(line) =  line  {
                // split on whitespace
                let res = Vec::from_iter(line.split(" ").map(String::from));
                let enemy = res[0].clone();
                let my = res[1].clone();

                if my == "X"{
                    println!("LOOSE :(");
                    if enemy == "A"{
                        points_hand = points_hand + point_scissor;
                    }
                    if enemy == "B"{
                        points_hand = points_hand + point_rock;
                    }
                    if enemy == "C"{
                        points_hand = points_hand + point_paper;
                    }
                }
                if my == "Y"{
                    println!("draw");
                    points_played = points_played + point_draw;
                    if enemy == "A"{
                        points_hand = points_hand + point_rock;
                    }
                    if enemy == "B"{
                        points_hand = points_hand + point_paper;
                    }
                    if enemy == "C"{
                        points_hand = points_hand + point_scissor;
                    }
                }
                if my == "Z"{
                    println!("WIN! yuhu");
                    points_played = points_played + point_wins;
                    if enemy == "A"{
                        points_hand = points_hand + point_paper;
                    }
                    if enemy == "B"{
                        points_hand = points_hand + point_scissor;
                    }
                    if enemy == "C"{
                        points_hand = points_hand + point_rock;
                    }
                }
            }
            
        }
    println!("{points_hand}");
    println!("{points_played}");
    let total = points_hand + points_played;
    println!("{total}");
    }
}

// helpers 

// read file input by line 
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// 17181