use cmd_lib::run_cmd;

pub struct StartQemu {
    qemu: String,
}

pub trait QemuVirt {
    fn qemu_virt();
}
//currently it allows you to choose what vm you want to run.
//want it to be able to make copies of the vms as well.
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
                4: Lubuntu \n
                5: Exit"
            );
            qemu_virt.qemu.clear();
            std::io::stdin()
                .read_line(&mut qemu_virt.qemu)
                .expect("please enter a correct value.");
            let qemu = qemu_virt.qemu.trim();
            match qemu {
                "1" => {
                    run_cmd!(gnome-terminal -e bash_qemu/kali.bash)
                    .unwrap();
                    eprintln!("Starting Kali Linux");
                    continue;
                }
                "2" => {
                    run_cmd!(
                        gnome-terminal -e bash_qemu/arch-qemu.bash
                    )
                    .unwrap();
                    eprintln!("Starting Arch Linux");
                    continue;
                }
                "3" => {
                    run_cmd!(
                        gnome-terminal -e bash_qemu/lxle-qemu.bash
                    )
                    .unwrap();
                    eprintln!("Starting LXLE.");
                    continue;
                }
                "4" => {
                    run_cmd!(gnome-terminal -e bash_qemu/lubuntu.bash).unwrap();
                }
                "5" => {
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
