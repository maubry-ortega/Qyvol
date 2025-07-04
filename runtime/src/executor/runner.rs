// # VolleyDevByMaubry [14/∞] "Correr un componente es liberar el alma del código en un instante eterno."
use crate::executor::engine::MyState;
use anyhow::Result;
use wasmtime::Store;
use wasmtime::component::{Component, Linker};
use wasmtime_wasi::p2::bindings::sync::Command;

pub fn run_component(
    store: &mut Store<MyState>, linker: &Linker<MyState>, component_bytes: &[u8],
) -> Result<()> {
    let component = Component::new(store.engine(), component_bytes)?;

    let command = Command::instantiate(&mut *store, &component, linker)?;

    let result = command.wasi_cli_run().call_run(store)?;

    if let Err(()) = result {
        anyhow::bail!("El componente terminó con un código de salida de error.");
    }

    Ok(())
}
