
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let point_rock = 1;
    let point_paper = 2;
    let point_scissor = 3;
    
    let point_wins = 6;
    let point_draw = 3; 

    let mut count_points_played = 0;
    let mut points_played = 0;

    
    // ENEMY
    // A ROCK
    // B PAPER 
    // C Scissor 

    // MY
    // X ROCK 
    // Y PAPER 
    // Z SCISSOR
    if let Ok(lines) = read_lines("./input") {
        for line in lines{ 
            if let Ok(line) =  line  {
                // split on whitespace
                let res = Vec::from_iter(line.split(" ").map(String::from));
                let enemy = res[0].clone();
                let my = res[1].clone();

                if my == "X" {
                    count_points_played = count_points_played + point_rock;
                }
                if my == "Y" {
                    count_points_played = count_points_played + point_paper;
                }
                if my == "Z" {
                    count_points_played = count_points_played + point_scissor;
                }

                // draws
                if my == "X" && enemy == "A"{
                    println!("draw");
                    points_played = points_played + point_draw;
                }
                if my == "Y" && enemy == "B"{
                    println!("draw");
                    points_played = points_played + point_draw;
                }
                if my == "Z" && enemy == "C"{
                    println!("draw");
                    points_played = points_played + point_draw;
                }

                // win
                if my == "X" && enemy == "C"{
                    println!("won rock against scissor");
                    points_played = points_played + point_wins;
                }
                if my == "Y" && enemy == "A"{
                    println!("won paper against rock");
                    points_played = points_played + point_wins;
                }
                if my == "Z" && enemy == "B"{
                    println!("won scisscor against paper");
                    points_played = points_played + point_wins;
                }

            }
            
        }
        let total_points = points_played + count_points_played;
        println!("{count_points_played}");
        println!("{total_points}");
    }
}

// helpers 

// read file input by line 
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}