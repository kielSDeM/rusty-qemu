use cmd_lib::run_cmd;

struct Copy {
    copy: String,
}

trait QemuCopy {
    fn copy() -> String,
    fn copy_qemu() {},
}
//This function should copy make a copy of the vm image and bash script.
impl Copy for QemuCopy {
    copy() -> String {
        let mut copy_qemu = Copy {
            copy: String::new();
        };
        //Should rename the copy of the bash script.
        println!("Enter name of new bash script.")
        copy_qemu.copy.clear();
        std::io::stdin()
            .read_line(&mut copy_qemu.copy)
            .expect("please enter a correct value.");
        let copy = copy_qemu.copy.trim();
        //uses bash commands to make the magic happen.
        return copy_qemu.copy;
    }
    copy_qemu() {
        println!("Going to make a copy of an existing vm");
        run_cmd!("cp copy.bash img_copy/{}.bash", QemuCopy::copy()).unwrap();
    }
}