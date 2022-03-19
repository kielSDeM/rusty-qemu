use cmd_lib::run_cmd;

struct Qemu_Install {

}

trait Start_Install {
    qemu_libs(),
    install_qemu(),
}
//This should install qemu on hardware that does not have qemu installed on arch linux.
impl Qemu_Install for Start_Install {
    install_qemu() {
        run_cmd!(
                gnome-terminal -e 
                /home/zero/rusty-qemu/src/add_feats/libs-qemu.bash)
                .unwrap()
        run_cmd!(
                gnome-terminal -e 
                /home/zero/rusty-qemu/src/add_feats/qemu_install.bash)
                .unwrap();
        }
    }

