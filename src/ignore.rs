pub struct Ignored;

impl Ignored {
    pub fn ignored_files() -> Vec<&'static str> {
        vec![
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
        ]
    }

    pub fn ignored_extensions() -> Vec<&'static str> {
        vec![
            "ico", "png", "jpeg", "jpg", "gif", "bmp", "tiff", "webp", //Images
            "mp4", "mov", "avi", "mkv", "webm", // Vídeos
            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", // Documentos binários
            "zip", "rar", "7z", "tar", "gz", // Arquivos compactados
        ]
    }
}
