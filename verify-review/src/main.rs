use std::{process::Command, path::PathBuf, fmt::{Formatter, Display}};

use glob::glob;
use inquire::Select;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Clean (delete) all *.received.json files without review.
    #[arg(short, long)]
    clean: bool,

    /// Auto accept all *.received.json files without review.
    #[arg(short, long)]
    accept: bool,
}



/// Used to indicate the result of a review
pub enum ReviewResult {
    Match,
    NoMatch,
    NoChanges,
}

/// Store information about a test that can be reviewed
#[derive(Clone, PartialEq, Eq, Debug)]
struct ReviewableTest {
    base_name: String,
    accepted: PathBuf,
    accepted_sha256: String,
    received: PathBuf,
    received_sha256: String,
    already_matches: bool,
}


fn main() {
    let args = Cli::parse();
    if args.clean {
        clean();
    } else if args.accept {
        auto_accept();
    } else {
        read_and_review_tests();
    }
}

/// Clean (delete) all *.received.json files without review.
fn clean() {
    let tests_to_review = find_reviewable_tests();
    tests_to_review.iter().for_each(|test| {
        println!("Removing {}", test.base_name);
        std::fs::remove_file(&test.received).expect("Failed to remove accepted file");
    });
}

/// Auto accept all *.received.json files without review.
fn auto_accept() {
    let tests_to_review = find_reviewable_tests();
    tests_to_review.iter().for_each(|test| {
        println!("Accepting received file for {}", test.base_name);
        std::fs::remove_file(&test.accepted).expect("Failed to remove accepted file");
        std::fs::rename(&test.received, &test.accepted).expect("Failed to rename received file");
    });
}


/// Default operation - read and review all tests with changes
fn read_and_review_tests() {
    let mut tests_to_review = find_reviewable_tests();
    if tests_to_review.is_empty() {
        println!("No reviewable tests found");
        return;
    }

    // Delete received files where the text already matches.
    tests_to_review.iter().for_each(|test| {
        if test.already_matches {
            println!("Removing already matching received file for {}", test.base_name);
            std::fs::remove_file(&test.received).expect("Failed to remove accepted file");
        }
    });

    // Remove tests where already_matches is true.
    tests_to_review.retain(|test| !test.already_matches);
    
    while !tests_to_review.is_empty() {
        let user_input = Select::new("Test to review?", tests_to_review.clone()).prompt();

        if user_input.is_err() {
            println!("No test selected");
            break;
        }

        let test = user_input.unwrap();

        let result = review(&test);
        match result {
            Ok(ReviewResult::Match) => {
                println!("Files match, marking as reviewed.");
                tests_to_review.retain(|t| t != &test);
                std::fs::remove_file(&test.received).expect("Failed to remove accepted file");
            },
            Ok(ReviewResult::NoMatch) => {
                let confirm = inquire::Confirm::new("Changes made, but files don't match. Mark as reviewed and remove the received file?").prompt();
                if confirm.is_err() {
                    println!("No changes made.");
                    continue;
                }
                if confirm.unwrap() {
                    tests_to_review.retain(|t| t != &test);
                    std::fs::remove_file(&test.received).expect("Failed to remove accepted file");
                } else {
                    println!("Not marking as reviewed.");
                    continue;
                }
            },
            Ok(ReviewResult::NoChanges) => {
                let confirm = inquire::Confirm::new("No changes made. Mark as reviewed and remove the received file?").prompt();
                if confirm.is_err() {
                    println!("No changes made.");
                    continue;
                }
                if confirm.unwrap() {
                    tests_to_review.retain(|t| t != &test);
                    std::fs::remove_file(&test.received).expect("Failed to remove accepted file");
                } else {
                    println!("Not marking as reviewed.");
                    continue;
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        }
    }

}

/// Walks the current directory looking for pairs of files matching *.accepted.json and *.received.json
fn find_reviewable_tests() -> Vec<ReviewableTest> {
    glob("**/*.received.json").expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter_map(|entry| {
            // Get the filename as a string
            let received = entry;
            let received_sha256 = hash_file(&received);
            let base_name = received.file_name().unwrap().to_str().unwrap().to_string().replace(".received.json", "");
            
            // See if the matching accepted.json file exists.
            let accepted = received.with_file_name(format!("{}.accepted.json", base_name));
            let accepted_sha256 = hash_file(&accepted);

            let already_matches = accepted_sha256 == received_sha256;

            if accepted.exists() {
                Some(ReviewableTest {
                    base_name,
                    accepted,
                    accepted_sha256,
                    received,
                    received_sha256,
                    already_matches,
                })
            } else {
                None
            }
        })
        .collect()
}


/// Run a file through sha256sum and return the hash.
fn hash_file(file: &PathBuf) -> String {
    // We read the file to a string and trim it to remove any trailing whitespace.
    let text = std::fs::read_to_string(file).expect("failed to read file");
    let text = text.trim();
    sha256::digest(text)
}

impl Display for ReviewableTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base_name)
    }
}

/// Run vimdiff on the accepted and received files.
fn review(test_case: &ReviewableTest) -> Result<ReviewResult, Box<dyn std::error::Error>> {
    let initial_accepted = hash_file(&test_case.accepted);
    let initial_received = hash_file(&test_case.received);

    let mut res = Command::new("vimdiff")
        .arg(&test_case.accepted)
        .arg(&test_case.received)
        .spawn()?;
    res.wait()?;
    
    let final_accepted = hash_file(&test_case.accepted);
    
    if final_accepted == initial_received {
        Ok(ReviewResult::Match)
    } else if final_accepted != initial_accepted {
        Ok(ReviewResult::NoMatch)
    } else {
        Ok(ReviewResult::NoChanges)
    }
}

