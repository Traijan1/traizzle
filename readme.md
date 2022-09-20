# Traizzle

Traizzle is my kernel project in which I want to learn more about Kernel and OS Programming.  
I develop only it when I have enough time for it.

<br>

## Status
- A Serial Port Driver, current implementation only uses COM1. It's used for Debug purpose.
- A very simple frambuffer abstraction that allows me to print chars with a PSF-Font, Strings and Rectangles.
- Rust print!- and println!-Macros are working and use the Console abstraction currently.

<br>

## Current plans
- Rethink about how I structure the Kernel (do I need a Console abstraction in my kernel? (probably not)).
- Implement Interrupts.
- Implement Paging. 

<br>

## How to run

You can run the kernel in the crate directory with following command: **cargo krun**. 
This will open qemu-system-x86_64 so be sure to have it installed.

If you want to try it on real hardware use: **cargo kimage**. The efi file will be in target/x86_64-unknown-efi/release/uefi.efi.
After that you can flash it with rufus or balenaEtcher on an USB drive and boot from it.

<br>

## After informations

I use the bootloader from [here](https://github.com/rust-osdev/bootloader).  
I learned the setup for Rust Kernel development from the [Rust OS Dev Blog](https://os.phil-opp.com/).