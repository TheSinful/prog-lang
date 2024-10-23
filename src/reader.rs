use std::{env, io::Read, path::PathBuf};

use crate::processing::parser::Parser;

pub struct Arguments {
    pub program_name: String,
    pub file: PathBuf,
}

pub struct CLI {
    pub args: Arguments,
}

impl CLI {
    pub fn get_args(&mut self) -> () {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            println!("Unexpected arguments!")
        } else if args.len() == 0 {
            println!("Expected a file path!")
        }

        let arguments = self.args = Arguments {
            program_name: args[0].clone(),
            file: PathBuf::from(args[1].clone()),
        };
    }

    pub fn file_exists(&self) -> bool {
        self.args.file.exists()
    }

    pub fn read_file(&self) -> std::io::Result<String> {
        let mut file = std::fs::File::open(&self.args.file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let processed_contents = contents
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join(" \n ");

        Ok(processed_contents)
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.get_args();

        if !self.file_exists() {
            println!("File does not exist at this path!");
            return Ok(());
        }

        let body = self.read_file()?;
        let mut parser = Parser::default();
        parser.parse(&body);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    fn setup_test_file(content: &str) -> PathBuf {
        let path = PathBuf::from("test_file.txt");
        let mut file = File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        path
    }

    #[test]
    fn test_get_args() {
        // Set the environment variables to simulate command-line arguments
        let args = vec!["program_name".to_string(), "test_file.txt".to_string()];
        env::set_var("CARGO_BIN_EXE_test", &args[0]);
        env::set_var("CARGO_BIN_EXE_test_file", &args[1]);

        // Temporarily override the command-line arguments
        let original_args = env::args().collect::<Vec<_>>();
        env::set_var("CARGO_BIN_EXE_test", &args[0]);
        env::set_var("CARGO_BIN_EXE_test_file", &args[1]);

        let mut cli = CLI {
            args: Arguments {
                program_name: String::new(),
                file: PathBuf::new(),
            },
        };

        // Simulate the command-line arguments
        env::set_var("CARGO_BIN_EXE_test", &args[0]);
        env::set_var("CARGO_BIN_EXE_test_file", &args[1]);

        cli.get_args();

        assert_eq!(cli.args.program_name, "program_name");
        assert_eq!(cli.args.file, PathBuf::from("test_file.txt"));

        // Restore the original command-line arguments
        env::set_var("CARGO_BIN_EXE_test", &original_args[0]);
        env::set_var("CARGO_BIN_EXE_test_file", &original_args[1]);
    }
    #[test]
    fn test_file_exists() {
        let path = setup_test_file("test content");
        let cli = CLI {
            args: Arguments {
                program_name: "test_program".to_string(),
                file: path.clone(),
            },
        };

        assert!(cli.file_exists());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_read_file() {
        let path = setup_test_file("line 1\nline 2\nline 3");
        let cli = CLI {
            args: Arguments {
                program_name: "test_program".to_string(),
                file: path.clone(),
            },
        };

        let contents = cli.read_file().unwrap();
        assert_eq!(contents, "line 1 \n line 2 \n line 3");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_run() {
        let path = setup_test_file("line 1\nline 2\nline 3");
        let mut cli = CLI {
            args: Arguments {
                program_name: "test_program".to_string(),
                file: path.clone(),
            },
        };

        let result = cli.run();
        assert!(result.is_ok());

        std::fs::remove_file(path).unwrap();
    }
}
