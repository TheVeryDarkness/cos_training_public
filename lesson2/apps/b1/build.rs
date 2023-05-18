use std::io::Result;

extern crate cc;

fn main() {
    gen_linker_script().unwrap();
}

fn gen_linker_script() -> Result<()> {
    let content = "CFG_BASE_ADDRESS = 0xffffffc080200000;\n";
    std::fs::write("linker_riscv64_generated.lds", content)?;
    Ok(())
}
