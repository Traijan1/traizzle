# General informations

This is a tiny hobby project to learn Kernel Development in **Rust**.

# How to run

You can run the kernel in the crate directory with following command: **cargo krun**. 
This will open qemu-system-x86_64 so be sure to have it installed.

If you want to try it on real hardware use: **cargo kimage**. The efi file will be in target/x86_64-unknown-efi/release/uefi.efi.
After that you can flash it with rufus or balenaEtcher on an USB drive and boot from it.

# After informations

I use the bootloader from here: https://github.com/rust-osdev/bootloader