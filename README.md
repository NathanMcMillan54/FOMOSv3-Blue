# FOMOSv3-Blue v3 - Novusk v3-beta

A [Novusk](https://github.com/new-kernel/novusk) based OS

Build, use ``x86_64`` or ``aarch64``:
```commandline
make all ARCH=<arch> FEATURES=<device> # Read Documentation/devices.md
```

Run x86_64:
```commandline
qemu-system-x86_64 FOMOSv3.img
```

Run Aarch64:
```commandline
qemu-system-aarch64 -machine <device> -serial null -serial stdio -kernel kernel8.img
```

### Project history

[FOMOSv1-Yellow](https://github.com/NathanMcMillan54/FOMOSv1-Yellow) (Free, Open source, Mobile, Operating, System) 
<u>was</u> a UI for mobile devices, Linux would start a Js UI that someone would use. Eventually I wanted it to turn 
into a full OS, and it will one day with Novusk.

This is an OS that can run on whatever you want it to. Laptops, PCs, ands microcontrollers.
