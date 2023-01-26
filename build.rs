use std::io;
use winres::WindowsResource;

fn main() -> io::Result<()> {
    WindowsResource::new()
        .set_icon("assets/icon.ico")
        .compile()?;
    Ok(())
}
