#!/bin/bash
FILE1=qemu_img/lxle.qcow2
FILE2=qemu_iso/lxle-18043-64.iso
# creates a virtual disk for lxle if there is not one
if [ ! -f "$FILE1" ]
then
    echo "Going to create a lxle virt-disk"
    qemu-img create -f raw lxle.qcow2 20G 
    mv lxle.qcow2 qemu_img
else
    echo "files exists"
fi
# downloads lxle.iso if it does not exist in current directory.
if [ ! -f "$FILE2" ]
then
echo "going to download lxle"
wget -P qemu_iso https://sourceforge.net/projects/lxle/files/Final/OS/18.04.3-64/lxle-18043-64.iso
else
    echo "lxle downloaded"
fi
# starts qemu with the iso if it is not installed or starts the vm.
if qemu-img info qemu_img/lxle.qcow2 | grep -i "disk size: 4 KiB"
then
    qemu-system-x86_64 -drive file=qemu_img/lxle.qcow2,format=raw -boot d \
    -cdrom qemu_iso/lxle-18043-64.iso  -m 2048M -cpu host \
    -smp 1 -spice port=3001,disable-ticketing=on \
    -vga qxl -enable-kvm 
    echo "vm creation successfully completed"
else
    qemu-system-x86_64 -drive file=qemu_img/lxle.qcow2,format=raw -boot d \
    -m 4096M -cpu host -device intel-hda -device hda-duplex \
    -smp 2 -spice port=3001,disable-ticketing=on \
    -vga qxl -enable-kvm 
    echo "starting lxle"
fi 
