use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(file) = args.get(1) {
        let data = fs::read_to_string(file)?;
        let code = hex::decode(data.trim())?;

        for s in evm_bytecode::heuristic::selectors_from_bytecode(&code) {
            println!("0x{}", hex::encode(s));
        }

        for s in evm_bytecode::heuristic::events_from_bytecode(&code) {
            println!("0x{}", hex::encode(s));
        }

        println!(
            "Is creation code: {}",
            if evm_bytecode::heuristic::is_creation_code(&code) {
                "Yes"
            } else {
                "No"
            }
        );
    } else {
        eprintln!("no file");
    }

    Ok(())
}
