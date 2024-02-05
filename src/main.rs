use std::path::PathBuf;

mod cli;


fn add_extension(path: &mut std::path::PathBuf, extension: impl AsRef<std::path::Path>) {
    match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();
            ext.push(".");
            ext.push(extension.as_ref());
            path.set_extension(ext)
        }
        None => path.set_extension(extension.as_ref()),
    };
}

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(config_path) = matches.get_one::<PathBuf>("FILE_NAME") {
        // ensure FILE_NAME.old exists and is a file, create it if it doesn't

        let mut old_file = config_path.clone();
        add_extension(&mut old_file, "old");

        if !old_file.exists() {
            println!("Creating {}", old_file.display());
            std::fs::write(&old_file, "").expect("Could not create .old file");
        }
        // now we swap the contents of FILE_NAME and FILE_NAME.old
        let temp_file = config_path.with_extension("tmp");
        std::fs::rename(&config_path, &temp_file).expect("Could not rename file");
        std
            ::fs::rename(&old_file, &config_path)
            .expect("Could not rename .old file");
        std
            ::fs::rename(&temp_file, &old_file)
            .expect("Could not rename temp file");



    }
}

