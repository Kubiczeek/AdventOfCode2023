use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    //game id = index + 1
    let scratch_cards: Vec<&str> = contents.lines().map(|x| x.split(": ").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>();
    let mut copied_cards: Vec<i32> = vec![1; scratch_cards.len()];

    for (i, card) in scratch_cards.iter().enumerate() {
        let mut wins = get_wins(card);

        //run as many times as the num in copied_cards[i]
        let multiplier = copied_cards[i];
        let mut j = i+1;
        while wins > 0 {
            copied_cards[j] = copied_cards[j] + 1*multiplier;
            j += 1;
            wins -= 1;
        }

    }
    println!("{:?}", copied_cards.iter().sum::<i32>());
}

fn get_wins(scratch_card: &str) -> i32 {
    let mut wins = 0;
    let card_data = scratch_card.split(" | ").collect::<Vec<&str>>();
    let winning_numbers = card_data[0].split(" ").filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
    let scratch_numbers = card_data[1].split(" ").filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
    for num in scratch_numbers.iter() {
        if winning_numbers.contains(num) {
            wins += 1;
        }
    }
    wins
}

