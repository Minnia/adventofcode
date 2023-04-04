use serde_json::to_string_pretty;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    convert_string_to_json();
    let contents = read_file();
    let data: Vec<_> = contents
        .trim()
        .split("\n\n")
        .map(|elves_calories| {
            elves_calories
                .lines()
                .map(|calories: &str| calories.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.iter().sum::<i32>())
        .collect::<Vec<_>>();

    let find_max = data.iter().max().unwrap();

    let second_highest = data.iter().filter(|&x| x != find_max).max().unwrap();
    let third_highest = data
        .iter()
        .filter(|&x| x != find_max && x != second_highest)
        .max()
        .unwrap();

    let sum_of_max_second_third = find_max + second_highest + third_highest;

    let find_min = data.iter().min().unwrap();

    println!("With text max:\n{:#?}", find_max);
    println!("With text find second level:\n{:#?}", second_highest);
    println!("With text find third level:\n{:#?}", third_highest);
    println!("With text find sum:\n{:#?}", sum_of_max_second_third);

    println!("With text min:\n{:#?}", find_min);
}

fn convert_string_to_json() -> std::io::Result<()> {
    // Read the contents of the file into a String variable
    let mut file = File::open("src/calories.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Split the contents into lines and add a dash between non-empty lines
    let mut formatted_contents = String::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            // println!("empty");
            formatted_contents.push_str("empty");
        } else {
            formatted_contents.push_str(line);
            formatted_contents.push(' ');
        }
    }

    // Serialize the file contents into a JSON string
    let json_str = to_string_pretty(&formatted_contents)?;

    // Print the JSON string to the console
    // for line in json_str.lines() {
    //     println!("{}", line);
    // }

    // Split the contents into lines and sum the numbers before the first empty line
    let mut sum = 0;
    for line in json_str.lines() {
        if line.trim().is_empty() {
            println!("Found empty line, stopping summing");
            break;
        }
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        sum += nums.iter().sum::<i32>();
    }

    println!("Sum of numbers before empty line: {}", sum);

    Ok(())
}

fn read_file() -> String {
    let contents =
        fs::read_to_string("src/calories.txt").expect("Something went wrong reading the file");
    return contents;
}
