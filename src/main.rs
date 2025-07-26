use random_word::Lang;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

fn get_random_sentence(n: usize) -> String {
    let mut sentence = String::new();
    for _ in 0..n {
        sentence.push_str(random_word::get(Lang::En));
        sentence.push_str(" ");
    }
    sentence.pop();
    sentence
}

fn print_stats(time_taken: Duration, num_words: usize, sentence: &String) {
    let wpm = 60_f32 * num_words as f32 / time_taken.as_secs_f32();
    let characters_per_second = sentence.len() as f32 / time_taken.as_secs_f32();

    println!("\nSpeed = {} WPM", wpm);
    println!("Speed = {} Characters per Second", characters_per_second);
    println!("Total time taken = {:?}", time_taken);
    println!("Number of words = {}", num_words);
    println!("Number of characters = {}\n", sentence.len());
}

fn main() {
    let num_words = 4;
    let sentence = get_random_sentence(num_words);
    println!("Please type the following sentence:\n");
    println!("{}\n", sentence);

    println!("Press 'y' and then Enter to start.");

    let stdin = io::stdin();
    // Lock stdin for faster I/O and get an iterator over lines.
    let mut lines = stdin.lock().lines();
    let user_choice = lines
        .next()
        .unwrap()
        .expect("Failed to read the first line");

    if user_choice != "y" {
        println!("User exited. Have a good day.");
        return;
    }

    let timer = Instant::now(); // Record the starting time

    let user_speed_test_answer = lines
        .next()
        .unwrap()
        .expect("Failed to read user test input");

    match user_speed_test_answer == sentence {
        true => {
            println!("\nCorrect answer!");
            print_stats(timer.elapsed(), num_words, &sentence);
        }
        false => {
            println!("Wrong answer. You mistyped somewhere.");
        }
    }
}
