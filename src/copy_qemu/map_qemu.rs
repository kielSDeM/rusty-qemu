use std::collections::HashMap;
mod repl_virt;
use repl_virt::Repl_VM;
use repl_virt::Replicate;

struct List {
    vm: HashMap<i32, String>,
    key: i32,
    value: String,
}

trait AddMap {
    fn add_map() {},
}
//Need to create for loop that will iterate and increase as the size of the hashmap incrases.
impl List for AddMap {
    fn add_map() {
        let mut add_map = List {
            key: i32,
            value: String::new(),
        };
        println!("inserting a choice for VM copy.");
        
        println!("Please create a name for new VM.");
        value.clear();
        std::io::stdin().read_line(&mut value);
        let value = value.trim();
        Replicate::replicate(key, value);
        for(key, value) in vm.iter().enumerate() {
            println!("{}: {}", key, value);
        }
}

}





