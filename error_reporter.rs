use crate::lexer::Token;

pub struct ErrorReporter {
    errors: Vec<String>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        ErrorReporter { errors: Vec::new() }
    }

    pub fn report_error(&mut self, message: String, token: Option<&Token>) {
        let location = match token {
            Some(t) => format!("(Satır: {}, Sütun: {})", t.line, t.column),
            None => "(Bilinmeyen konum)".to_string(),
        };
        self.errors.push(format!("Hata {}: {}", location, message));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn print_errors(&self) {
        for error in &self.errors {
            eprintln!("{}", error);
        }
    }

    // Sahne64 API'sine özgü hata mesajları için yardımcı fonksiyonlar eklenebilir
    pub fn report_sahne64_api_error(&mut self, api_call: &str, message: String, token: Option<&Token>) {
        self.report_error(format!("Sahne64 API Hatası ({}: {}): {}", api_call, message, token.map(|t| format!("(Satır: {}, Sütun: {})", t.line, t.column)).unwrap_or_else(|| "(Bilinmeyen konum)".to_string())), token);
    }

    // İleride uyarılar için de benzer bir mekanizma eklenebilir
     warnings: Vec<String>,
     pub fn report_warning(...)
     pub fn print_warnings(...)
}