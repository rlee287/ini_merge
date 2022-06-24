use std::env::args_os;
use std::process::ExitCode;
use std::fs::File;

use ini_merge::merge_ini;

use ini::Ini;

fn main() -> ExitCode {
    let args: Vec<_> = args_os().collect();
    if args.len() != 4 {
        // This includes -h and --help
        eprintln!("Usage: ini_merge base_file patch_file output_file");
        eprintln!("Keys and values in patch_file will overwrite ones in base_file");
        return ExitCode::from(2);
    }
    let mut out_file = match File::options().write(true).create_new(true).open(&args[3]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening output_file: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let base_conf = match Ini::load_from_file(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading base_file: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let patch_conf = match Ini::load_from_file(&args[2]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading patch_file: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let out_conf = merge_ini(&base_conf, &patch_conf);

    out_conf.write_to(&mut out_file).unwrap();
    return ExitCode::SUCCESS;
}
