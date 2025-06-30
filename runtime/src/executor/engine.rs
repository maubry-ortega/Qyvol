// # VolleyDevByMaubry [10/∞] "El estado del motor enciende la maquinaria de la revolución portátil."

use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi::ResourceTable;
use anyhow::Result;

pub struct MyState {
    pub wasi_ctx: WasiCtx,
    pub table: ResourceTable,
}

impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

pub fn build_state() -> Result<MyState> {
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .build();

    Ok(MyState {
        wasi_ctx,
        table: ResourceTable::new(),
    })
}
