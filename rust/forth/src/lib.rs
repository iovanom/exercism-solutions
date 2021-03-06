use std::collections::HashMap;
use std::str::SplitWhitespace;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

type OperandsResult<V> = Result<V, Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut commands = input.split_whitespace();
        while let Some(command) = commands.next() {
            let command = command.to_lowercase();
            match command.as_ref() {
                ":" => self.collect_word(&mut commands)?,
                _ => self.exec_command(command)?,
            }
        }
        Ok(())
    }

    fn exec_command(&mut self, command: String) -> ForthResult {
        if let Some(word_commands) = self.get_word_commands(&command) {
            self.eval(&word_commands)?;
        } else {
            match command.as_ref() {
                "+" => self.add()?,
                "-" => self.subtract()?,
                "/" => self.divide()?,
                "*" => self.multiply()?,
                "dup" => self.dup()?,
                "drop" => self.drop()?,
                "swap" => self.swap()?,
                "over" => self.over()?,
                _ => {
                    if let Ok(command) = command.parse::<Value>() {
                        self.stack.push(command);
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
            }
        }
        Ok(())
    }

    fn get_word_commands(&self, word: &str) -> Option<String> {
        if let Some(word_commands) = self.words.get(word) {
            Some(String::from(word_commands))
        } else {
            None
        }
    }

    fn get_operands(&mut self) -> OperandsResult<(Value, Value)> {
        let op1 = self.stack.pop();
        let op2 = self.stack.pop();
        if let (Some(op1), Some(op2)) = (op1, op2) {
            Ok((op1, op2))
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn get_operand(&mut self) -> OperandsResult<Value> {
        if let Some(op) = self.stack.pop() {
            Ok(op)
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn add(&mut self) -> ForthResult {
        let (a, b) = self.get_operands()?;
        self.stack.push(b + a);
        Ok(())
    }

    fn subtract(&mut self) -> ForthResult {
        let (a, b) = self.get_operands()?;
        self.stack.push(b - a);
        Ok(())
    }

    fn divide(&mut self) -> ForthResult {
        let (a, b) = self.get_operands()?;
        if a == 0 {
            return Err(Error::DivisionByZero);
        }
        self.stack.push(b / a);
        Ok(())
    }

    fn multiply(&mut self) -> ForthResult {
        let (a, b) = self.get_operands()?;
        self.stack.push(b * a);
        Ok(())
    }

    fn dup(&mut self) -> ForthResult {
        let o = self.get_operand()?;
        self.stack.push(o);
        self.stack.push(o);
        Ok(())
    }
    fn drop(&mut self) -> ForthResult {
        self.get_operand()?;
        Ok(())
    }

    fn swap(&mut self) -> ForthResult {
        let (a, b) = self.get_operands()?;
        self.stack.push(a);
        self.stack.push(b);
        Ok(())
    }

    fn over(&mut self) -> ForthResult {
        let (a, b) = self.get_operands()?;
        self.stack.push(b);
        self.stack.push(a);
        self.stack.push(b);
        Ok(())
    }

    fn validate_word(&mut self, word: &str) -> ForthResult {
        if word.chars().all(|c| c.is_digit(10)) {
            Err(Error::InvalidWord)
        } else {
            Ok(())
        }
    }

    fn collect_word(&mut self, commands: &mut SplitWhitespace) -> ForthResult {
        let mut word: Option<String> = None;
        let mut word_commands = String::new();
        while let Some(command) = commands.next() {
            let mut command = command.to_lowercase();
            if word.is_none() {
                self.validate_word(&command)?;
                word = Some(command);
            } else if command == ";" {
                if let Some(word) = word {
                    self.add_word(word, word_commands);
                    return Ok(());
                }
            } else if word.is_some() {
                if let Some(inner_word) = self.get_word_commands(&command) {
                    command = inner_word;
                }
                word_commands += &command;
                word_commands += " ";
            }
        }
        Err(Error::InvalidWord)
    }

    fn add_word(&mut self, word: String, commands: String) {
        self.words.insert(word, commands);
    }
}
