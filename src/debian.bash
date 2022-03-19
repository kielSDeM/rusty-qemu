#!/bin/bash
FILE1=/home/zero/rusty-qemu/src/debian.img
FILE2=debian-11.2.0-amd64-netinst.iso
# creates a virtual disk for lxle if there is not one
if [ ! -f "$FILE1" ]
then
    echo "Going to create a lxle virt-disk"
    qemu-img create -f raw debian.img 20G 
else
    echo "files exists"
fi
# downloads lxle.iso if it does not exist in current directory.
if [ ! -f "$FILE2" ]
then
echo "going to download lxle"
wget https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-11.2.0-amd64-netinst.iso
else
    echo "lxle downloaded"
fi
# starts qemu with the iso if it is not installed or starts the vm.
if qemu-img info debian.img | grep -i "disk size: 4 KiB"
then
    qemu-system-x86_64 -hda debian.img -boot d \
    -cdrom debian-11.2.0-amd64-netinst.iso  -m 2048M -cpu host \
    -smp 1 -spice port=3001,disable-ticketing=on \
    -vga qxl -enable-kvm 
    echo "vm creation successfully completed"
else
    qemu-system-x86_64 -hda debian.img -boot d \
    -m 4096M -cpu host \
    -smp 2 -spice port=3001,disable-ticketing=on \
    -vga qxl -enable-kvm 
    echo "starting debian"
fi 
