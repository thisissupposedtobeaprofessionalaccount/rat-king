use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

const PORT: u16 = 6247;
const HOST: &str = "127.0.0.1";
const INSTRUCTION_FILE: &str = "instructions.rk";

fn main() {
    let instructions = std::fs::read_to_string(INSTRUCTION_FILE).expect("Failed to read file");
    let mut instructions = instructions_factory(&instructions);
    println!("{:?}", instructions);

    let full_address = format!("{}:{}", HOST, PORT);
    let tcp_listener = TcpListener::bind(full_address);

    match tcp_listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        handle_connection(stream, &mut instructions);
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_e) => {}
    }
}

fn validate_instruction(instruction: &str) -> Result<(), std::io::Error> {
    let instruction = instruction.trim();
    let instruction_parts = instruction.split(" ").collect::<Vec<&str>>();

    if instruction_parts.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid instruction",
        ));
    }

    let prefix = instruction_parts[0];

    match prefix {
        "cmd" => {
            return Ok(());
        }
        "set" => {
            return Ok(());
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid instruction",
            ));
        }
    }
}
fn instructions_factory(source_instruction: &str) -> Vec<&str> {
    let mut instructions = source_instruction.split("\n").collect::<Vec<&str>>();
    instructions.pop();
    let mut final_instructions : Vec<&str> = Vec::new();
    let mut i = 0;

    for instruction in &instructions {
        match validate_instruction(instruction) {
            Err(_) => {
                eprintln!("Invalid instruction at line {} : {}", i + 1, instruction);
            }
            Ok(_) => {
                final_instructions.push(instruction);
            
            },
        }
        i += 1;
    }

    final_instructions
}

fn handle_connection(stream: TcpStream, instructions: &mut Vec<&str>) {
    manage_instructions(instructions, &stream);
    let client_response = read_client_response(&stream);

    if let Ok(response) = client_response {
        println!("{}", response);
    }
}
fn manage_instructions(instructions: &mut Vec<&str>, stream: &TcpStream) -> Option<()> {
    let current_instruction = instructions.get(0);
    if let Some(instruction) = current_instruction {
        match send_instruction(stream, instruction) {
            Ok(_) => {
                instructions.remove(0);
                return Some(());
            }
            Err(_) => {
                return None;
            }
        }
    }
    None
}
fn send_instruction(
    mut stream: &std::net::TcpStream,
    instruction: &str,
) -> Result<(), std::io::Error> {
    let instruction = format!("{}\n", instruction);
    stream.write(instruction.as_bytes())?;
    Ok(())
}

fn read_client_response(stream: &TcpStream) -> Result<String, std::io::Error> {
    let mut buffer = [0; 1024];
    stream.peek(&mut buffer)?;
    let buffer = String::from_utf8_lossy(&buffer);

    Ok(buffer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_instruction_valid() {
        let valid_instruction = "cmd ls";

        assert!(validate_instruction(valid_instruction).is_ok());
    }

    #[test]
    fn test_validate_instruction_invalid() {
        let valid_instruction = "aaa ls";

        assert!(validate_instruction(valid_instruction).is_err());
    }

    #[test]
    fn test_instructions_factory_all_valid() {
        let source_instruction = "cmd ls
set a 1
";
        let expected_instructions = vec!["cmd ls", "set a 1"];

        let instructions = instructions_factory(source_instruction);

        assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn test_instructions_factory_all_invalid() {
        let source_instruction = "fnif ls
rat a 1
";
        let expected_instructions: Vec<&str> = Vec::new();

        let instructions = instructions_factory(source_instruction);

        assert_eq!(instructions, expected_instructions);
    }
}
