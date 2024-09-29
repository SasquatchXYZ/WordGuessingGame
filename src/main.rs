// use axum::{extract::Path, routing::get};
// use tokio::net::TcpListener;

// async fn double(Path(input): Path<String>) -> String {
//     match input.parse::<i32>() {
//         Ok(num) => format!("{} times 2 is {}!", num, num * 2),
//         Err(e) => format!("Uh oh, weird input: {e}")
//     }
// }

const RANDOM_WORDS: [&str; 6] =
    ["MB", "Windy", "Gnomes", "Johnny", "Seoul", "Interesting"];

#[derive(Clone, Debug, Default)]
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

impl GameApp {
    fn start(&mut self) {
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
    fn print_results(&self) {
        let output = self
            .current_word
            .chars()
            .map(|c| {
                if self.right_guesses.contains(&c) {
                    c
                } else {
                    '*'
                }
            })
            .collect::<String>();
        println!("{output}");
    }
    fn take_guess(&mut self, guess: String) {
        match guess.chars().count() {
            0 => println!("What are you doing? Please guess something"),
            1 => {
                let the_guess = guess.chars().next().unwrap();

                match self.check_guesses(the_guess) {
                    Guess::AlreadyGuessed => {
                        println!("You already guessed {the_guess}!");
                    }
                    Guess::Right => {
                        self.right_guesses.push(the_guess);
                        println!("Yes, it contains a {the_guess}!");
                    }
                    Guess::Wrong => {
                        self.wrong_guesses.push(the_guess);
                        println!("Nope, it doesn't contain a {the_guess}!")
                    }
                }
                self.print_results();
                println!(
                    "Already guess: {}",
                    self.wrong_guesses.iter().collect::<String>()
                );
            }
            _ => {
                if self.current_word == guess {
                    println!("You guessed right, it's {}!", self.current_word);
                } else {
                    println!("Bzzt! It's not '{guess}', it's {}.\nTime to move on to another word!",
                             self.current_word
                    );
                }
                self.start();
            }
        }
    }
}

fn main() {
    let mut app = GameApp::default();
    app.start();

    loop {
        println!("Guess the word!");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        app.take_guess(guess.trim().to_lowercase());
    }
}

// #[tokio::main]
// async fn main() {
//     let app = axum::Router::new()
//         .route("/", get(|| async { "The server works!" }))
//         .route("/game/:guess",
//                get(|Path(guess): Path<String>| async move { format!("The guess is {guess}") }),
//         )
//         .route("/double/:number", get(double));
//
//     let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
//     axum::serve(listener, app)
//         .await
//         .unwrap();
// }
