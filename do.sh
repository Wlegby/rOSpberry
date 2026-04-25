#!/bin/bash
cargo rustc -- -C link-arg=--script=./linker.ld && cargo objcopy -- -O binary ./kernel8.img && sudo mount /dev/sdb1 /mnt/ && sudo cp kernel8.img /mnt && sudo umount /mnt
