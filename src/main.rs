use std::fs;

use clap::Parser;
use glob::glob;

//TODO file parser
//TODO Doc model
//TODO Doc generation

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Расширения файлов. Пример code_report_rs rs js
    file_extensions: Vec<String>,

    ///Использовать расширения файлов по умолчанию rs, go cs toml java html   
    ///Вы так же можете добавить свои рассширения фалйов[FILE_EXTENSIONS]...
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

    for file_extension in file_extensions {
        for entry in glob(&("./**/**/**/**/**/**/**/*.".to_owned() + &file_extension))
            .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => println!("{:?}", fs::read_to_string(path)),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
