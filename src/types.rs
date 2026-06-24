#[derive(Debug, Clone)]
pub struct SimpleFinding{
    pub file_path : String,
    pub line_number : usize,
    pub message : String
}

#[derive(Debug, Clone)]
pub enum ScannerEvent{
    ProcessingFile(String),
    FoundIssue(SimpleFinding),
    Finished,
}