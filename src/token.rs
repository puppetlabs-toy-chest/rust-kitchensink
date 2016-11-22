use std::io::{self, Read};
use std::fs::File;
use utils;

/// Reads the contents of a token file from the input path.
pub fn read_token(path: String) -> io::Result<String> {
    let mut f = try!(File::open(&path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s.trim().to_owned())
}

/// Given a `home_dir` (e.g. from `std::env::home_dir()`), returns the default
/// location of the token, `$HOME/.puppetlabs/token`.
pub fn default_token_path() -> String {
    let mut home_dir = utils::home_dir();
    home_dir.push(".puppetlabs");
    home_dir.push("token");
    home_dir.to_str().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Write, Error};
    use std::path::PathBuf;

    extern crate tempdir;
    use self::tempdir::*;

    fn create_temp_path(temp_dir: &TempDir, file_name: &str) -> PathBuf {
        temp_dir.path().join(file_name)
    }

    fn spit_token(file_path: &str, token_content: &str) -> Result<(), Error> {
        let mut f = try!(File::create(file_path));
        try!(f.write_all(token_content.as_bytes()));
        Ok(())
    }

    #[test]
    fn it_works() {
        let temp_dir = TempDir::new_in("target", "test-").unwrap();
        let temp_path = create_temp_path(&temp_dir, "testtoken");
        let path_str = temp_path.as_path().to_str().unwrap();

        spit_token(path_str, "fkgjh95 ghdlfjgh   ").unwrap();
        assert_eq!("fkgjh95 ghdlfjgh", read_token(path_str.to_string().clone()).unwrap());
    }
}
