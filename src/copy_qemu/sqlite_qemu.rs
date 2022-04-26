pub mod copy_qemu;
use copy_qemu::Copy;
use copy_qemu::QemuCopy;

use sqlite::State;

struct Database {
    vm_copy: String,
}
trait Store {
    fn sqlite(),
}
impl Store for Database {
    fn sqlite() {
        let mut sqlite = Database {
            vm_copy: Copy::copy(),
        }; 
        let connection = sqlite::open(":memory:").unwrap();
        connection.execute(
            "
            INSERT INTO vm_copies VALUES ('{}');
            ", vm_copy,
        )
        .unwrap();
    }
}