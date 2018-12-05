pub mod exec {
    use kinds::*;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::iter::Iterator;

    pub fn run(cfg: Config) -> Result<(), Box<Error>> {
        for file in cfg.files {
            let mut f = File::open(file)?;
            let mut buffer = String::new();
            f.read_to_string(&mut buffer)?;
            for line in maybe_number(cfg.number, &buffer) {
                println!("{}", line)
            }
        }

        Ok(())
    }
    /// Breaks the contents into lines, possibly numbering them first
    ///
    /// # Examples
    ///
    /// ```
    /// use kitten::exec::maybe_number;
    /// let letters = "a\nb\nc\n";
    ///
    /// assert_eq!(vec!["1 a", "2 b", "3 c"], maybe_number(true, letters));
    /// ```
    pub fn maybe_number(number: bool, contents: &str) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for (i, line) in contents.lines().enumerate() {
            if number {
                output.push(format!("{} {}", i + 1, line))
            } else {
                output.push(String::from(line))
            }
        }
        output
    }
}

pub mod kinds {

    pub struct Config {
        pub number: bool,
        pub files: Vec<String>,
    }

    impl Config {
        /// Creates a new configuration from the given args
        /// if the first argument is "-n", output will be numbered.
        /// all other input will be read as filenames.
        ///
        /// # Examples
        ///
        /// ```
        /// use kitten::kinds::Config;
        /// let args = vec!["-n".to_string(), "file1".into(), "file2".into()];
        ///
        /// let cfg = Config::new(&mut args.into_iter()).unwrap();
        /// assert_eq!(cfg.number, true);
        /// assert_eq!(cfg.files, vec!["file1", "file2"]);
        /// ```
        pub fn new<I>(vals: I) -> Result<Config, &'static str>
        where
            I: IntoIterator<Item = String>,
        {
            let mut iter = vals.into_iter();
            let mut cfg = Config {
                number: false,
                files: Vec::new(),
            };

            if let Some(file) = iter.next() {
                if file == "-n" {
                    cfg.number = true;
                } else {
                    cfg.files.push(file);
                }
            }
            cfg.files.extend(iter);
            Ok(cfg)
        }
    }
}

#[cfg(test)]
mod test {
    use exec::*;

    static CONTENTS: &str = "\
Whose woods these are,
I think I know.
His house is in the village, though.";

    #[test]
    fn number_lines() {
        assert_eq!(
            vec![
                "1 Whose woods these are,",
                "2 I think I know.",
                "3 His house is in the village, though."
            ],
            maybe_number(true, CONTENTS),
        );
    }

    #[test]
    fn unnumbered_lines() {
        assert_eq!(
            vec![
                "Whose woods these are,",
                "I think I know.",
                "His house is in the village, though."
            ],
            maybe_number(false, CONTENTS),
        )
    }
}
