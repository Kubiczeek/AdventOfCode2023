use std::fs;

fn main() {
    let r_cubes = 12;
    let g_cubes = 13;
    let b_cubes = 14;
    let mut min_cubes = Vec::new();
    let mut good_ids = Vec::new();
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let game = line.split(": ").collect::<Vec<&str>>();
        let game_id = game[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let game_data = game[1].split("; ").collect::<Vec<&str>>();
        let mut few_r_cubes = 0;
        let mut few_g_cubes = 0;
        let mut few_b_cubes = 0;
        let mut valid = true;
        for round in game_data {
            let round_data = round.split(", ").collect::<Vec<&str>>();

            for cube in round_data {
                let cube_data = cube.split(" ").collect::<Vec<&str>>();
                let amount = cube_data[0].parse::<i32>().unwrap();
                let color = cube_data[1];
                match color {
                    "red" => {
                        if amount > r_cubes {
                            valid = false;
                        }
                        if amount > few_r_cubes {
                            few_r_cubes = amount;
                        }
                    }
                    "green" => {
                        if amount > g_cubes {
                            valid = false;
                        }
                        if amount > few_g_cubes {
                            few_g_cubes = amount;
                        }
                    }
                    "blue" => {
                        if amount > b_cubes {
                            valid = false;
                        }
                        if amount > few_b_cubes {
                            few_b_cubes = amount;
                        }
                    }
                    _ => {
                        println!("Invalid color");
                    }
                }
            }
        }
        if valid {
            good_ids.push(game_id);
        }
        min_cubes.push(few_r_cubes*few_g_cubes*few_b_cubes);
    }

    println!("{:?}", good_ids.iter().sum::<i32>());
    println!("{:?}", min_cubes.iter().sum::<i32>());
}
