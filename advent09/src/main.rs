use std::io::Error;
use std::fs::File;
use std::io::prelude::*;

fn read_input(filename: &str) -> Result<String, Error> {
  let mut input = String::new();
  File::open(filename)?.read_to_string(&mut input)?;
  return Ok(input);
}

fn main() {
  match read_input("input.txt") {
    Ok(input) => {
      let (part1, part2) = answers(&input);
      println!("Part 1 answer: {}", part1);
      println!("Part 2 answer: {}", part2);
    },
    Err(e) => println!("Error: {}", e),
  }
}


const STATE_NORMAL: i32 = 0;
const STATE_IN_GARBAGE: i32 = 1;
const STATE_IN_BANG: i32 = 2;

fn answers(input: &str) -> (i32, i32) {
  let mut chars = input.chars();
  let mut done = false;

  let mut depth = 0;
  let mut score = 0;
  let mut garbage_chars = 0;
  let mut state = STATE_NORMAL;

  while !done {
    match chars.next() {
      None => { done = true; },
      Some(c) => {
        //println!("c={} state={} depth={} score={}", c, state, depth, score);

        if state == STATE_IN_BANG {
          state = STATE_IN_GARBAGE;
        }
        else if state == STATE_IN_GARBAGE {
          if c == '>' {
            state = STATE_NORMAL;
          }
          else if c == '!' {
            state = STATE_IN_BANG;
          }
          else {
            garbage_chars += 1;
          }
        }
        else if state == STATE_NORMAL {
          if c == '{' {
            depth += 1;
            score += depth;
          }
          else if c == '}' {
            depth -= 1;
          }
          else if c == '<' {
            state = STATE_IN_GARBAGE;
          }
        }
      }
    }
  }

  return (score, garbage_chars);
}

