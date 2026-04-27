#!/bin/bash

if [ "$#" -ne 0 ]; then
  echo "this is $1"
  cargo rustc -- -C link-arg=--script=./linker.ld
  cargo objcopy -- -O binary ./kernel8.img

  sudo mount $1 /mnt/
  sudo cp kernel8.img /mnt
  sudo umount /mnt
fi
