mod copy_qemu;
use copy_qemu::Copy;
use copy_qemu::QemuCopy;
use cmd_lib::run_cmd;

struct Image {
    image: String,
}

trait QemuImage {
    fn image_qemu() {},
}
//This function should image make a image of the vm image and bash script.
impl Image for QemuImage {
    image_qemu() {
        let mut image_qemu = image {
            image: String::new();
        };
        //SHould give a new name to the vm image
        println!("Enter name of new vm image.")
        image_qemu.image.clear();
        std::io::stdin()
            .read_line(&mut image_qemu.image)
            .expect("please enter a correct value.");
        let image = image_qemu.image.trim();
        //uses bash commands to make the magic happen.
        run_cmd!("cp ~/rusty-qemu/src/qemu_img/lubu.qcow2 \
        image_qemu/img_image/{}.qcow2", image).unwrap();
        run_cmd!("sed -i 's/image/{}/g' {}.bash", image, Qemucopy::copy()).unwrap();
    }
}