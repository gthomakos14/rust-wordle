use random_word::Lang;
use std::io;

fn main() {
    let mut success = false;
    let answer_word = random_word::gen_len(5, Lang::En)
        .unwrap_or_default();

    let mut countdown = 1;
    let word_list = random_word::all_len(5, Lang::En).unwrap_or_default();

    while countdown < 7{
        println!("Guess a 5 letter word. This is guess {countdown}");
        let user_guess = take_guess();
        if user_guess.len() != 5{
            println!("I said 5 letters.");
            continue
        }
        let mut found = false;
        for &s in word_list{
            if s == user_guess{
                found = true;
                break
            }
        }
        if !found{
            println!("That's not a real word.");
            continue
        }
        if check_win(user_guess.clone(), answer_word.to_string()) {
            println!("You got it in {countdown}!");
            success = true;
            break
        }
        countdown += 1;
        for (count, letter) in user_guess.chars().enumerate(){
            if letter == answer_word.chars().nth(count).unwrap_or_default(){
                println!("{letter}: green");
                continue;
            }
            else if answer_word.contains(letter){
                println!("{letter}: yellow");
            }
            else {
                println!("{letter}: black")
            }
        }
    }
    if !success {
        println!("You didn't get it and you're a loser. The word was {answer_word}.")
    }
}

fn take_guess() -> String{
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    let guess = guess.trim();

    // assert_eq!(guess.len(), 5);
    guess.to_string().to_lowercase()
}

fn check_win(guess_word: String, winning_word: String) -> bool{
    guess_word == winning_word
}