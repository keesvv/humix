# humix

A human-first Unix approach.

## Goals

- “Do one thing, and do it well” ~ Unix philosophy
- Portability
- ELF as executable format for cross-platform compatibility
- Hints, failsafes and interactive friendliness in interactive programs
- A framework[^1] for several common computing tasks
- WebAssembly support[^2] (?)

[^1]:
    A set of libraries intended for standardizing a certain task, such as
    text manipulation, shell behaviour or filesystem browsing. I will elaborate
    further on this goal in the near future.

[^2]:
    This would allow for simulating the inner workings of the kernel in the
    browser. Could be a fun replacement for [webnix](https://github.com/keesvv/webnix).

## Try it

### Building

**Prerequisites**

- Rust toolchain (`nightly`)
  - `x86_64-unknown-none` target
  - `rust-src` component

`cargo build -p humix`

### Generating a bootable image for x86-64

**Prerequisites**

- All from [Building](#building)
- Rust toolchain (`nightly`)
  - `llvm-tools-preview` component
- [`bootimage`](https://github.com/rust-osdev/bootimage)

`cargo bootimage -p humix`

## Roadmap

### Kernel

- [x] VGA text mode driver
- [x] Kernel buffer (kprint)
- [x] CPU exception handling
- [x] Kernel panics
- [x] IRQs
- [x] Serial driver
- [ ] Syscalls
- [ ] Memory management
- [ ] Virtual filesystem
- [ ] Device filesystem (devfs)
- [ ] Filesystem (enlightenfs, efs)
- [ ] Threading
- [ ] Process management
  - [ ] File descriptors
  - [ ] Standard streams
- [ ] TTY
- [ ] Graphics
- [ ] Sound
- [ ] Networking
- [ ] Jails[^3]

[^3]:
    Being able to put restraints on programs and isolate them into
    separate environments.

### Userspace

- [ ] Standard library
- [ ] Init system
- [ ] Shell (renaissance shell, rsh)
- [ ] System administration utilities (su, useradd, etc)
- [ ] Filesystem utilities (fsck, mkfs, etc)
- [ ] Core utilities (ls, rm, etc)
- [ ] Installer TUI
- [ ] Display server
- [ ] Window manager
- [ ] Audio codecs

This list will be updated from time to time.

## Known issues

### To be solved

### Solved

- The kernel is still very hardwired to the x86 platform. It should
  be further abstracted to allow for portability. (fixed in ce079bc)

- The VGA text mode buffer does not scroll yet. After the maximum
  of 25 lines in the buffer has been reached, further printed text
  won't be shown on the screen and will overflow beyond the video
  memory at some point. (fixed in b8790de)

## Author

Kees van Voorthuizen ([@keesvv](https://github.com/keesvv))

## License

[GPLv3](./LICENSE)
