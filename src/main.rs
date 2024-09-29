use axum::{extract::Path, routing::get};
use std::sync::Mutex;
use tokio::net::TcpListener;

const RANDOM_WORDS: [&str; 6] =
    ["MB", "Windy", "Gnomes", "Johnny", "Seoul", "Interesting"];

static GAME: Mutex<GameApp> = Mutex::new(GameApp {
    current_word: String::new(),
    right_guesses: vec![],
    wrong_guesses: vec![],
});

#[derive(Clone, Debug)]
struct GameApp {
    current_word: String,
    right_guesses: Vec<char>,
    wrong_guesses: Vec<char>,
}

enum Guess {
    Right,
    Wrong,
    AlreadyGuessed,
}

async fn get_res_from_static(Path(guess): Path<String>) -> String {
    GAME.lock().unwrap().take_guess(guess)
}

impl GameApp {
    fn restart(&mut self) {
        self.current_word =
            RANDOM_WORDS[fastrand::usize(..RANDOM_WORDS.len())].to_lowercase();
        self.right_guesses.clear();
        self.wrong_guesses.clear();
    }
    fn check_guesses(&self, guess: char) -> Guess {
        if self.right_guesses.contains(&guess) ||
            self.wrong_guesses.contains(&guess) {
            return Guess::AlreadyGuessed;
        }
        match self.current_word.contains(guess) {
            true => Guess::Right,
            false => Guess::Wrong,
        }
    }
    fn results_so_far(&self) -> String {
        let mut output = String::new();
        for c in self.current_word.chars() {
            if self.right_guesses.contains(&c) {
                output.push(c);
            } else {
                output.push('*');
            }
        }
        output
    }
    fn take_guess(&mut self, guess: String) -> String {
        let guess = guess.to_lowercase();
        let mut output = String::new();
        match guess {
            guess if guess.chars().count() == 1 => {
                let the_guess = guess.chars().next().unwrap();

                match self.check_guesses(the_guess) {
                    Guess::AlreadyGuessed => {
                        output.push_str(&format!("You already guessed {the_guess}!\n"));
                    }
                    Guess::Right => {
                        self.right_guesses.push(the_guess);
                        output.push_str(&format!("Yes, it contains a {the_guess}!\n"));
                    }
                    Guess::Wrong => {
                        self.wrong_guesses.push(the_guess);
                        output.push_str(&format!("Nope, it doesn't contain a {the_guess}!\n"));
                    }
                }
                output.push_str(&self.results_so_far());
            }
            guess => {
                if self.current_word == guess {
                    output.push_str(&format!("You guessed right, it's {}!  Let's plan again!", self.current_word));
                } else {
                    output.push_str(&format!("Bzzt! It's not '{guess}', it's {}.\nTime to move on to another word!",
                                             self.current_word
                    ));
                }
                self.restart();
            }
        }
        output
    }
}

#[tokio::main]
async fn main() {
    GAME.lock().unwrap().restart();

    let app = axum::Router::new()
        .route("/", get(|| async { "The server is running well!" }))
        .route("/game/:guess", get(get_res_from_static));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
