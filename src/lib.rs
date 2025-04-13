use std::{
    fs,
    path::{Path, PathBuf},
};

use docx_rs::{
    Docx, DocxError, IndentLevel, NumberingId, Paragraph, Run, Table, TableCell, TableRow,
};
use walkdir::WalkDir;

#[derive(Default, Debug)]
pub struct ReportGen {
    doc: Docx,
}

impl ReportGen {
    pub fn new(doc: Docx) -> Self {
        Self { doc }
    }
    pub fn new_from_path(
        path: impl AsRef<Path>,
        file_extensions: &[String],
    ) -> Result<Self, walkdir::Error> {
        let mut doc = Docx::new();
        let mut paths: Vec<PathBuf> = vec![];

        for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if e.metadata()?.is_file()
                && file_extensions.contains(
                    &e.path()
                        .extension()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
            {
                println!("{}", e.path().display());
                paths.push(e.path().to_path_buf());
            }
        }
        doc.gen_table(&paths).expect("Error generating table");
        doc.gen_body_with_list(&paths)
            .expect("Error generating body");
        Ok(Self { doc })
    }

    pub fn save_file(self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        self.doc.save_in_file(path)
    }
}

trait GenFile {
    fn gen_table(&mut self, inputs: &[PathBuf]) -> Result<(), DocxError>;
    fn gen_body_with_list(&mut self, inputs: &[PathBuf]) -> Result<(), DocxError>;
    #[allow(dead_code)]
    fn gen_body(&mut self, input_path: PathBuf) -> Result<(), DocxError>;
    fn save_in_file<T: AsRef<Path>>(&self, path: T) -> Result<(), std::io::Error>;
}
impl GenFile for Docx {
    fn gen_table(&mut self, inputs: &[PathBuf]) -> Result<(), DocxError> {
        let mut table = Table::new(vec![TableRow::new(vec![
            TableCell::new().add_paragraph(
                Paragraph::new().add_run(Run::new().add_text("Имя файла").size(12 * 2)),
            ),
            TableCell::new().add_paragraph(
                Paragraph::new().add_run(Run::new().add_text("Количество строк кода").size(12 * 2)),
            ),
            TableCell::new().add_paragraph(
                Paragraph::new().add_run(Run::new().add_text("Размер (Кбайт)").size(12 * 2)),
            ),
        ])]);

        for path in inputs {
            println!("{}", path.as_os_str().to_str().unwrap());
            table = table.add_row(TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(path.as_os_str().to_str().unwrap())
                            .size(12 * 2),
                    ),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(get_file_text(path).len().to_string())
                            .size(12 * 2),
                    ),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(format!("{:.2}", fs::metadata(path).unwrap().len() / 1024))
                            .size(12 * 2),
                    ),
                ),
            ]));
        }
        *self = self.to_owned().add_table(table);
        Ok(())
    }
    fn gen_body_with_list(&mut self, inputs: &[PathBuf]) -> Result<(), DocxError> {
        for input in inputs {
            let code_cell = TableCell::new().width(10000, docx_rs::WidthType::Dxa);
            let line = fs::read_to_string(input).expect("Error reading file");
            let lines: Vec<&str> = line.split('\n').collect();

            // Add initial paragraph with file path
            *self = self.to_owned().add_paragraph(
                Paragraph::new()
                    .add_run(
                        Run::new()
                            .add_text(input.as_path().to_str().unwrap())
                            .size(14 * 2),
                    )
                    .numbering(NumberingId::new(2), IndentLevel::new(0))
                    .size(16 * 2),
            );
            let mut p = Paragraph::new();
            for line in lines {
                p = p
                    .add_run(
                        Run::new()
                            .add_text(line)
                            .add_break(docx_rs::BreakType::TextWrapping)
                            .size(12 * 2),
                    )
                    .size(16 * 2)
            }

            let code_cell = code_cell.add_paragraph(p);

            let table = Table::new(vec![TableRow::new(vec![code_cell])]);
            *self = self.to_owned().add_table(table);
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

        Ok(())
    }

    fn save_in_file<T: AsRef<Path>>(&self, path: T) -> Result<(), std::io::Error> {
        let file = fs::File::create(&path).unwrap_or(fs::File::open(&path)?);
        self.to_owned().build().pack(file)?;
        Ok(())
    }
}

fn get_file_text(input_path: &PathBuf) -> Vec<String> {
    fs::read_to_string(input_path)
        .unwrap()
        .split('\n')
        .map(str::to_string)
        .collect()
}
