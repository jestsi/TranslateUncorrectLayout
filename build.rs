use std::io;
use windres::Build;

fn main() -> io::Result<()> {
    Build::new().compile("tray-data.rc").unwrap();
    Ok(())
}