use crate::util::ascii::{ascii_code_list, ASCIIError};

pub fn run() {
    match ascii_code_list('A', 'z') {
        Ok(result) => {
            for code in result {
                print!("{} ", code as char);
            }
            println!();
        }
        Err(e) => match e {
            ASCIIError::InvalidChar => {
                eprintln!("PrintCharsError::InvalidChar");
            }
        },
    }
}
