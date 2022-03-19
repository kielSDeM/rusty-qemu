use std::collections::HashMap;

struct Repl_VM {
    vm: HashMap<String, String>,
    a: String,
    b: String,
}

trait Replicate {
    replicate(a: String, b: String) -> HashMap<String, String>,
}
impl Repl_VM for Replicate {
    fn replicate(a: String, b: String) {
        let mut replicate = Repl_Virt {
            a: String::new(),
            b: String::new(),
            vm: HashMap::new(),
        };
        vm.insert(String::from(a), String::from(b));
        return vm;
    }
}