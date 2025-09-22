use std::env;
mod shell_utils;
use std::collections::HashMap;

mod module_collapse;

mod module_split;

mod module_unique;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut supported_modules: HashMap<String, fn()> = HashMap::new();
    supported_modules.insert("collapse".to_string(), module_collapse::main);
    supported_modules.insert("split".to_string(), module_split::main);
    supported_modules.insert("unique".to_string(), module_unique::main);

    let sys_call = args.first().unwrap();
    let callable_name = if sys_call.contains("/") {
        sys_call.split("/").last().unwrap()
    } else {
        sys_call
    };

    match supported_modules.get(callable_name) {
        Some(callable) => callable(),
        None => shell_utils::main(supported_modules),
    };
}
