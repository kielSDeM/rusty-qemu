#!/bin/bash

sudo pacman -Syy
sudo pacman -S qemu
wget https://download.qemu.org/qemu-7.0.0-rc0.tar.xz
tar xvJf qemu-7.0.0-rc0.tar.xz
cd qemu-7.0.0-rc0
./configure --enable-spice --enable-libusb
make && sudo make install