mod copy_qemu;
use copy_qemu::Copy;
use copy_qemu::QemuCopy;
use cmd_lib::run_cmd;

struct Port {
    port: String,
}

trait QemuPort {
    fn port_qemu() {},
}
//This function should copy make a copy of the vm image and bash script.
impl Port for QemuPort {
    port_qemu() {
        let mut port_qemu = Copy {
            port: String::new();
        };
        //Should create new port number to vnc from.
        println!("Please enter a port number.")
        qemu_copy.port.clear();
        std::io::stdin()
            .read_line(&mut copy_qemu.port)
            .expect("please enter a correct value.");
        let port = port_qemu.port.trim();
        //uses bash commands to make the magic happen.
        run_cmd!("sed -i 's/3000/{}/g' {}.bash", port, Qemucopy::copy()).unwrap();
    }
}