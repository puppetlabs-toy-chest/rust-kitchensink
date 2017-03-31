use hyper;
use std::io::{Read, Write};
use std::env;
use std::path::PathBuf;

#[cfg(windows)]
use windows;

/// Like `println!` but for stderr.
#[macro_export]
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

/// Like `panic!` but with prettier output.
#[macro_export]
macro_rules! pretty_panic(
    ($($arg:tt)*) => (
        {
            println_stderr!($($arg)* );
            ::std::process::exit(1)
        }
    )
);

/// Type alias for the result of a hyper HTTP request.
pub type HyperResponse = hyper::client::response::Response;
pub type HyperError = hyper::error::Error;
pub type HyperResult = Result<HyperResponse, HyperError>;

/// Exits with an error if the response did not have status 200.
pub fn assert_status_ok(response: &mut HyperResponse) {
    if response.status != hyper::Ok {
        let mut temp = String::new();
        if let Err(x) = response.read_to_string(&mut temp) {
            panic!("Unable to read response from server: {}", x);
        }
        pretty_panic!("Error response {} from server: {}", response.status, temp)
    }
}

#[cfg(windows)]
pub fn home_dir() -> PathBuf {
    env::remove_var("HOME".to_string());
    env::home_dir().expect("%USERPROFILE% directory is not configured")
}

#[cfg(windows)]
pub fn puppetlabs_dir() -> PathBuf {
    let mut path = windows::get_special_folder(&windows::FOLDERID_ProgramData).unwrap();
    path.push("PuppetLabs");
    return path;
}

#[cfg(not(windows))]
pub fn home_dir() -> PathBuf {
    env::home_dir().expect("$HOME directory is not configured")
}

#[cfg(not(windows))]
pub fn puppetlabs_dir() -> PathBuf {
    PathBuf::from("/etc/puppetlabs")
}

pub fn global_client_tools_dir() -> PathBuf {
    let mut path = puppetlabs_dir();
    path.push("client-tools");
    return path;
}

pub fn local_client_tools_dir() -> PathBuf {
    let mut conf_dir = home_dir();
    conf_dir.push(".puppetlabs");
    conf_dir.push("client-tools");
    return conf_dir;
}

pub trait NotEmpty<T> {
    fn not_empty(self) -> T;
}

impl NotEmpty<Option<String>> for Option<String> {
    fn not_empty(self) -> Option<String> {
        self.and_then(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
    }
}

pub fn default_certificate_file() -> String {
    let mut path = puppetlabs_dir();
    path.push("puppet");
    path.push("ssl");
    path.push("certs");
    path.push("ca");
    path.set_extension("pem");
    path.to_str().unwrap().to_owned()
}
