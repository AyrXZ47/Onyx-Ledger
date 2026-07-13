use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
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

impl Transaction {
    pub fn is_valid(&self) -> bool {
        self.entries.iter().map(|e| e.amount).sum::<i64>() == 0
    }
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

thread_local! {
    static WORKSPACE_STORE: RefCell<Option<HashMap<String, Workspace>>> = RefCell::new(None);
}

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

#[wasm_bindgen]
pub fn init_system() {
    WORKSPACE_STORE.with(|store| {
        *store.borrow_mut() = Some(HashMap::new());
    });
}

#[wasm_bindgen]
pub fn create_workspace(name: &str, ws_type: &str) -> String {
    let workspace_type = match ws_type {
        "Personal" => WorkspaceType::Personal,
        "Freelance" => WorkspaceType::Freelance,
        "Business" => WorkspaceType::Business,
        _ => panic!("invalid workspace type: {ws_type}"),
    };

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed).to_string();
    let workspace = Workspace::new(id.clone(), name.to_string(), workspace_type);

    WORKSPACE_STORE.with(|store| {
        let mut guard = store.borrow_mut();
        if let Some(ref mut map) = *guard {
            map.insert(id.clone(), workspace);
        } else {
            panic!("system not initialized: call init_system() first");
        }
    });

    id
}

#[wasm_bindgen]
pub fn validate_and_add_transaction(ws_id: &str, transaction_json: &str) -> Result<(), JsValue> {
    let transaction: Transaction = serde_json::from_str(transaction_json).map_err(|e| {
        JsValue::from_str(&format!("failed to parse transaction JSON: {e}"))
    })?;

    if !transaction.is_valid() {
        return Err(JsValue::from_str(
            "transaction entries do not sum to zero: invalid double-entry",
        ));
    }

    WORKSPACE_STORE.with(|store| {
        let mut guard = store.borrow_mut();
        let map = guard
            .as_mut()
            .ok_or_else(|| JsValue::from_str("system not initialized: call init_system() first"))?;

        let workspace = map.get_mut(ws_id).ok_or_else(|| {
            JsValue::from_str(&format!("workspace not found: {ws_id}"))
        })?;

        workspace.ledger.transactions.push(transaction);
        Ok(())
    })
}
