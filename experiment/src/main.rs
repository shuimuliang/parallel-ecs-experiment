use std::env;

fn main() -> anyhow::Result<()> {
    let mut lib_path = env::current_dir()?;
    lib_path.push("target/debug/libecs_experiment.dylib");
    unsafe {
        let lib = libloading::Library::new(lib_path)?;
        println!("lib: {:?}", lib);
    }
    Ok(())
}
