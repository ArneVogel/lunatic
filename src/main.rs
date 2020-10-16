use anyhow::Result;
use tokio::runtime::Builder;
use wasmtime::{Config, Engine, Module};

use lunatic_vm::patching::patch;
use lunatic_vm::process::creator::{spawn, FunctionLookup, MemoryChoice};

use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let wasm_path = args.get(1).expect("Not enough arguments passed");
    let wasm = fs::read(wasm_path).expect("Can't open WASM file");

    // Transfrom WASM file into a format
    let (min_memory, wasm) = patch(&wasm)?;

    let mut config = Config::new();
    config.wasm_threads(true);
    config.wasm_simd(true);
    config.static_memory_guard_size(128 * 1024 * 1024); // 128 Mb
    let engine = Engine::new(&config);

    let module = Module::new(&engine, wasm)?;

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        spawn(
            engine,
            module,
            FunctionLookup::Name("_start"),
            MemoryChoice::New(min_memory),
        )
        .await
        .unwrap()
    })?;

    Ok(())
}
