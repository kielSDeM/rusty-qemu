mod copy_qemu;
use copy_qemu::Copy;
use copy_qemu::QemuCopy;
use cmd_lib::run_cmd;

struct Delete {

}

trait DeleteVM {
    fn delete_cp() {},
}

impl Delete for DeleteVM {
    fn delete_cp() {
        run_cmd!("rm img_copy/{}.qcow2", Qemucopy::copy()).unwrap();
        run_cmd!("rm {}.bash", Qemucopy::copy()).unwrap();
    }
}
