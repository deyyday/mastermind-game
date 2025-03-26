extern crate colored;
use std::io;
use colored::*;

fn input_taker() ->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    if input.trim().len() < 4{
        println!("---------------------------------------------------------");
        println!("| Please Enter 4 Letters that have spaces between them. |");
        println!("---------------------------------------------------------");
    }
    let  word = input.to_uppercase();
    return word; 
}

// fn split_the_input(input: &String)-> &str{
//     let word = input.trim();
//     print!("{:?}", word.chars());
//     return word;git remote -v
// }

fn guess()->String{
    println!("-------------------------------------------------");
    println!("| Now Guess.. You Need To Pick 4 Colors From     |");
    println!("| RED YELLOW ORANGE GREEN BLUE VIOLET            |");
    println!("| Enter Characters with spaces. [like R V G Y]   |");
    println!("-------------------------------------------------");
    let word = input_taker();
    return word;
}

fn compare(guess: &Vec<&str>, ans: &Vec<&str>){
    // let mut result: Vec<&str> = Vec::new();
    let mut print= 0;
    for i in 0..4{
        if guess[i]== ans[i]{
            print!("{}","•".bold().white());
            print +=1;
        }
        else if ans.contains(&guess[i]) {
            print!("{}","•".bold().black());
            print +=1;
        }
    }
    if print ==0{
        println!("None of your guess matches. Please try again.")
    }
    println!(" ");
}

fn new_game(){
    println!("-------------------------------------------------");
    println!("| Lesssgo.. You Need To Pick 4 Colors From below |");
    println!("| R - for RED                                    |");
    println!("| Y - for YELLOW                                 |");
    println!("| O - for ORANGE                                 |");
    println!("| G - for GREEN                                  |");
    println!("| B - for BLUE                                   |");
    println!("| V - for VIOLET                                 |");
    println!("| Enter Characters with spaces. [like R V G Y]   |");
    println!("-------------------------------------------------");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    if input.trim().len() < 6{
        println!("---------------------------------------------------------");
        println!("| Please Enter 4 Letters that have spaces between them. |");
        println!("---------------------------------------------------------");
    }else{
    let input = input.to_uppercase();
    let ans = input.trim().split("").collect::<Vec<_>>();
    let mut chances = 6;
    while chances >0{
        let return_value =guess();
        let guess = return_value.trim().split(" ").collect::<Vec<_>>();
        if guess == ans{
            println!("Hurray You Guessed Correctly.");
            break;
        }
        else{
            compare(&guess, &ans);
        }
        chances -=1;
    }}
}

fn tutorial(){
    println!(" ");
    println!("Welcome to Tutorials");
}

fn exit(){
    println!(" ");
    println!("Are you sure you wanna say bye-bye to us");
}
fn main() {
    println!("--------------------------\n| Welcome To Matermind!   |\n| Press '1' for New Game. |\n| Press '2' for Tutorial. |\n| Press '3' to exit.      |\n--------------------------");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line.");
    println!(" ");
    let choice: i32  = input.trim().parse().expect("Failed to convert into integer");
    match choice {
        1 => new_game(),
        2 => tutorial(),
        3 => exit(),
        _ => println!("Choose among 1/2/3.")
    }
}
