use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(file) = args.get(1) {
        let data = fs::read_to_string(file)?;
        let code = hex::decode(data.trim())?;

        for s in evm_bytecode::selectors_from_bytecode(&code) {
            println!("{:#010x}", s);
        }
    } else {
        eprintln!("no file");
    }

    Ok(())
}
