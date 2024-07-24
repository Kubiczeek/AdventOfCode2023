use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let scratch_card = contents.lines().map(|line| line.split(": ").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>();
    let mut wins = Vec::new();

    for card in scratch_card {
        let split_card = card.split(" | ").collect::<Vec<&str>>();
        let winning_numbers = split_card[0].split(" ").filter(|&i| i != "").collect::<Vec<&str>>();
        let scratch_numbers = split_card[1].split(" ").filter(|&i| i != "").collect::<Vec<&str>>();
        let mut win_count = 0;
        dbg!(card);
        for scratch_num in scratch_numbers {
            if winning_numbers.contains(&scratch_num) {
                dbg!(scratch_num);
                if win_count == 0 {
                    win_count += 1;
                } else {
                    win_count = win_count * 2;
                }
            }
        }
        wins.push(win_count);
    }

    println!("{:?}", wins);
    println!("{:?}", wins.iter().sum::<i32>());
}