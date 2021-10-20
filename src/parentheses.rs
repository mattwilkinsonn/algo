use simple_error::SimpleError;

struct Stack {
    pub array: [char; 10],
    pub pointer: usize,
}

impl Stack {
    pub fn new() -> Self {
        let array = ['\n'; 10];
        let pointer = 0;

        Self { array, pointer }
    }

    pub fn push(&mut self, value: char) -> Result<(), SimpleError> {
        if self.pointer >= self.array.len() {
            return Err(SimpleError::new("Stack is full"));
        }

        self.array[self.pointer] = value;
        self.pointer += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<char, SimpleError> {
        if self.pointer == 0 {
            return Err(SimpleError::new("Stack is empty"));
        }
        self.pointer -= 1;
        let value = self.array[self.pointer];
        self.array[self.pointer] = '\n';
        Ok(value)
    }
}

fn check_parentheses(input: &str) -> bool {
    let mut stack = Stack::new();

    for char in input.chars() {
        if char == '{' || char == '(' || char == '[' {
            stack.push(char).unwrap();
            continue;
        }

        let popped = stack.pop().unwrap();

        if char == '}' && popped != '{'
            || char == ')' && popped != '('
            || char == ']' && popped != '['
        {
            return false;
        }
    }

    true
}

fn main() {
    let string = "(([][){})";

    let is_valid = check_parentheses(string);

    println!("input: {}", &string);

    match is_valid {
        true => println!("balanced"),
        false => println!("unbalanced"),
    }
}
