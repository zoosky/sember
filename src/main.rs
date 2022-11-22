use std::{fs, collections::HashMap};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SiteConfig {
    accent: Option<String>,
    show_cv: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AboutConfig {
    name: String,
    email: Option<String>,
    photo: Option<String>,
    location: Option<String>,
    description: Option<String>,
    description_long: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LinkConfig {
    label: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JobConfig {
    company: String,
    url: Option<String>,
    position: String,
    location: String,
    description: String,
    from: String,
    to: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EducationConfig {
    name: String,
    url: Option<String>,
    degree: String,
    field_of_study: String,
    from: String,
    to: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    site: Option<SiteConfig>,
    about: AboutConfig,
    links: Option<Vec<LinkConfig>>,
    jobs: Option<Vec<JobConfig>>,
    education: Option<Vec<EducationConfig>>,
    skills: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum TemplateData {
    Str(String),
    Bool(bool),
}

#[derive(RustEmbed)]
#[folder = "views/"]
struct View;

fn s(str: &str) -> String {
    return str.to_string();
}

fn compose_page_data(name: &str, config: &Config) -> HashMap<String, TemplateData> {
    let mut page_data = HashMap::new();

    // set name
    page_data.insert(s("name"), TemplateData::Str(name.to_string()));

    // set page title
    let title = match name {
        "index" => config.about.name.clone(),
        "cv" => format!("{} — Résumé", config.about.name.clone()),
        _ => String::new(),
    };

    page_data.insert(s("title"), TemplateData::Str(title));

    // set accent
    let accent = config.site.clone().unwrap_or_default().accent.unwrap_or("#000".to_string());
    page_data.insert(s("accent"), TemplateData::Str(accent));

    // show cv?
    let show_cv = config.site.clone().unwrap_or_default().show_cv.unwrap_or(false);
    page_data.insert(s("show_cv"), TemplateData::Bool(show_cv));

    return page_data;
}

fn compile_template_partials(name: &str, config: &Config) -> HashMap<String, String> {
    let partials_names = Vec::from(["head", "foot", "nav"]);
    let mut partials: HashMap<String, String> = HashMap::new();

    for partial in partials_names {
        let template_contents = View::get(&format!("./partials/{}.twig", partial)).unwrap();
        let template_str = std::str::from_utf8(template_contents.data.as_ref()).unwrap();
        
        // create context
        let mut context = Context::new();
        context.insert("config", config);
        context.insert("page", &compose_page_data(name, &config));

        // create template
        let template = Tera::one_off(template_str, &context, false);
        
        if template.is_ok() {
            partials.insert(s(partial), template.unwrap());
        } else {
            println!("{:?}", template.err().unwrap());
            partials.insert(s(partial), format!("Failed to render partial: {}", partial));
        }
    }

    return partials;
}

fn compile_template(name: &str, config: &Config) -> String {
    let template_contents = View::get(&format!("./{}.twig", name)).unwrap();
    let template_str = std::str::from_utf8(template_contents.data.as_ref()).unwrap();
    
    // create context
    let mut context = Context::new();
    context.insert("config", &config);
    context.insert("page", &compose_page_data(name, &config));
    context.insert("partials", &compile_template_partials(name, &config));
    
    // create template
    let template = Tera::one_off(template_str, &context, false);

    return template.unwrap();
}

/// Reads the `sember.toml` file into a `Config` struct.
fn read_config() -> Config {
    let config_str = fs::read_to_string("./sember.toml").unwrap_or(String::default());
    let config: Config = toml::from_str(&config_str).unwrap();

    return config;
}

fn delete_public_dir() {
    fs::remove_dir_all("./public").unwrap_or(());
}

fn build_index(config: &Config) {
    let html = compile_template("index", config);

    fs::create_dir_all("./public").unwrap();
    fs::write("./public/index.html", html).unwrap();
}

fn build_cv(config: &Config) {
    let html = compile_template("cv", config);

    fs::create_dir_all("./public").unwrap();
    fs::write("./public/cv/index.html", html).unwrap();
}

fn main() {
    let config = read_config();

    // delete public dir
    delete_public_dir();

    // build index
    build_index(&config);

    // build cv
    build_cv(&config);
}