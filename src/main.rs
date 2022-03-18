pub mod qemu;
use qemu::QemuVirt;
use qemu::StartQemu;

fn main() {
    StartQemu::qemu_virt();
}
