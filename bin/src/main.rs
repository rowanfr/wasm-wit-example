use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

// Generate bindings for the host
wasmtime::component::bindgen!({
    path: "../wit/example.wit",
    world: "test"
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let component = Component::from_file(&engine, "../wasm/example.wasm")?;
    let test_component = Test::instantiate(&mut store, &component, &linker)?;
    let result = test_component.call_respond(&mut store, "Rob")?;
    println!("{}", result);
    Ok(())
}
