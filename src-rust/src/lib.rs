use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub enum WorkspaceType {
    Personal,
    Freelance,
    Business,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
    pub account_id: String,
    pub amount: i64,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub description: String,
    pub entries: Vec<Entry>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Ledger {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub workspace_type: WorkspaceType,
    pub ledger: Ledger,
}

#[wasm_bindgen]
impl Workspace {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, name: String, workspace_type: WorkspaceType) -> Workspace {
        Workspace {
            id,
            name,
            workspace_type,
            ledger: Ledger {
                accounts: Vec::new(),
                transactions: Vec::new(),
            },
        }
    }
}
