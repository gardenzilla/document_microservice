use std::io::Write;
use std::process::Command;
use std::{ffi::OsString, fs};

use handlebars::Handlebars;
use serde::Serialize;

#[derive(Debug)]
pub enum DocumentKind {
    CashIn,
    CashOut,
    Procurement,
    InventoryLog,
    // ..
}

impl ToString for DocumentKind {
    fn to_string(&self) -> String {
        match self {
            DocumentKind::CashIn => format!("cash_in"),
            DocumentKind::CashOut => format!("cash_out"),
            DocumentKind::Procurement => format!("procurement"),
            DocumentKind::InventoryLog => format!("inventory_log"),
        }
    }
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
    fn get_template(&self) -> &'static str {
        match self {
            DocumentKind::CashIn => std::include_str!("../templates/cash_in.tex"),
            DocumentKind::CashOut => std::include_str!("../templates/cash_out.tex"),
            DocumentKind::Procurement => std::include_str!("../templates/procurement.tex"),
            DocumentKind::InventoryLog => std::include_str!("../templates/inventory_log.tex"),
        }
    }
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Get => format!("get"),
            Method::Post => format!("post"),
        }
    }
}

#[derive(Debug)]
pub struct QueryResponse {
    kind: DocumentKind,
    id: String,
    document_pdf_base64: String,
}

#[derive(Debug)]
pub enum DocumentError {
    NotFound { kind: DocumentKind, id: u32 },
    IdTaken { id: u32 },
    RenderError(String),
    FileError(String),
}

impl ToString for DocumentError {
    fn to_string(&self) -> String {
        match self {
            DocumentError::NotFound { kind, id } => format!(
                "NotFound! kind: {}, id: {}",
                kind.to_string(),
                id.to_string()
            ),
            DocumentError::IdTaken { id } => format!("ID taken id: {}", id),
            DocumentError::RenderError(e) => format!("Render Error: {}", e),
            DocumentError::FileError(e) => format!("File error {}", e),
        }
    }
}

#[derive(Debug)]
pub enum RenderError {
    TempDirError,
    TexFileCreationError,
    RenderError(String),
    PdfLatexError,
    PdfReadError(String),
}

impl ToString for RenderError {
    fn to_string(&self) -> String {
        match self {
            RenderError::TempDirError => format!("TempDirError"),
            RenderError::TexFileCreationError => format!("Tex file creation error"),
            RenderError::RenderError(e) => format!("Render error: {}", e),
            RenderError::PdfLatexError => format!("Pdf Latex error"),
            RenderError::PdfReadError(e) => format!("Pdf read error: {}", e),
        }
    }
}

fn pdf_render(latex: &str) -> Result<Vec<u8>, RenderError> {
    let tmp = tempdir::TempDir::new("pdf_render").map_err(|_| RenderError::TempDirError)?;
    let input_file = tmp.path().join("input.tex");
    let output_file = tmp.path().join("input.pdf");

    let _ = fs::write(&input_file, latex).map_err(|_| RenderError::TexFileCreationError)?;

    let mut cmd = Command::new("pdflatex");

    cmd.args(&["input.tex"]);

    cmd.current_dir(tmp.path());

    let output = cmd
        .output()
        .map_err(|e| RenderError::RenderError(e.to_string()))?;

    if !output.status.success() {
        return Err(RenderError::PdfLatexError);
    }

    fs::read(output_file).map_err(|e| RenderError::PdfReadError(e.to_string()))
}

pub fn create_document<T>(
    id: u32,
    kind: DocumentKind,
    data: T,
) -> Result<QueryResponse, DocumentError>
where
    T: Serialize,
{
    let pdf_path =
        std::path::PathBuf::from(format!("data/documents/{}/{}.pdf", kind.get_location(), id));

    if pdf_path.exists() {
        return Err(DocumentError::IdTaken { id: id });
    }

    let reg = Handlebars::new();
    // render without register
    let res = reg
        .render_template(kind.get_template(), &data)
        .map_err(|e| DocumentError::RenderError(e.to_string()))?;

    let mut pdf_vec = pdf_render(&res).map_err(|e| DocumentError::RenderError(e.to_string()))?;
    println!("Vec is {:?}", &pdf_vec);

    // Create dir if not exist
    if let Some(root_path) = pdf_path.parent() {
        fs::create_dir_all(root_path).map_err(|e| DocumentError::FileError(e.to_string()))?;
    }

    let mut file = std::fs::File::create(pdf_path).unwrap();

    file.write_all(&mut pdf_vec)
        .map_err(|e| DocumentError::FileError(e.to_string()))?;

    file.flush()
        .map_err(|e| DocumentError::FileError(e.to_string()))?;

    Ok(QueryResponse {
        kind: kind,
        id: format!("{:x}", id),
        document_pdf_base64: "".into(),
    })
}

pub fn load_document(
    method: Method,
    kind: DocumentKind,
    id: u32,
) -> Result<QueryResponse, DocumentError> {
    todo!()
}
