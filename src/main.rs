use clap::{Arg, Command};
use std::io::{self, BufRead};
use cached::proc_macro::cached;

// Caching the alignment length calculation
#[cached(size = 100)]
fn calculate_alignment_length(cigar: String) -> f64 {
    let mut total_length = 0.0;
    let mut num_part = String::new();

    for c in cigar.chars() {
        if c.is_ascii_digit() {
            num_part.push(c);
        } else {
            if let Ok(length) = num_part.parse::<f64>() {
                if "MID".contains(c) { // Only consider M, I, D for the length
                    total_length += length;
                }
            }
            num_part.clear();
        }
    }

    total_length
}

fn calculate_identity(total_length: f64, nm: u32) -> f64 {
    if total_length > 0.0 {
        (total_length - nm as f64) / total_length
    } else {
        0.0 // Avoid division by zero
    }
}

fn main() {
    let matches = Command::new("Cigar Filter")
        .version("1.0")
        .about("Filters CIGAR strings by sequence identity and alignment ratio in SAM files")
        .arg(
            Arg::new("min_identity")
                .long("identity")
                .short('i')
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Minimum matching identity as a float"),
        )
        .arg(
            Arg::new("min_ratio")
                .long("ratio")
                .short('r')
                .value_parser(clap::value_parser!(f64))
                .required(true)
                .help("Minimum query alignment ratio as a float"),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .action(clap::ArgAction::SetTrue)
                .help("If used, print lines where identity <= min_identity (ignoring ratio), otherwise print lines where identity > min_identity and ratio > min_ratio"),
        )
        .get_matches();

    let min_identity: f64 = *matches.get_one::<f64>("min_identity").unwrap();
    let min_ratio: f64 = *matches.get_one::<f64>("min_ratio").unwrap();
    let reverse = matches.get_flag("reverse");

    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() > 5 {
            if let Some(nm_tag) = parts.iter().find(|&x| x.starts_with("NM:i:")) {
                if let Ok(nm) = nm_tag[5..].parse::<u32>() {
                    let cigar = parts[5].to_string();
                    let total_length = calculate_alignment_length(cigar);
                    let identity = calculate_identity(total_length, nm);
                    let query_length = parts[9].len() as f64;
                    let ratio = total_length / query_length;

                    if reverse {
                        // If reverse is used, print if identity <= min_identity
                        if identity <= min_identity {
                            println!("{}", line);
                        }
                    } else {
                        // If reverse is not used, print if identity > min_identity and ratio > min_ratio
                        if identity > min_identity && ratio > min_ratio {
                            println!("{}", line);
                        }
                    }
                }
            }
        }
    }
}
