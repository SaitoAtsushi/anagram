mod anagram;
mod primes;
use anagram::Anagram;
use std::io::BufRead;
use std::ops::Not;

enum CommandLineArguments {
    Nothing,
    Help,
    AnagramSearch(String, String),
    CandidateSearch(String, String),
}

use CommandLineArguments::*;

fn commnad_line_arguments() -> Option<CommandLineArguments> {
    let mut arg_iter = std::env::args();
    let _command_name = arg_iter.next()?;
    let option = arg_iter.next();
    match option {
        None => Some(Nothing),
        Some(s) if s == "--help" => match arg_iter.next() {
            Some(_) => None?,
            _ => Some(Help),
        },
        Some(s) if s == "--candidates" => {
            let word = arg_iter.next()?;
            let filename = arg_iter.next()?;
            Some(CandidateSearch(word, filename))
        }
        Some(s) => {
            let word = s;
            let filename = arg_iter.next()?;
            Some(AnagramSearch(word, filename))
        }
    }
}

static USAGE: &str = r#"
anagram command line syntax:
  anagram --help                             show command line help.
  anagram --candidates word dictionary       show anagram parts candidates.
  anagram word dictionary                    show anagram words combination.
"#;

fn main() {
    let cla = commnad_line_arguments().expect("invalid command line arguments.");
    match cla {
        Nothing | Help => {
            eprint!("{}", USAGE);
            return;
        }
        AnagramSearch(ref word, ref filename) => {
            let file = std::fs::File::open(filename).expect("failed to open file");
            let input_buffer = std::io::BufReader::new(file);
            let word_list_iter = input_buffer.lines().map(Result::unwrap);
            let anagram = Anagram::new(&word);
            let anagram_words = anagram.search(word_list_iter);

            for v in anagram_words.iter() {
                if v.is_empty().not() {
                    print!("{}", v[0]);
                }
                for e in &v[1..] {
                    print!(",{}", e);
                }
                println!("");
            }
        }
        CandidateSearch(ref word, ref filename) => {
            let file = std::fs::File::open(filename).expect("failed to open file");
            let input_buffer = std::io::BufReader::new(file);
            let word_list_iter = input_buffer.lines().map(Result::unwrap);
            let anagram = Anagram::new(&word);
            let candidate_words = anagram.search_candidate(word_list_iter);
            for (s, _) in candidate_words {
                println!("{}", s);
            }
        }
    }
}
