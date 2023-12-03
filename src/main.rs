use std::fs;
use substring::Substring;

fn main() {
    println!("Day One Part One Solution: {:?}", day_one_part_one());
    println!("Day One Part Two Solution: {:?}", day_one_part_two());
    println!("Day Two Part One Solution: {:?}", day_two_part_one().iter().sum::<u32>());
    println!("Day Two Part Two Solution: {:?}", day_two_part_two().iter().sum::<u32>());
}

fn day_one_part_one() -> i32 {
    let data = fs::read_to_string("C:\\Users\\oscar\\Documents\\repo\\Rust\\advent-of-code\\src\\input_d1.txt").expect("error reading file");
    let data_lines = data.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;

    for line in data_lines {
        let mut first: i32 = -1;
        let mut last: i32 = -1;
        // println!("Line: {}", line);
        for character in line.chars() {
            match character.to_digit(10) {
                None => {}
                Some(n) => {
                    if first == -1 {
                        first = n as i32;
                        last = n as i32;
                    } else {
                        last = n as i32;
                    }
                }
            }
        }
        let line_number = (first * 10) + last;
        // println!("Line sum: {}", line_number);
        sum += line_number;
    }
    // println!("Total sum: {}", sum);
    return sum;
}

fn day_one_part_two() -> i32 {
    let data = fs::read_to_string("C:\\Users\\oscar\\Documents\\repo\\Rust\\advent-of-code\\src\\input_d1.txt").expect("error reading file");
    let data_lines = data.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;

    for line in data_lines {
        let mut first: i32 = -1;
        let mut last: i32 = -1;

        let char_list: Vec<char> = line.chars().collect();
        let mut n = -1;

        // println!("Line: {}", line);

        for i in 0..char_list.len() {
            match char_list[i] {
                'o' => if i+2 < char_list.len() && line.substring(i, i+3) == "one" { n = 1 }
                't' => if i+2 < char_list.len() && char_list[i+1] == 'w' && line.substring(i, i+3) == "two" { n = 2 } else if i+4 < char_list.len() && line.substring(i, i+5) == "three" { n = 3 }
                'f' => if i+3 < char_list.len() && char_list[i+1] == 'o' && line.substring(i, i+4) == "four" { n = 4 } else if i+3 < char_list.len() && line.substring(i, i+4) == "five" { n = 5 }
                's' => if i+2 < char_list.len() && char_list[i+1] == 'i' && line.substring(i, i+3) == "six" { n = 6 } else if i+4 < char_list.len() && line.substring(i, i+5) == "seven" { n = 7 }
                'e' => if i+4 < char_list.len() && line.substring(i, i+5) == "eight" { n = 8 }
                'n' => if i+3 < char_list.len() && line.substring(i, i+4) == "nine" { n = 9 }
                _ => match char_list[i].to_digit(10) {
                    None => { n = -1}
                    Some(v) => { n = v as i32 }
                }
            }

            if n != -1 {
                if first == -1 {
                    first = n;
                    last = n;
                } else {
                    last = n;
                }
            }
        }
        sum += (first * 10) + last;
        // println!("Number: {}", (first * 10) + last);
    }

    return sum;
}

fn day_two_part_one() -> Vec<u32> {
    let data = fs::read_to_string("C:\\Users\\oscar\\Documents\\repo\\Rust\\advent-of-code\\src\\input_d2.txt").expect("error reading file");
    let data_lines = data.split("\n").collect::<Vec<&str>>();
    let mut games: Vec<u32> = vec![];
    let mut id = 0;
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    for line in data_lines {
        id += 1;
        games.push(id);
        let sets = line.substring(line.find(':').expect("Invalid game line") + 2, line.len()).split(';').collect::<Vec<&str>>();

        for set in sets {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let cubes = set.split(',').collect::<Vec<&str>>();
            for cube in cubes {
                let cube_colors = cube.trim().split(' ').collect::<Vec<&str>>();
                let number = cube_colors[0].parse::<i32>().unwrap();
                let cube_chars: Vec<char> = cube_colors[1].trim().chars().collect();
                match cube_chars[0] {
                    'r' => { red = number }
                    'g' => { green = number }
                    'b' => { blue = number }
                    _ => {  }
                }
            }
            if red > max_red || green > max_green || blue > max_blue {
                games.pop();
                break;
            }
        }
    }

    return games;
}

fn day_two_part_two() -> Vec<u32> {
    let data = fs::read_to_string("C:\\Users\\oscar\\Documents\\repo\\Rust\\advent-of-code\\src\\input_d2.txt").expect("error reading file");
    let data_lines = data.split("\n").collect::<Vec<&str>>();
    let mut games: Vec<u32> = vec![];


    for line in data_lines {
        let sets = line.substring(line.find(':').expect("Invalid game line") + 2, line.len()).split(';').collect::<Vec<&str>>();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in sets {
            let cubes = set.split(',').collect::<Vec<&str>>();
            for cube in cubes {
                let cube_colors = cube.trim().split(' ').collect::<Vec<&str>>();
                let number = cube_colors[0].parse::<i32>().unwrap();
                let cube_chars: Vec<char> = cube_colors[1].trim().chars().collect();
                match cube_chars[0] {
                    'r' => if number > red { red = number }
                    'g' => if number > green { green = number }
                    'b' => if number > blue { blue = number }
                    _ => {  }
                }
            }
        }

        games.push((red * green * blue) as u32)
    }

    return games;
}
