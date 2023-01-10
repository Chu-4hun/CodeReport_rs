use std::{fs, path::PathBuf};

use clap::Parser;
use docx_rs::{Docx, DocxError, IndentLevel, NumberingId, Paragraph, Run};
use glob::glob;

//TODO Doc model
//TODO Doc generation

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Расширения файлов. Пример code_report_rs rs js
    file_extensions: Vec<String>,

    ///Использовать расширения файлов по умолчанию rs, go cs toml java html   
    ///Вы так же можете добавить свои расширения файлов[FILE_EXTENSIONS]...
    ///Пример: code_report_rs --use_defaut_values shtml css
    #[arg(short, long, default_value_t = false)]
    use_defaut_values: bool,
}

fn main() {
    let args = Args::parse();
    let mut file_extensions = vec![
        String::from("rs"),
        String::from("go"),
        String::from("cs"),
        String::from("toml"),
        String::from("java"),
        String::from("html"),
    ];

    if !args.file_extensions.is_empty() && !args.use_defaut_values {
        file_extensions = args.file_extensions;
    } else if args.use_defaut_values {
        for extension in args.file_extensions {
            file_extensions.push(extension);
        }
        file_extensions.dedup();
    }
    let mut doc = Docx::new();
    for file_extension in file_extensions {
        for entry in glob(&("./**/**/**/**/**/**/**/*.".to_owned() + &file_extension))
            .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    println!("{:?}", path.to_str().unwrap());
                    println!("{:?}", gen_file(path, &mut doc));
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
    let path = std::path::Path::new("./numbering.docx");
    let file = fs::File::create(path).unwrap_or(fs::File::open(path).unwrap());
    doc.to_owned().build().pack(file).unwrap();
}

fn gen_file(input_path: PathBuf, doc: &mut Docx) -> Result<(), DocxError> {
    // let path = std::path::Path::new("./numbering.docx");
    // let file = fs::File::open(path).unwrap_or(fs::File::create(path).unwrap());
    *doc = doc.to_owned().add_paragraph(
        Paragraph::new()
            .add_run(Run::new().add_text(input_path.as_path().to_str().unwrap()))
            .numbering(NumberingId::new(2), IndentLevel::new(0)),
    );
    let lines: Vec<String> = fs::read_to_string(input_path)
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .collect();
    for line in lines {
        *doc = doc
            .to_owned()
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
    }

    Ok(())
}
