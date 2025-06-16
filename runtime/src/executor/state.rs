// # VolleyDevByMaubry [15/∞] "El estado guarda el latido de un sistema que respira libertad."
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiView};

pub struct State {
    pub table: Table,
    pub wasi: WasiCtx,
}

impl WasiView for State {
    fn table(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}
