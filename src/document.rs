use core::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{fs, path::PathBuf};

use chrono::Datelike;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

static INPUT_TEX_NAME: &'static str = "_input_autogenerated.tex";
static OUTPUT_PDF_NAME: &'static str = "_input_autogenerated.pdf";

#[derive(Debug)]
pub struct Prefix(&'static str);

impl Prefix {
    pub fn new(prefix: &'static str) -> Self {
        if prefix.len() > 5 {
            panic!(format!("Too long prefix: {}. Max 5 character", prefix));
        }
        if prefix != prefix.to_uppercase() {
            panic!(format!(
                "Prefix should be UPPERCASE! {} instead of {}",
                prefix.to_uppercase(),
                prefix
            ));
        }
        Self(prefix)
    }
}

impl From<&Prefix> for &'static str {
    fn from(p: &Prefix) -> Self {
        p.0
    }
}

impl ToString for Prefix {
    fn to_string(&self) -> String {
        self.0.into()
    }
}

pub struct Id {
    prefix: String,
    year: u32,
    id: u32,
}

pub enum IdError {
    FormatError,
}

impl Id {
    pub fn new(prefix: String, year: u32, id: u32) -> Self {
        Self { prefix, year, id }
    }
    pub fn parse_str(id_str: &str) -> Result<Id, IdError> {
        let parts: Vec<&str> = id_str.split('-').collect();
        if parts.len() != 3 {
            return Err(IdError::FormatError);
        }
        let prefix = parts[0].to_string();
        let year: u32 = parts[1].parse::<u32>().map_err(|_| IdError::FormatError)?;
        let id: u32 = parts[2].parse::<u32>().map_err(|_| IdError::FormatError)?;
        Ok(Id { prefix, year, id })
    }
    pub fn as_file_name(&self) -> String {
        format!("{}-{}-{}.pdf", self.prefix, self.year, self.id)
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        format!("{}-{}-{}", self.prefix, self.year, self.id)
    }
}

#[derive(Debug)]
pub struct Provider<T>
where
    for<'de> T: DocumentProvider + Serialize + Deserialize<'de> + Debug,
{
    prefix: Prefix,
    provider: T,
}

