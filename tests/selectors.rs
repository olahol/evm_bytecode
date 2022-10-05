use evm_bytecode::selectors_from_bytecode;
use std::{env, fs, path};

fn read_data_dir() -> Vec<String> {
    let path = path::Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests/data");

    fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_owned())
        .collect()
}

fn read_selectors(file_name: &str) -> Vec<u32> {
    let data = fs::read_to_string(file_name).unwrap();
    data.lines()
        .map(|line| u32::from_str_radix(&line[2..], 16).unwrap())
        .collect()
}

fn read_bytecode(file_name: &str) -> Vec<u8> {
    let data = fs::read_to_string(file_name).unwrap();
    hex::decode(data.trim()).unwrap()
}

#[test]
fn parse_selectors_from_files() {
    for file in read_data_dir() {
        if !file.ends_with(".bin") {
            continue;
        }

        let code = read_bytecode(&file);

        let selectors1 = read_selectors(&file.replace(".bin", ".txt"));
        let selectors2 = selectors_from_bytecode(&code);

        assert_eq!(
            selectors1.len(),
            selectors2.len(),
            "should find same number of selectors in {}",
            file
        );

        for s in selectors1 {
            assert!(
                selectors2.contains(&s),
                "selector {:#010x} should be in output of {}",
                s,
                file
            );
        }
    }
}
