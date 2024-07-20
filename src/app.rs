use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

#[derive(Debug, Default)]
pub struct App {
    pub readonly: bool,
    pub file_paths: Vec<String>,
    pub buffers: Vec<File>,
    pub active_file: Option<usize>,
}

impl App {
    pub fn new() -> Self {
        App {
            readonly: false,
            file_paths: vec![],
            buffers: vec![],
            active_file: None,
        }
    }

    pub fn open_file(&mut self, file_index: usize) -> std::io::Result<()> {
        self.active_file = Some(file_index);
        let file_path = self
            .file_paths
            .get(file_index)
            .expect("Unable to open the file");

        let file = File::open(file_path)?;

        self.buffers.push(file);

        Ok(())
    }

    pub fn get_active_file_content(&mut self) -> std::io::Result<Vec<String>> {
        if self.active_file.is_none() {
            if self.file_paths.is_empty() {
                eprintln!("No active files");
                exit(1);
            }
            self.active_file = Some(0);
            let _ = self.open_file(0);
        }

        let file = self.buffers.get(self.active_file.unwrap()).unwrap();
        let buffer = BufReader::new(file);
        let content: Vec<String> = buffer.lines().map(|x| x.unwrap()).collect();

        Ok(content)
    }
}
