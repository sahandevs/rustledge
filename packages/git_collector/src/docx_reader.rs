use docx::DocxFile;
use std::path::Path;
use docx::document::{BodyContent, ParagraphContent, RunContent};

pub fn read_all_docx_text(path: &Path) -> Option<String> {
    // let docx_file = DocxFile::from_file(path);
    // if let Err(_) = docx_file { return None; }
    // let docx_file = docx_file.unwrap();
    // let docx = docx_file.parse();
    // if let Err(_) = docx { return None; }
    // let docx = docx.unwrap();
    let f = DocxFile::from_file(path).unwrap();
    let docx = f.parse().unwrap();

    let mut result = String::new();

    for content in &docx.document.body.content {
        match content {
            BodyContent::Paragraph(paragraph) => {
                for content in &paragraph.content {
                    match content {
                        ParagraphContent::Run(run) => {
                            for content in &run.content {
                                match content {
                                    RunContent::Text(text) => {
                                        result += &text.text;
                                        result += "\n";
                                    }
                                    RunContent::Break(_) => {
                                        result += "\n";
                                    }
                                }
                            }
                        }
                        ParagraphContent::Link(link) => {
                            for content in &link.content.content {
                                match content {
                                    RunContent::Text(text) => {
                                        result += &text.text;
                                    }
                                    RunContent::Break(_) => {
                                        result += "\n";
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn get_test_docx() -> &'static Path {
        Path::new("./test_files/docx_test.docx")
    }

    #[test]
    fn test_it_parsers_simple_docx_correctly() {
        let result = read_all_docx_text(get_test_docx()).unwrap();
        assert_eq!(result, "New page\nNew page\nTest test test\nمتن\n \nفارسی\nPerisna\nLong text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text \n\n\nNew page\n\n");
    }
}