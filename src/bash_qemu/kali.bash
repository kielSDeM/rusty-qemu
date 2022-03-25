#!/bin/bash
#written by Kiel DeMarco

FILE1=qemu_img/kali.qcow2
FILE2=qemu_iso/kali-linux-2022.1-installer-amd64.iso
#creates kali virtual disk if it does not exist
if [ ! -f "$FILE1" ]
then
    echo "Going to create a kali virt-disk & download kali iso"
    qemu-img create -f raw kali.qcow2 40G 
    mv kali.qcow2 qemu_img
else
    echo "files exist"
fi
#downloads kali iso if not already downloaded
if [ ! -f "$FILE2" ]
then
wget -P qemu_iso https://cdimage.kali.org/kali-2022.1/kali-linux-2022.1-installer-amd64.iso
else
    echo "kali downloaded"
fi

#starts kali img with iso or without iso if installed.
if qemu-img info qemu_img/kali.qcow2 | grep -i "disk size: 4 KiB"
then
    qemu-system-x86_64 -drive file=qemu_img/kali.qcow2,format=raw -boot d \
    -cdrom qemu_iso/kali-linux-2022.1-installer-amd64.iso  -m 4096M -cpu host \
    -smp 2 -enable-kvm 
    echo "vm creation successfully completed"
else
    echo "starting kali"
    qemu-system-x86_64 -drive file=src/qemu_img/kali.qcow2,format=raw  -boot d \
    -m 8192M -cpu host -device intel-hda -device hda-duplex \
    -smp 4 -spice port=3000,disable-ticketing=on \
    -vga qxl -enable-kvm -device intel-hda -device hda-duplex \
    -device virtio-serial-pci,id=virtio-serial0,max_ports=16,bus=pci.0,addr=0x5 \
    -chardev spicevmc,name=vdagent,id=vdagent \
    -device virtserialport,nr=1,bus=virtio-serial0.0,chardev=vdagent,name=com.redhat.spice.0 \
    -usb -device usb-host,hostbus=1,hostport=2 \
    -device virtio-net,netdev=vmnic -netdev user,id=vmnic
fi 