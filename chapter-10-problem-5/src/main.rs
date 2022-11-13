// Author: Grant Duchars
// Project: CSC 28500 Team Project 2
// Problem: Chapter 10 Computations and Explorations 5
use std::io::{stdin, stdout, Write};
use std::time;

fn main() {
    // Welcome user and prompt for user input
    println!("Welcome to the Gray code generator! Please enter the length of Gray codes you want generated.");
    print!("Enter number of bits: ");
    stdout().flush().unwrap();

    // Get user input and convert to integer
    let mut length = String::new();
    stdin().read_line(&mut length).unwrap();
    length.pop();
    let length = length.parse::<usize>().unwrap();

    // Run Gray code recursion function
    println!("\nGray codes through recursion:");
    let recursion_now = time::Instant::now();
    gray_code_recursion(0, length, (1 << length) - 1);
    let recursion_elapsed = recursion_now.elapsed();
    println!();

    // Run Gray code lists function
    println!("Gray codes through lists:");
    let lists_now = time::Instant::now();
    let mut list: Vec<usize> = vec![0, 1];
    gray_code_lists(&mut list, 1, length);
    for num in list.iter() {
        print!("{num:0length$b} ");
    }
    println!("\n");
    let lists_elapsed = lists_now.elapsed();

    // Print out timings for both functions
    println!(
        "Gray code function timings:\nRecursion: {} seconds\nLists: {} seconds",
        recursion_elapsed.as_secs_f32(),
        lists_elapsed.as_secs_f32()
    );
}

fn gray_code_recursion(num: usize, length: usize, max_val: usize) {
    if num == max_val {
        println!("{:0length$b}", (num >> 1) ^ num);
        return;
    }
    print!("{:0length$b} ", (num >> 1) ^ num);
    gray_code_recursion(num + 1, length, max_val);
}

fn gray_code_lists(list: &mut Vec<usize>, cur_length: usize, max_length: usize) {
    let mut reverse_list: Vec<usize> = list.clone();
    reverse_list.reverse();
    for i in 0..1 << cur_length {
        reverse_list[i] |= 1 << cur_length;
    }
    if cur_length != max_length {
        list.append(&mut reverse_list);
        gray_code_lists(list, cur_length + 1, max_length);
    }
}
