use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "cxxd")]
#[command(author = "Ebooth <pauldejeandev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "A copy of unix command line tool xxd", long_about = None)]
pub struct Args {
    file: String,

    #[arg(short = 'e')]
    little_endian: bool,

    #[arg(
        short = 'g',
        long = "group_size",
        default_value_t = 2,
        default_value_if("little_endian", "true", "4")
    )]
    group_size: usize,

    #[arg(short = 'l', long = "len")]
    length: Option<usize>,

    #[arg(short = 'c', long = "cols", default_value_t = 16)]
    cols: usize,
}

pub fn execute_command(args: &Args) -> i32 {
    let file = match std::fs::read(&args.file) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return 1;
        }
    };

    let options = HexDumpOptions {
        little_endian: args.little_endian,
        group_size: args.group_size,
        cols: args.cols,
        length: args.length,
    };

    let hex_dump = create_hex_dump(&file, &options);
    println!("{}", hex_dump.join("\n"));
    return 0;
}

struct HexDumpOptions {
    little_endian: bool,
    group_size: usize,
    cols: usize,
    length: Option<usize>,
}

fn create_hex_dump(file: &Vec<u8>, options: &HexDumpOptions) -> Vec<String> {
    let data = if let Some(len) = options.length {
        &file[..len]
    } else {
        &file
    };

    data.chunks(options.cols)
        .enumerate()
        .map(|(index, block)| {
            let hex_string = get_block_hex_string(block, options);
            let ascii_value = get_block_ascii_value(block);
            format!("{index:08x}: {hex_string} {ascii_value}")
        })
        .collect()
}

fn get_block_hex_string(block: &[u8], options: &HexDumpOptions) -> String {
    let group_size = if options.group_size == 0 {
        16
    } else {
        options.group_size
    };

    block
        .chunks(group_size)
        .map(|group| {
            if options.little_endian {
                group
                    .iter()
                    .rev()
                    .map(|&b| byte_to_hex_string(b))
                    .collect::<String>()
            } else {
                group
                    .iter()
                    .map(|&b| byte_to_hex_string(b))
                    .collect::<String>()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
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
