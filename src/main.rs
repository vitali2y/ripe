// ripe - Rust pipe editor

use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
    process::{self, Command, ExitStatus},
};
use tempfile::NamedTempFile;

fn get_editor() -> String {
    env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| String::from("vi"))
}

fn run_editor(editor: &str, path: &std::path::Path) -> io::Result<ExitStatus> {
    Command::new(editor).arg(path).status()
}

fn read_file_to_vec(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // if single CLI argument (optional) is present then it's template file as prefix for content
    let template_path = if args.len() > 1 {
        Some(PathBuf::from(&args[1]))
    } else {
        None
    };

    let mut tmp = NamedTempFile::new()?;

    if let Some(path) = template_path {
        let template = read_file_to_vec(&path).unwrap_or_else(|e| {
            eprintln!("ripe: cannot read template {}: {}", path.display(), e);
            process::exit(2);
        });

        tmp.write_all(&template)?;
        tmp.write_all(b"\n\n")?;
    }

    // if let Some(path) = template_path {
    //     let template = read_file_to_vec(&path)?;
    //     tmp.write_all(&template)?;
    //     tmp.write_all(b"\n\n")?;
    // }

    io::copy(&mut io::stdin(), &mut tmp)?;
    tmp.flush()?;
    let path = tmp.path().to_owned();

    let editor = get_editor();
    let status = run_editor(&editor, &path)?;

    if !status.success() {
        eprintln!("editor exited with non-zero status: {}", status);
        std::process::exit(status.code().unwrap_or(1));
    }

    let mut output = Vec::new();
    File::open(&path)?.read_to_end(&mut output)?;
    io::stdout().write_all(&output)?;

    Ok(())
}
