use std::{fs, path::PathBuf};

use clap::Parser;
use docx_rs::{
    Docx, DocxError, IndentLevel, NumberingId, Paragraph, Run, Table, TableCell, TableRow,
};
use glob::glob;

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
    let mut paths: Vec<PathBuf> = vec![];

    for file_extension in file_extensions {
        for entry in glob(&("./**/**/**/**/**/**/**/*.".to_owned() + &file_extension))
            .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    println!("{:?}", path.to_str().unwrap());
                    paths.push(path);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
    doc.gen_table(&paths).expect("Error generating table");
    doc.gen_body_with_list(&paths)
        .expect("Error generating body");
    doc.save_in_file(&String::from("./report.docx"))
        .expect("Error saving file");
}

trait GenFile {
    fn gen_table(&mut self, inputs: &Vec<PathBuf>) -> Result<(), DocxError>;
    fn gen_body_with_list(&mut self, inputs: &Vec<PathBuf>) -> Result<(), DocxError>;
    fn gen_body(&mut self, input_path: PathBuf) -> Result<(), DocxError>;
    fn save_in_file(&self, path: &String) -> Result<(), std::io::Error>;
}
impl GenFile for Docx {
    fn gen_table(&mut self, inputs: &Vec<PathBuf>) -> Result<(), DocxError> {
        let mut table = Table::new(vec![TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Имя файла").size(12*2))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Количество строк кода").size(12*2))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Размер (Кбайт)").size(12*2))),
        ])]);

        for path in inputs {
            println!("{}", path.as_os_str().to_str().unwrap());
            table =
                table.add_row(TableRow::new(vec![
                    TableCell::new().add_paragraph(
                        Paragraph::new()
                            .add_run(Run::new().add_text(path.as_os_str().to_str().unwrap()).size(12*2)),
                    ),
                    TableCell::new().add_paragraph(
                        Paragraph::new()
                            .add_run(Run::new().add_text(get_file_text(&path).len().to_string()).size(12*2)),
                    ),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(
                        Run::new().add_text(format!("{:.2}",fs::metadata(path).unwrap().len()/1024)).size(12*2),
                    )),
                ]));
        }
        *self = self.to_owned().add_table(table);
        Ok(())
    }
    fn gen_body_with_list(&mut self, inputs: &Vec<PathBuf>) -> Result<(), DocxError> {
        for input in inputs {
            *self = self.to_owned().add_paragraph(
                Paragraph::new()
                    .add_run(
                        Run::new()
                            .add_text(input.as_path().to_str().unwrap())
                            .size(16 * 2),
                    )
                    .numbering(NumberingId::new(2), IndentLevel::new(0))
                    .size(16 * 2),
            );
            let lines: Vec<String> = get_file_text(&input);
            for line in lines {
                *self = self
                    .to_owned()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
            }
        }
        Ok(())
    }

    fn gen_body(&mut self, input_path: PathBuf) -> Result<(), DocxError> {
        *self = self.to_owned().add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(input_path.as_path().to_str().unwrap())
                        .size(16 * 2),
                )
                .numbering(NumberingId::new(2), IndentLevel::new(0))
                .size(16 * 2),
        );
        let lines: Vec<String> = get_file_text(&input_path);
        for line in lines {
            *self = self
                .to_owned()
                .add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
        }
        Ok(())
    }

    fn save_in_file(&self, input_path: &String) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(input_path);
        let file = fs::File::create(path).unwrap_or(fs::File::open(path)?);
        self.to_owned().build().pack(file)?;
        Ok(())
    }
}

fn get_file_text(input_path: &PathBuf) -> Vec<String> {
    fs::read_to_string(input_path)
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .collect()
}
