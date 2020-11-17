use serde::Serialize;

pub enum DocumentKind {
    CashIn,
    CashOut,
    Procurement,
    InventoryLog,
    // ..
}

impl DocumentKind {
    fn get_location(&self) -> &'static str {
        match self {
            DocumentKind::CashIn => "cash_in",
            DocumentKind::CashOut => "cash_out",
            DocumentKind::Procurement => "procurement",
            DocumentKind::InventoryLog => "inventory_log",
        }
    }
}

pub enum Method {
    Get,
    Post,
}

pub struct QueryResponse {
    kind: DocumentKind,
    id: String,
    document_pdf_base64: String,
}

pub enum DocumentError {
    NotFound { kind: DocumentKind, id: u32 },
}

pub fn create_document<T>(
    method: Method,
    kind: DocumentKind,
    data: T,
) -> Result<QueryResponse, DocumentError>
where
    T: Serialize,
{
    todo!()
}

pub fn load_document(
    method: Method,
    kind: DocumentKind,
    id: u32,
) -> Result<QueryResponse, DocumentError> {
    todo!()
}
