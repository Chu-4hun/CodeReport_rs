use clap::Parser;
use code_report_rs::ReportGen;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = false)]
struct Args {
    /// Расширения файлов. Пример code_report_rs rs js
    file_extensions: Vec<String>,

    ///Использовать расширения файлов по умолчанию rs, go cs toml java html   
    ///Вы так же можете добавить свои расширения файлов[FILE_EXTENSIONS]...
    ///Пример: code_report_rs --use_defaut_values shtml css
    #[arg(short, long, default_value_t = false)]
    use_defaut_values: bool,
}

fn main() -> Result<(), std::io::Error> {
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

    ReportGen::new_from_path("./", &file_extensions)
        .unwrap()
        .save_file("./report.docx")
}
