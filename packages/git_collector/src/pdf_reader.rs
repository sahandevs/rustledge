use std::path::Path;

pub fn read_all_pdf_text(_path: &Path) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn get_test_docx() -> &'static Path {
        Path::new("./test_files/pdf_test.pdf")
    }

    #[test]
    #[ignore]
    fn test_it_parsers_simple_docx_correctly() {
        let result = read_all_pdf_text(get_test_docx()).unwrap();
        assert_eq!(result, "New page\nNew page\nTest test test\nمتن\n \nفارسی\nPerisna\nLong text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text Long text \n\n\nNew page\n\n");
    }
}