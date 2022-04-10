pub mod copy_qemu;
pub mod image_qemu;
pub mod port_qemu;
pub mod sqllite_qemu;

use copy_qemu::Copy;
use copy_qemu::QemuCopy;
use image_qemu::Image;
use image_qemu::QemuImage;
use port_qemu::Port;
use port_qemu::QemuPort;
use sqlite_qemu::Store;
use sqlite_qemu::Database;

fn copy_finish() {
    Copy::copy_qemu();
    Image::image_qemu();
    Port::port_qemu();
    Store::sqlite();
}