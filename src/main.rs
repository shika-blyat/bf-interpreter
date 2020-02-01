#[derive(Debug)]
struct Memory {
    block: Vec<usize>,
    pos: usize,
}
impl Memory {
    pub fn increment_current(&mut self) {
        self.block[self.pos] += 1;
    }
    pub fn decrement_current(&mut self) {
        self.block[self.pos] -= 1;
    }
    pub fn move_right<'a>(&mut self) -> Result<(), &'a str> {
        if self.pos >= self.block.len() - 1 {
            self.block.push(0);
        }
        self.pos += 1;
        Ok(())
    }
    pub fn move_left<'a>(&mut self) -> Result<(), &'a str> {
        if self.pos == 0 {
            return Err("Cannot decrease memory pointer if it's already at 0");
        }
        self.pos -= 1;
        Ok(())
    }
    pub fn take_char<'a>(&mut self) -> Result<(), &'a str> {
        use std::io::Read;

        let input = std::io::stdin()
            .bytes()
            .next()
            .and_then(|byte| byte.ok())
            .map(|byte| byte as usize)
            .ok_or("Failed to read an unique char from stdin")?;
        self.block[self.pos] = input;
        Ok(())
    }
    pub fn print_char(&mut self) -> Result<(), String> {
        use std::convert::TryInto;
        use std::io::{self, Write};
        let current = self.block[self.pos];
        let c = current
            .try_into()
            .ok()
            .and_then(|c| std::char::from_u32(c).or_else(|| None))
            .ok_or(format!("`{}` isn't a valid utf8 char", current))?;
        print!("{}", c);
        io::stdout().flush().unwrap();
        Ok(())
    }
    pub fn current(&self) -> usize {
        self.block[self.pos]
    }
}

fn eval<'a>(s: &'a str, mem: &mut Memory) -> Result<(), String> {
    let chars = s.chars().collect::<Vec<char>>();
    let mut current = 0;
    let mut loop_stack = vec![];
    loop {
        if current >= chars.len() {
            break Ok(());
        }
        match chars[current] {
            '+' => mem.increment_current(),
            '-' => mem.decrement_current(),
            '>' => mem.move_right()?,
            '<' => mem.move_left()?,
            ',' => mem.take_char()?,
            '.' => mem.print_char()?,
            '[' => loop_stack.push(current),
            ']' => {
                if mem.current() != 0 && mem.current() != 48 {
                    // 48 is the ascii value of 0
                    current = match loop_stack.last() {
                        Some(v) => *v,
                        None => {
                            return Err(format!(
                                "Closing delimiter without opening one at char {}",
                                current
                            ))
                        }
                    }
                } else {
                    loop_stack.pop();
                }
            }
            _ => (), // Everything else is a comment
        }
        current += 1;
    }
}

// Should print `Hello World!\n`
const CODE: &str = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";

fn main() {
    let mut mem = Memory {
        block: vec![0],
        pos: 0,
    };
    match eval(CODE, &mut mem) {
        Ok(_) => println!("\n{:#?}", mem),
        Err(e) => println!("\n{:#?}", e),
    }
}
