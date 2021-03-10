#[cfg(feature = "english")]
pub use english_linguistic_pkg as i18n;
#[cfg(feature = "french")]
pub use french_linguistic_pkg as i18n;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

trait ToCapitalize {
    fn to_capitalize(self) -> String;
}

impl ToCapitalize for &str {
    fn to_capitalize(self) -> String {
        let mut fchar = self.chars();
        match fchar.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + fchar.as_str(),
        }
    }
}

fn print_message(message: &str, mut end_of_line: Option<&str>) {
    if end_of_line.is_none() {
        end_of_line = Some("!");
    }
    println!("{}{}", message.to_capitalize(), end_of_line.unwrap());
}

fn ask_question(message: &str) {
    println!("{}?", message.to_capitalize());
}

fn wait_for_approval(yes_possibilities: &[&'static str], no_possibilites: &[&'static str]) -> bool {
    let mut ret = String::new();
    loop {
        ret.clear();
        io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
        ret.pop();
        if yes_possibilities.contains(&ret.as_str()) {
            return true;
        } else if no_possibilites.contains(&ret.as_str()) {
            return false;
        } else {
            print_message(i18n::INVALID_INPUT, None);
        }
    }
}

fn ask_for_number() -> u8 {
    let mut ret = String::new();
    loop {
        ret.clear();
        io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
        ret.pop();
        match ret.parse::<u8>() {
            Ok(value) => { return value; },
            _ => continue,
        }
    }
}

fn play_round(to_guess: u8) {
    loop {
        print_message(i18n::GUESS_A_NUMBER, Some(":"));
        let user_guess: u8 = ask_for_number();
        match user_guess.cmp(&to_guess) {
            Ordering::Equal => { print_message(i18n::PREDICTION_WIN, None); break; },
            Ordering::Less => { print_message(i18n::PREDICTION_LESS, Some("...")); },
            Ordering::Greater => { print_message(i18n::PREDICTION_GREATER, Some("...")); },
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    print_message(i18n::HELLO, None);
    loop {
        ask_question(i18n::PLAY_ROUND);
        if wait_for_approval(i18n::YES_POS, i18n::NO_POS) {
            let rng: u8 = rng.gen_range(0..10);
            play_round(rng);
        } else {
            break;
        }
    }
    print_message(i18n::GOOD_BYE, None);
}
