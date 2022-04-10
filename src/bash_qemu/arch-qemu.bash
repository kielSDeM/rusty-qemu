
#!/bin/bash
#written by Kiel DeMarco

FILE1=~/rusty-qemu/src/bash_qemu/arch.img
FILE2=~/rusty-qemu/src/bash_qemu/archlinux-2022.03.01-x86_64.iso
#creates arch virtual disk if it does not exist
if [ ! -f "$FILE1" ]
then
    echo "Going to create an arch virt-disk"
    qemu-img create -f raw arch.img 30G 
    mv arch.img qemu_img
else
    echo "files exists"
fi
#downloads arch iso if not already downloaded
if [ ! -f "$FILE2" ]
then
	echo "downloading arch linux"
	wget -P qemu_iso http://mirror.arizona.edu/archlinux/iso/2022.03.01/archlinux-2022.03.01-x86_64.iso
else
    echo "arch downloaded"
fi

#starts arch img with iso or without iso if installed.
if qemu-img info arch.img | grep -i "disk size: 4 KiB"
then
#This condition is for when the os is not installed.
    qemu-system-x86_64 -drive file=qemu_img/arch.img,format=raw -boot d \
    -cdrom src/qemu_iso/archlinux-2022.03.01-x86_64.iso  -m 4096M -cpu host \
    -smp 2 -enable-kvm 
    echo "starting arch booter"
else
#Here you can edit this condition for once the vm is installed on the image.
    qemu-system-x86_64 -drive file=qemu_img/arch.img,format=raw -boot d \
    -m 4096M -cpu host -device intel-hda -device hda-duplex \
    -smp 4 -spice port=3001,disable-ticketing=on \
    -vga qxl -enable-kvm \
    -device virtio-serial-pci,id=virtio-serial0,max_ports=16,bus=pci.0,addr=0x5 \
    -chardev spicevmc,name=vdagent,id=vdagent \
    -device virtserialport,nr=1,bus=virtio-serial0.0,chardev=vdagent,name=com.redhat.spice.0

    echo "starting arch"
fi 
