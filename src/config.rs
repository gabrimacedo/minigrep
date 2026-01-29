use std::{env, path::PathBuf};

#[derive(Debug, PartialEq)]
pub enum Source {
    File(PathBuf),
    Directory(PathBuf),
    Stdin,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub source: Source,
    pub pattern: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String], has_sdtin: bool) -> Result<Self, String> {
        let pattern = args.get(1).ok_or("Search string not specified")?;

        let source = match args.get(2) {
            Some(path) => {
                let path = PathBuf::from(path);
                if path.is_dir() {
                    Source::Directory(path)
                } else {
                    Source::File(path)
                }
            }
            None => {
                if has_sdtin {
                    Source::Stdin
                }
                // last case, use current dir
                else {
                    Source::Directory(env::current_dir().unwrap())
                }
            }
        };

        Ok(Config { pattern, source })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::temp_dir;

    fn args(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn src_is_file() {
        let dir = temp_dir();
        let file = dir.as_path().join("test.txt");
        let a = args(&["minigrep", "find this", file.to_str().unwrap()]);
        let config = Config::build(&a, false).unwrap();

        assert_eq!("find this", config.pattern);
        assert_eq!(Source::File(file.clone()), config.source);

        let config = Config::build(&a, true).unwrap();

        assert_eq!("find this", config.pattern);
        assert_eq!(Source::File(file), config.source)
    }

    #[test]
    fn src_is_directory() {
        // NO STDIN AND DIR PATH
        let dir = temp_dir();
        let path = dir.into_os_string().into_string().unwrap();
        let args = [
            String::from("minigrep"),
            String::from("find this"),
            path.clone(),
        ];
        let config = Config::build(&args, false).unwrap();

        assert_eq!("find this", config.pattern);
        assert_eq!(
            Source::Directory(PathBuf::from(path.clone())),
            config.source
        );

        // STDIN AND DIR PATH
        let config = Config::build(&args, true).unwrap();

        assert_eq!("find this", config.pattern);
        assert_eq!(
            Source::Directory(PathBuf::from(path.clone())),
            config.source
        );
    }

    #[test]
    fn src_is_stdin() {
        // no path or file argument
        let args = [String::from("minigrep"), String::from("he")];
        let has_sdtin = true;
        let config = Config::build(&args, has_sdtin).unwrap();

        assert_eq!("he", config.pattern);
        assert_eq!(Source::Stdin, config.source);
    }

    #[test]
    fn fall_back_source() {
        // STDIN AND NO PATH ARGUMENT
        let curr_dir = env::current_dir().unwrap();
        let args = [String::from("minigrep"), String::from("find this")];
        let config = Config::build(&args, false).unwrap();

        assert_eq!("find this", config.pattern);
        assert_eq!(Source::Directory(curr_dir), config.source)
    }
}