impl<T> Provider<T>
where
    for<'de> T: DocumentProvider + Serialize + Deserialize<'de> + Debug,
{
    pub fn init(prefix: &'static str, provider: T) -> Self {
        Self {
            prefix: Prefix::new(prefix),
            provider,
        }
    }
    pub fn get_document_bytes(&self, id: &str) -> Result<Vec<u8>, DocumentError> {
        // Deserialize ID
        let id: Id = Id::parse_str(id).map_err(|_| DocumentError::IdError)?;

        // Create document path
        // data/documents/LOCATION_NAME/PREFIX/YEAR/ID.pdf
        let document_path = std::path::PathBuf::from(format!(
            "data/documents/{}/{}/{}/{}",
            &self.provider.get_location(),
            &self.prefix.to_string(),
            id.year,
            id.as_file_name()
        ));

        // Check wheter the requested document exist
        if !&document_path.exists() {
            return Err(DocumentError::NotFound {
                kind: self.provider.get_location(),
                id: id.to_string(),
            });
        }

        // Read document as bytes array and return it
        fs::read(&document_path).map_err(|e| {
            DocumentError::InternalError(format!(
                "Could not read the requested document! {}",
                e.to_string()
            ))
        })
    }
    pub fn get_document_base64(&self, id: &str) -> Result<String, DocumentError> {
        let document_bytes = self.get_document_bytes(id)?;
        Ok(base64_encode(&document_bytes))
    }
    pub fn create_document(&self, data: &str) -> Result<Id, DocumentError> {
        // Parse data string to object T
        let object: T = serde_json::from_str(data)
            .map_err(|e| DocumentError::SerializationError(e.to_string()))?;

        // Define next ID
        let next_id: u32 = 1; // Todo!: implement this one!

        let new_document_id: Id = Id::new(
            self.prefix.to_string(),
            chrono::Utc::today().year() as u32,
            next_id,
        );

        // Create path to the final PDF
        let pdf_path = std::path::PathBuf::from(format!(
            "data/documents/{}/{}/{}/{}",
            self.provider.get_location(),
            self.prefix.to_string(),
            new_document_id.year,
            new_document_id.as_file_name()
        ));

        // Check if the file already exist
        // Impossible error situation if next_id implementation is correct
        if pdf_path.exists() {
            panic!(format!(
                "FATAL ERROR! Tried to create file {}, but path alread exists!",
                &pdf_path.to_string_lossy()
            ));
        }

        // Register a new handlebars instance
        let reg = Handlebars::new();

        // Create temp template variable
        let mut template: String = String::new();

        // Define template_path to get access tempalte file(s)
        let template_path = Path::new("templates").join(self.provider.get_location());

        // Read template file
        File::open(&template_path.join("template.tex"))
            .map_err(|e| {
                DocumentError::InternalError(format!(
                    "A template.tex file nem nyitható meg! {}",
                    e.to_string()
                ))
            })?
            .read_to_string(&mut template)
            .map_err(|e| {
                DocumentError::InternalError(format!(
                    "A template.tex file nem olvasható! {}",
                    e.to_string()
                ))
            })?;

        // render without register
        let rendered_template = reg
            .render_template(
                &template,
                &DataHolder::new(self.prefix.0, next_id.to_string(), object),
            )
            .map_err(|e| DocumentError::RenderError(e.to_string()))?;

        // Create temp dir for template files
        let tmp = tempdir::TempDir::new("pdf_render")
            .map_err(|_| DocumentError::InternalError(format!("Temp folder error")))?;

        // Copy all the template files into the temp dir
        fs::read_dir(&template_path)
            .map_err(|e| {
                DocumentError::InternalError(format!(
                    "template folder read error! {}",
                    e.to_string()
                ))
            })?
            .into_iter()
            .for_each(|f| {
                if let Ok(file) = f {
                    let _ = fs::copy(file.path(), &tmp.path().join(file.file_name()));
                }
            });

        // Set auto generated input tex file path to generate and fill with data
        let input_file_path = tmp.path().join(INPUT_TEX_NAME);

        // Generate parsed tex file
        let _ = File::create(&input_file_path).map_err(|e| {
            DocumentError::InternalError(format!(
                "input auto generated tex file cannot created! {}",
                e.to_string()
            ))
        })?;

        // Write rendered tex template to the input tex file
        let _ = fs::write(&input_file_path, rendered_template).map_err(|e| {
            DocumentError::InternalError(format!(
                "Hiba az auto generált latex file mentésekor! {}",
                e.to_string()
            ))
        })?;

        fs::read_dir(&tmp)
            .unwrap()
            .into_iter()
            .for_each(|f| println!("File: {:?}", f.unwrap().file_name()));

        // Set output PDF path
        let output_file_path = tmp.path().join(OUTPUT_PDF_NAME);

        let mut cmd = Command::new("pdflatex");

        cmd.args(&[INPUT_TEX_NAME]);

        cmd.current_dir(tmp.path());

        let cmd_output = cmd.output().map_err(|e| {
            DocumentError::InternalError(format!("Error while pdflatex render! {}", e.to_string()))
        })?;

        if !cmd_output.status.success() {
            return Err(DocumentError::InternalError(format!(
                "Error while pdflatex render!"
            )));
        }

        let mut pdf_bytes = fs::read(&output_file_path).map_err(|e| {
            DocumentError::InternalError(format!("Failed to read the generated PDF file! {}", e))
        })?;

        let parent_path = pdf_path.parent().unwrap(); // todo!: implement error handling here!

        fs::create_dir_all(parent_path).map_err(|e| {
            DocumentError::InternalError(format!(
                "Error while creating root path for document! {}",
                e.to_string()
            ))
        })?;

        let mut document_file = File::create(&pdf_path).map_err(|e| {
            DocumentError::InternalError(format!(
                "Failed to create generated PDF file! {}",
                e.to_string()
            ))
        })?;

        File::write_all(&mut document_file, &mut pdf_bytes).map_err(|e| {
            DocumentError::InternalError(format!(
                "Error while writing content to generated PDF! {}",
                e.to_string()
            ))
        })?;

        document_file
            .flush()
            .map_err(|e| DocumentError::InternalError(format!("Flush error: {}", e.to_string())))?;

        Ok(new_document_id)
    }
}

pub trait DocumentProvider {
    fn get_location(&self) -> &'static str; // Should implement
}

// Encode bytes array
fn base64_encode(input: &Vec<u8>) -> String {
    base64::encode(input)
}

#[derive(Debug)]
pub enum DocumentError {
    SerializationError(String),
    NotFound { kind: &'static str, id: String },
    IdTaken { id: u32 },
    RenderError(String),
    InternalError(String),
    IdError,
}

#[derive(Serialize)]
struct DataHolder<T>
where
    T: Serialize,
{
    id: String,
    prefix_id: String,
    data: T,
}

impl<T> DataHolder<T>
where
    T: Serialize,
{
    fn new(prefix: &'static str, id: String, data: T) -> Self {
        DataHolder {
            id: id.clone(),
            prefix_id: format!("{}{}", prefix, id),
            data,
        }
    }
}
