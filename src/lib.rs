use std::io;
use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::io::stdout;
use std::io::stdin;
use std::path::Path;


pub type IOResult = Result<String, io::Error>;

pub fn prompt(p: &str) -> IOResult {
    print!("{}", p);
    let r = stdout().flush();
    if r.is_err() {
        return Err(r.unwrap_err())
    }

    let mut buf = String::new();
    stdin().read_line(&mut buf)
        .map(|_| buf)
}

pub fn read_file(p: &Path) -> IOResult {
    let mut file = File::open(p).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map(|_| content)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
