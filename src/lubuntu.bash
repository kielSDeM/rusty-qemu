#!/bin/bash
FILE1=/home/zero/rusty-qemu/src/lubu.qcow2
FILE2=lubuntu-20.04.4-desktop-amd64.iso
# creates a virtual disk for lxle if there is not one
if [ ! -f "$FILE1" ]
then
    echo "Going to create a lxle virt-disk"
    qemu-img create -f raw lubu.qcow2 20G 
else
    echo "files exists"
fi
# downloads lxle.iso if it does not exist in current directory.
if [ ! -f "$FILE2" ]
then
echo "going to download lxle"
wget https://cdimage.ubuntu.com/lubuntu/releases/20.04.4/release/lubuntu-20.04.4-desktop-amd64.iso
else
    echo "lxle downloaded"
fi
# starts qemu with the iso if it is not installed or starts the vm.
if qemu-img info lxle.img | grep -i "disk size: 4 KiB"
then
    qemu-system-x86_64 -hda lxle.qcow2 -boot d \
    -cdrom lubuntu-20.04.4-desktop-amd64.iso  -m 2048M -cpu host \
    -smp 1 -spice port=3002,disable-ticketing=on \
    -vga qxl -enable-kvm 
    echo "vm creation successfully completed"
else
    qemu-system-x86_64 -hda lubu.qcow2 -boot d \
    -m 4096M -cpu host \
    -smp 2 -spice port=3002,disable-ticketing=on \
    -vga qxl -enable-kvm 
    echo "starting lubuntu"
fi 
