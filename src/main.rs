use std::fmt;
use std::io::{self, Write};

struct Stack<T: fmt::Display> {
    elements: Vec<T>,
}

#[allow(dead_code)]
impl<T: fmt::Display> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            elements: Vec::new(),
        }
    }
    fn push(&mut self, elem: T) {
        self.elements.push(elem);
    }
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_len = self
            .elements
            .iter()
            .map(|item| format!("{}", item).len())
            .max()
            .unwrap_or(0);

        writeln!(f, "{}{}{}", "+", "-".repeat(max_len + 2), "+")?;
        for item in self.elements.iter().rev() {
            let item_str = format!("{}", item);
            writeln!(f, "| {:^width$} |", item_str, width = max_len)?;
            writeln!(f, "{}{}{}", "+", "-".repeat(max_len + 2), "+")?;
        }
        Ok(())
    }
}

fn read_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn handle_pop<T: fmt::Display>(stack: &mut Stack<T>) {
    match stack.pop() {
        Some(item) => println!("Popped: {}", item),
        None => println!("Stack is empty"),
    }
}

fn handle_push<T: fmt::Display>(stack: &mut Stack<T>, element: T) {
    stack.push(element);
}

fn handle_print<T: fmt::Display>(stack: &Stack<T>) {
    if stack.is_empty() {
        println!("Stack is empty");
    } else {
        println!("{}", stack);
    }
}

enum Command {
    Push(String),
    Pop,
    Print,
    Exit,
    Invalid,
}

impl Command {
    fn from_input(input: &str) -> Self {
        let trimmed = input.trim();
        if trimmed.starts_with("push ") {
            Command::Push(trimmed[5..].trim().to_string())
        } else if trimmed == "pop" {
            Command::Pop
        } else if trimmed == "print" {
            Command::Print
        } else if trimmed == "exit" {
            Command::Exit
        } else {
            Command::Invalid
        }
    }
}

fn main() {
    let mut stack: Stack<String> = Stack::new();

    loop {
        let input = read_input();
        let command = Command::from_input(&input);

        match command {
            Command::Push(item) => handle_push(&mut stack, item),
            Command::Pop => handle_pop(&mut stack),
            Command::Print => handle_print(&stack),
            Command::Exit => break,
            Command::Invalid => println!("Invalid command"),
        }
    }
}
