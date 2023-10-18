use std::ffi::OsStr;

fn main() -> ::std::io::Result<()> {
    let mut parent = std::env::current_dir()?;
    while parent.file_name() != Some(OsStr::new("space_physics_engine")) {
        parent.pop();
    };
    let path = parent.join("integrations/c/headers/");
    //println!("{}", path.display());
    #[cfg(feature = "headers")]
    return c_space_physics_engine::generate(path);
    #[cfg(not(feature = "headers"))]
    {
        println!("feature header is not enabled");
        return Ok(())
    }
}