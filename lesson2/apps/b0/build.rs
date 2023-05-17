use std::io::Result;

extern crate cc;

fn main() {
    gen_linker_script().unwrap();

    cc::Build::new()
        .compiler("riscv64-linux-gnu-gcc")
        .file("src/initcalls.c")
        .flag("-mabi=lp64d")
        .compile("initcalls");
}

fn gen_linker_script() -> Result<()> {
    let content = "CFG_BASE_ADDRESS = 0xffffffc080200000;\n";
    std::fs::write("linker_riscv64_generated.lds", content)?;
    Ok(())
}
