use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "cxxd")]
#[command(author = "Ebooth <pauldejeandev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "A copy of unix command line tool xxd", long_about = None)]
pub struct Args {
    file: String,
}

pub fn execute_command(args: &Args) -> i32 {
    let file = match std::fs::read(&args.file) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return 1;
        }
    };

    let hex_dump = create_hex_dump(file);
    println!("{}", hex_dump.join("\n"));
    return 0;
}

fn create_hex_dump(file: Vec<u8>) -> Vec<String> {
    let blocks: Vec<&[u8]> = file.chunks(16).collect();
    let mut hex_dump: Vec<String> = Vec::new();
    for (index, block) in blocks.into_iter().enumerate() {
        let hex_string = get_block_hex_string(block);
        let ascii_value = get_block_ascii_value(block);
        hex_dump.push(format!("{index:08x}: {hex_string} {ascii_value}"));
    }
    hex_dump
}

fn get_block_hex_string(block: &[u8]) -> String {
    let hex_block = block
        .iter()
        .map(|byte| byte_to_hex_string(*byte))
        .collect::<Vec<String>>();

    let hex_string: Vec<String> = hex_block.chunks(2).map(|chunk| chunk.join("")).collect();
    return hex_string.join(" ");
}

fn get_block_ascii_value(block: &[u8]) -> String {
    let ascii_value: String = block
        .iter()
        .map(|byte| {
            if *byte >= 32 && *byte <= 126 {
                *byte as char
            } else {
                '.'
            }
        })
        .collect();
    ascii_value
}

fn byte_to_hex_string(byte: u8) -> String {
    format!("{:02x}", byte)
}
