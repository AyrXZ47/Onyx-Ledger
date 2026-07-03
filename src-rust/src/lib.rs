use wasm_bindgen::prelude::*;

// Esta macro es la magia que expone la función pura de Rust a TypeScript
#[wasm_bindgen]
pub fn inicializar_motor() -> String {
    String::from("Onyx Ledger: Núcleo en Rust operativo. Contabilidad de doble entrada lista.")
}
