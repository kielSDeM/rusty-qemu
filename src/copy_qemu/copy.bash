#!/bin/bash
echo "please enter kali lubu or lxle"
#This script will be used to make copies of VMs.
    qemu-system-x86_64 -drive file=qemu_img/copy.qcow2,format=raw -boot d \
    -m 2048M -cpu host -device intel-hda -device hda-duplex \
    -smp 2 -spice port=3000,disable-ticketing=on \
    -netdev user,id=mynet0,net=192.168.76.0/24,dhcpstart=192.168.76.9 \
    -device e1000,netdev=mynet0,mac=10:5c:32:31:d2:dd  \
    -vga qxl -enable-kvm 
    echo "starting lubuntu"
fi 
