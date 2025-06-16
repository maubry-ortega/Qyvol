// # VolleyDevByMaubry [10/∞] "El estado del motor enciende la maquinaria de la revolución portátil."
use crate::executor::errors::ExecutorError;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

pub struct MyState {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub fn build_state() -> Result<MyState, ExecutorError> {
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();

    let state = MyState { ctx: wasi_ctx, table: ResourceTable::new() };

    Ok(state)
}
