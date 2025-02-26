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

    #[arg(short = 's', long = "seek", default_value_t = 0)]
    seek: i32,
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
        seek: args.seek,
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
    seek: i32,
}

fn create_hex_dump(file: &[u8], options: &HexDumpOptions) -> Vec<String> {
    let offset = if options.seek >= 0 {
        options.seek as usize
    } else {
        file.len().saturating_sub((-options.seek) as usize)
    };

    let left_trimmed_data: &[u8] = file.get(offset..).unwrap_or(&[]);

    let trimmed_data = options.length.map_or(left_trimmed_data, |len| {
        left_trimmed_data.get(..len).unwrap_or(left_trimmed_data)
    });

    trimmed_data
        .chunks(options.cols)
        .enumerate()
        .map(|(index, block)| {
            let byte_offset = index * options.cols + offset;
            let hex_string = get_block_hex_string(block, options);
            let ascii_value = get_block_ascii_value(block);
            format!("{byte_offset:08x}: {hex_string}  {ascii_value}")
        })
        .collect()
}

fn get_block_hex_string(block: &[u8], options: &HexDumpOptions) -> String {
    let group_size = if options.group_size == 0 {
        16
    } else {
        options.group_size
    };

    let expected_groups = options.cols / group_size;
    let width = group_size * 2;

    let mut groups: Vec<String> = block
        .chunks(group_size)
        .map(|group| {
            let hex_string: String = if options.little_endian {
                group.iter().rev().map(|&b| byte_to_hex_string(b)).collect()
            } else {
                group.iter().map(|&b| byte_to_hex_string(b)).collect()
            };
            format!("{:<width$}", hex_string, width = width)
        })
        .collect();

    groups.resize(expected_groups, format!("{:<width$}", "", width = width));
    groups.join(" ")
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
