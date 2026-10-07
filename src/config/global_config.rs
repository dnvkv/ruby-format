pub struct GlobalConfig {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub default_formatter: Formatter,
}

pub enum Formatter {
    Progress,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            include: vec![
                "**/*.rb",
                "**/*.arb",
                "**/*.axlsx",
                "**/*.builder",
                "**/*.gemfile",
                "**/*.gemspec",
                "**/*.jb",
                "**/*.jbuilder",
                "**/*.mspec",
                "**/*.opal",
                "**/*.pluginspec",
                "**/*.podspec",
                "**/*.rabl",
                "**/*.rake",
                "**/*.rbw",
                "**/*.ru",
                "**/*.ruby",
                "**/*.schema",
                "**/*.spec",
                "**/*.thor",
                "**/.irbrc",
                "**/.pryrc",
                "**/.simplecov",
                "**/buildfile",
                "**/Appraisals",
                "**/Berksfile",
                "**/Brewfile",
                "**/Buildfile",
                "**/Capfile",
                "**/Dangerfile",
                "**/Deliverfile",
                "**/Fastfile",
                "**/*Fastfile",
                "**/Gemfile",
                "**/Guardfile",
                "**/Jarfile",
                "**/Mavenfile",
                "**/Podfile",
                "**/Puppetfile",
                "**/Rakefile",
                "**/rakefile",
                "**/Schemafile",
                "**/Snapfile",
                "**/Steepfile",
                "**/Thorfile",
                "**/Vagrantfile",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
            exclude: vec!["node_modules/**/*", "tmp/**/*", "vendor/**/*", ".git/**/*"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            default_formatter: Formatter::Progress,
        }
    }
}
