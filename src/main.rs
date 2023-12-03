use std::fs;
use substring::Substring;

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn is_equal(&self, pos: &Pos) -> bool {
        if self.x == pos.x && self.y == pos.y {
            true
        } else {
            false
        }
    }
}

fn main() {
    println!("Day One Part One Solution: {:?}", day_one_part_one());
    println!("Day One Part Two Solution: {:?}", day_one_part_two());
    println!("Day Two Part One Solution: {:?}", day_two_part_one().iter().sum::<u32>());
    println!("Day Two Part Two Solution: {:?}", day_two_part_two().iter().sum::<u32>());
    println!("Day Three Part One Solution: {:?}", day_three_part_one().iter().sum::<u32>());
    println!("Day Three Part One Solution: {:?}", day_three_part_two().iter().sum::<u32>());
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

fn day_three_part_one() -> Vec<u32> {
    struct PartNumber {
        digits: Vec<u32>,
        has_symbol: bool
    }

    let data = fs::read_to_string("C:\\Users\\oscar\\Documents\\repo\\Rust\\advent-of-code\\src\\input_d3.txt").expect("error reading file");
    let data_lines = data.split("\n").map(|line| line.trim().chars().collect()).collect::<Vec<Vec<char>>>();
    let mut numbers: Vec<u32> = vec![];

    for (i, line) in data_lines.iter().enumerate() {
        let mut part_number: PartNumber = PartNumber { digits: vec![], has_symbol: false };
        for (j, char) in line.iter().enumerate() {
            let number = char.to_digit(10);
            match number {
                Some(n) => {
                    part_number.digits.push(n);
                    if check_adj_symbol(&data_lines, i, j) { part_number.has_symbol = true; }
                    if j == data_lines[i].len()-1 && part_number.has_symbol {
                        let final_number = part_number.digits.clone().into_iter().fold(0, |acc, n| (acc*10)+n);
                        numbers.push(final_number);
                    }
                },
                None => {
                    if part_number.digits.len() > 0 && part_number.has_symbol {
                        let final_number = part_number.digits.clone().into_iter().fold(0, |acc, n| (acc*10)+n);
                        numbers.push(final_number);
                    }
                    part_number.digits.clear();
                    part_number.has_symbol = false;
                }
            }
        }
    }

    return numbers;
}

fn day_three_part_two() -> Vec<u32> {

    #[derive(Debug)]
    struct PartNumber {
        digits: Vec<u32>,
        has_symbol: bool,
        gearpos: Pos
    }

    #[derive(Debug, Copy, Clone)]
    struct NumberPos {
        number: u32,
        pos: Pos
    }

    // Slice the file into a vector of lines and each line into a vector of characters
    let data = fs::read_to_string("C:\\Users\\oscar\\Documents\\repo\\Rust\\advent-of-code\\src\\input_d3.txt").expect("error reading file");
    let data_lines = data.split("\n").map(|line| line.trim().chars().collect()).collect::<Vec<Vec<char>>>();
    let mut numbers: Vec<u32> = vec![];
    let mut numbers_pos: Vec<NumberPos> = vec![]; // Positions of al the numbers with adjacent gears found
    let mut gears_pos: Vec<Pos> = vec![]; // Positions of all the gears found

    for (i, line) in data_lines.iter().enumerate() {
        // var used to temporarily store the digits and symbol as we scan a line
        let mut part_number: PartNumber = PartNumber { digits: vec![], has_symbol: false, gearpos: Pos{x: 0, y: 0} };
        for (j, char) in line.iter().enumerate() {
            // Search the vector for characters that are digits
            let number = char.to_digit(10);
            match number {
                Some(n) => {
                    // If the character is a digit push it to the current number memory
                    part_number.digits.push(n);
                    // Check if there is a gear adjacent to the number, if true, activate the has_symbol flag and store the gear position in the number
                    // if its the first time the gear is found also store the gear position in the gears list
                    let gear_pos = check_adj_gear(&data_lines, i, j);
                    if gear_pos.is_some() {
                        part_number.has_symbol = true;
                        let gear_pos = gear_pos.unwrap();
                        part_number.gearpos = gear_pos;
                        if !gears_pos.iter().any(|&pos| pos.is_equal(&gear_pos) ) { gears_pos.push(gear_pos) };
                    }
                    // If its the end of line and a gear was found form the number and add to list of number
                    if j == data_lines[i].len()-1 && part_number.has_symbol {
                        let final_number = part_number.digits.clone().into_iter().fold(0, |acc, n| (acc*10)+n);
                        numbers_pos.push(NumberPos{ number: final_number, pos: part_number.gearpos });
                    }
                },
                None => {
                    // If a non number was found check if we were forming a number and add it to list
                    if part_number.digits.len() > 0 && part_number.has_symbol {
                        let final_number = part_number.digits.clone().into_iter().fold(0, |acc, n| (acc*10)+n);
                        numbers_pos.push(NumberPos{ number: final_number, pos: part_number.gearpos });
                    }
                    // Clear memory number
                    part_number.digits.clear();
                    part_number.has_symbol = false;
                }
            }
        }
    }

    for pos in gears_pos.iter() {
        // Foreach gear position, check if at least 2 numbers share it and add the multiplication of the 2 numbers to the final number list
        let pairs = numbers_pos.iter().filter(|&number| number.pos.is_equal(pos)).collect::<Vec<&NumberPos>>();
        if pairs.len() == 2 {
            numbers.push(pairs.iter().fold(1, |a, &&b| a * b.number ));
        }
    }

    return numbers;
}

// Given a 2 dimensional vector and a position, checks all posible adjacent positions for a symbol that doesn't match the list
fn check_adj_symbol(data_lines: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let non_symbols: Vec<char> = vec!['.','0','1','2','3','4','5','6','7','8','9'];

    if x > 0 && y > 0 && !non_symbols.contains(&data_lines[x-1][y-1]) { return true; }
    if x > 0 && !non_symbols.contains(&data_lines[x-1][y]) { return true; }
    if x > 0 && y < data_lines[x].len()-1 && !non_symbols.contains(&data_lines[x-1][y+1]) { return true; }

    if y > 0 && !non_symbols.contains(&data_lines[x][y-1]) { return true; }
    if y < data_lines[x].len()-1 && !non_symbols.contains(&data_lines[x][y+1]) { return true; }

    if x < data_lines.len()-1 && y > 0 && !non_symbols.contains(&data_lines[x+1][y-1]) { return true; }
    if x < data_lines.len()-1 && !non_symbols.contains(&data_lines[x+1][y]) { return true; }
    if x < data_lines.len()-1 && y < data_lines[x].len()-1 && !non_symbols.contains(&data_lines[x+1][y+1]) { return true; }

    return false;
}

// Given a 2 dimensional vector and a position, checks all posible adjacent positions for a given character, returns the position of the found character or nothing
fn check_adj_gear(data_lines: &Vec<Vec<char>>, x: usize, y: usize) -> Option<Pos>{
    let symbol: &char = &'*';

    if x > 0 && y > 0 && &data_lines[x-1][y-1] == symbol { return Some(Pos{ x: x-1, y: y-1}); }
    if x > 0 && &data_lines[x-1][y] == symbol { return Some(Pos{ x: x-1, y }); }
    if x > 0 && y < data_lines[x].len()-1 && &data_lines[x-1][y+1] == symbol { return Some(Pos{ x: x-1, y: y+1}); }

    if y > 0 && &data_lines[x][y-1] == symbol { return Some(Pos{ x, y: y-1}); }
    if y < data_lines[x].len()-1 && &data_lines[x][y+1] == symbol { return Some(Pos{ x, y: y+1}); }

    if x < data_lines.len()-1 && y > 0 && &data_lines[x+1][y-1] == symbol { return Some(Pos{ x: x+1, y: y-1}); }
    if x < data_lines.len()-1 && &data_lines[x+1][y] == symbol { return Some(Pos{ x: x+1, y}); }
    if x < data_lines.len()-1 && y < data_lines[x].len()-1 && &data_lines[x+1][y+1] == symbol { return Some(Pos{ x: x+1, y: y+1}); }

    return None;
}
