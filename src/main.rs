use std::env;
use std::process::exit;

use haders;

fn main() {
    // Get command line arguments
    let args = env::args().collect::<Vec<String>>();

    // Check correct number of argument
    if args.len() != 4 {
        print_program_info();
    }

    let mode = &args[1];
    let decompressed_size = haders::size_parser(&args[2]);
    let out_zip_file = &args[3];

    // Correctness of args
    if (mode != "flat" && mode != "nested")
        || decompressed_size.is_err()
        || !out_zip_file.ends_with(".zip")
    {
        print_program_info();
    }

    let time_start = std::time::Instant::now();

    // Runs chosen zip bomb generator and save the decompressed size
    let total_size = match mode.as_str() {
        "flat" => haders::make_flat_zip_bomb(&out_zip_file, decompressed_size.clone().unwrap()),
        "nested" => haders::make_nested_zip_bomb(&out_zip_file, decompressed_size.clone().unwrap()),
        _ => exit(1),
    };

    let time_stop = time_start.elapsed();

    // Zip bomb compressed size
    let compressed_size = haders::check_file_size(&out_zip_file);

    // Prints zip bomb info
    println!(
        "Compressed File Size: {}",
        if compressed_size >= 1024 {
            format!("{:.2} MB", compressed_size as f32 / 1024f32)
        } else {
            format!("{} KB", compressed_size)
        }
    );
    println!("Size After Decompression: {} GB", total_size);
    println!("Generation Time: {:.2?}", time_stop);
}

/// Prints cli app info, how to use it?
fn print_program_info() {
    println!("CLI app that generates zip bombs for you");

    println!();
    println!("<executable> <mode> <size> <out_zip_file>");
    println!("  ./haders    flat   10gb     hehe.zip");
    println!();

    println!("<mode> - mode of compression");
    println!("  nested - nested zip file (zip file of zip files of ...)");
    println!("  flat   - flat file without nested zips");
    println!("<size> - decompression size (supported units: GB, TB, PB, EB)");
    println!("<out_zip_file> - path to destination file");

    exit(0);
}
