/// Common utilities used across the application

/// Check if an extension is for a text-based file (whitelist approach)
pub fn is_text_extension(extension: &str) -> bool {
    match extension.to_lowercase().as_str() {
        // Text-based programming languages and markup
        "rs" | "py" | "js" | "ts" | "java" | "cpp" | "c" | "h" | "cs" | "go" | "rb" | "php" |
        "swift" | "kt" | "groovy" | "scala" | "jsx" | "tsx" | "sh" | "bash" | "batch" | "ps1" |
        "html" | "htm" | "xml" | "xhtml" | "xsd" | "xsl" | "css" | "scss" | "sass" | "less" |
        "json" | "yaml" | "yml" | "md" | "markdown" | "txt" | "conf" | "config" | "ini" | "toml" |
        "env" | "bat" | "cmd" | "pl" | "pm" | "pyx" | "rst" | "adoc" | "asm" | "sql" | "prq" |
        "ps1" | "rsbuild" | "cson" | "coffee" | " UE4game" | "uex" | "graphql" | "gql" | "pug" |
        "jade" | "handlebars" | "hbs" | "mustache" | "hjson" | "elm" | "clj" | "cljs" | "cljc" |
        "mjml" | "wxml" | "swig" | "twig" | "jinja" | "jade" | "hql" | "graphql" | "gql" | "proto" |
        "hbs" | "handlebars" | "mustache" | "hjson" | "elm" | "clj" | "cljs" | "cljc" | "mjml" |
        "wxml" | "swig" | "twig" | "jinja" | "proto" => true,
        // Shell scripts
        "PS1" | "sh" | "bash" | "zsh" | "fish" | "M" | "bat" | "cmd" | "btn" | "vbs" => true,
        // Configuration files
        "cfg" | "config" | "ini" | "toml" | "env" | "conf" | "rc" | "vim" | "bashrc" | "zshrc" |
        "fishconfig" | "gitconfig" | "gitignore" | "fconf" | "fcatch" | "fcart" | "fcat" => true,
        // Documentation and markup
        "rst" | "adoc" | "asciidoc" | "asciidoc" | "md" | "markdown" | "mdown" | "mkdown" |
        "mkdn" | "mkd" | "mdwn" | "mmd" | "mdbase" | "mdtext" | "mdoc" | "mdown" | "mdwn" |
        "rmd" | "markdown" | "markdownl" | "markdown" | "mdtxt" | "txt" | "text" => true,
        // Web files
        "html" | "htm" | "htm" | "xhtml" | "shtml" | "shtm" | "xht" | "xhtml" | "xsht" | "jhtm" =>
        true,
        // Other text formats
        "csv" | "tsv" | "log" | "gitignore" | "yml" | "yaml" | "xml" | "xsd" | "xsl" | "css" |
        "scss" | "sass" | "less" | "sql" | "pl" | "pm" | "rst" | "adoc" | "asciidoc" | "adoc" |
        "md" | "markdown" | "txt" | "text" | "xsd" | "rxd" | "xml" | "xsl" | "dxml" | "exsd" |
        "pl" | "pl" | "pyx" | "rpx" | "rqx" | "rxp" | "rxp" | "xpx" | "rpx" | "rpq" => true,
        // Other languages
        "D" | "d" | "ais" | "glam" | "inum" => true,

        _ => false,
    }
}
