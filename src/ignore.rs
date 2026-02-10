pub struct Ignored {
    ignored_files: Vec<&'static str>,
    ignored_extensions: Vec<&'static str>,
}

impl Ignored {
    pub fn new() -> Self {
        Self {
            ignored_files: vec![
                "/.git",
                // Rust
                "/target",
                // Javascript/TypeScript
                "/modules",
                "/node_modules",
                "/dist",
                "/build",
                // Frameworks
                "/.angular",
                // IDEs
                "/.idea",
                "/.vscode",
                "/.vs",
            ],
            ignored_extensions: vec![
                "ico", "png", "jpeg", "jpg", "gif", "bmp", "tiff", "webp", //Images
                "mp4", "mov", "avi", "mkv", "webm", // Vídeos
                "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", // Documentos binários
                "zip", "rar", "7z", "tar", "gz", // Arquivos compactados
            ],
        }
    }

    pub fn get_ignored_files(&self) -> &Vec<&'static str> {
        &self.ignored_files
    }

    pub fn get_ignored_extensions(&self) -> &Vec<&'static str> {
        &self.ignored_extensions
    }
}
