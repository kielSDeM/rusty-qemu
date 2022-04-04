use std::collections::HashMap;

struct Repl_VM {
    vm: HashMap<String, String>,
    key: String,
    value: String,
}

trait Replicate {
    replicate(a: String, b: String),
}
impl Repl_VM for Replicate {
    fn replicate(key: String, value: String) {
        let mut replicate = Repl_VM {
            vm: HashMap::new(),
            key: String::new(),
            value: String::new(),
        };
        vm.insert(String::from(key), String::from(value));
    }
}