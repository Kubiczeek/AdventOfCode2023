use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let line_map = contents.lines().map(|line| {
        let mut part_line = String::new();
        for character in line.chars() {
            part_line.push(character);
            part_line = part_line.replace("one", "1").replace("two", "2").replace("three", "3")
                .replace("four", "4").replace("five", "5").replace("six", "6")
                .replace("seven", "7").replace("eight", "8").replace("nine", "9");
        }
        part_line
    }).collect::<Vec<String>>();

    let sum: i32 = line_map.iter().map(|line| {
        let digits: Vec<i32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect();
        format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<i32>().unwrap()
    })
        .sum();

    println!("{:?}", sum);
}
