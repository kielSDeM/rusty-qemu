use cmd_lib::run_cmd;
use cmd_lib::run_fun;
pub struct StartQemu {
    qemu: String,
}

pub trait QemuVirt {
    fn qemu_virt();
}

impl QemuVirt for StartQemu {
    fn qemu_virt() {
        let mut qemu_virt = StartQemu {
            qemu: String::new(),
        };
        loop {
            println!(
                "Choose which system to start: \n
                      1: Kali \n
                      2: Arch \n
                      3: LXLE \n
                      4: Exit"
            );
            qemu_virt.qemu.clear();
            std::io::stdin()
                .read_line(&mut qemu_virt.qemu)
                .expect("please enter a correct value.");
            let qemu = qemu_virt.qemu.trim();
            match qemu {
                "1" => {
                    run_cmd!(./kali-qemu.bash).unwrap();
                    eprintln!("Success!");
                    break;
                }
                "2" => {
                    run_cmd!(./arch-qemu.bash).unwrap();
                    eprintln!("Success!");
                    break;
                }
                "3" => {
                    run_fun!(./lxle-qemu.bash).unwrap();
                    eprintln!("Success!");
                    break;
                }
                "4" => {
                    println!("Program will now exit");
                    break;
                }
                _ => {
                    println!("Please enter a correct response.")
                }
            }
        }
    }
}
