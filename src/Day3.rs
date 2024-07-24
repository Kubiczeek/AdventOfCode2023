use std::fs;

#[derive(Debug)]
struct NumPos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Num {
    num: i32,
    pos: NumPos,
    length: i32,
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut map: Vec<Vec<&str>> = Vec::new();
    let mut all_nums: Vec<Num> = Vec::<Num>::new();
    let mut valid_nums: Vec<i32> = Vec::<i32>::new();

    for (i, line) in lines.enumerate() {
        let num = find_nums_in_line(line, i as i32);
        let split_line = line.split("").filter(|&x| x != "").collect::<Vec<&str>>();
        map.push(split_line);
        all_nums.extend(num);
    }

    for num in all_nums {
        if is_num_valid(&num, &map) {
            valid_nums.push(num.num);
        }
    }

    println!("Valid nums: {:?}", valid_nums);
    println!("Sum of valid nums: {:?}", valid_nums.iter().sum::<i32>());
}
fn find_nums_in_line(line: &str, line_num: i32) -> Vec<Num> {
    let mut nums: Vec<Num> = Vec::new();
    let mut num_constructor = Vec::new();
    let mut starting_pos = 0;
    for (i, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            num_constructor.push(character.to_string());
            if num_constructor.len() == 1 {
                starting_pos = i as i32;
            }
        } else if num_constructor.len() > 0 {
            let num = num_constructor.join("");
            num_constructor.clear();
            nums.push(Num {
                num: num.parse::<i32>().unwrap(),
                pos: NumPos { x: starting_pos, y: line_num },
                length: num.len() as i32,
            });
        }
    }
    nums
}

fn is_num_valid(num: &Num, map: &[Vec<&str>]) -> bool {
    let bounds_start = NumPos { x: num.pos.x - 1, y: num.pos.y - 1 };
    let bounds_end = NumPos { x: num.pos.x + num.length, y: num.pos.y + 1 };

    for y in bounds_start.y..=bounds_end.y {
        for x in bounds_start.x..=bounds_end.x {
            if y < 0 || x < 0 || y as usize >= map.len() || x as usize >= map[y as usize].len() {
                continue;
            }
            if map[y as usize][x as usize] != "." && !map[y as usize][x as usize].parse::<i32>().is_ok() {
                return true;
            }
        }
    }
    false
}