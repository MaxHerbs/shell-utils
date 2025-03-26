use glob::glob;
use minijinja::{context, Environment};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let enabled_modules_path = Path::new("./src/main");
    let module_list = create_module_list();

    let mut jinja_env = Environment::new();
    let my_jinja = fs::read_to_string(enabled_modules_path.with_extension("jinja")).unwrap();
    jinja_env.add_template("main", &my_jinja).unwrap();
    let tmpl = jinja_env.get_template("main").unwrap();

    let templated_rust = tmpl.render(context!(modules => module_list)).unwrap();

    let mut file = File::create(enabled_modules_path.with_extension("rs")).unwrap();
    file.write_all(templated_rust.as_bytes()).unwrap();

    println!("cargo:warning={:?}", module_list);
}

fn create_module_list() -> Vec<String> {
    let mut module_list: Vec<String> = Vec::new();

    let paths = glob("./src/module_*").unwrap();
    for path in paths.into_iter() {
        let path_buf = path.unwrap();
        let path_str = path_buf.to_str().unwrap();

        if !path_buf.is_dir() {
            panic!("There is a file in ./src matching 'module_*'. Remove it.");
        }

        let path_stem = path_str.split("/").nth(1).unwrap();
        let module_name = path_stem.split("_").nth(1).unwrap();
        module_list.push(module_name.to_owned());
    }

    module_list
}

// fn capitalize(name: &str) -> String {
//     let mut chars = name.chars();
//     match chars.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
//     }
// }
