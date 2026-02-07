pub struct Ignored;

impl Ignored {

    pub fn ignored_files() -> Vec<&'static str> {

        vec![
            // Generic
            "/.git",

            // Rust
            "/target",
            
            // Javascript/TypeScript
            "/modules",
            "/node_modules",

            // Frameworks
            "/.angular"

        ]

    }

    pub fn ignored_extensions() -> Vec<&'static str> {

        vec![
            //Images
            "ico",
            "png",
            "jpeg",

        ]

    }

}