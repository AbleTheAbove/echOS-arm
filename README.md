# EchOS

## Tooling
I can't exactly remember all the tools I installed, feel free to pr with any additions
`cargo install cargo-xbuild`


## Roadmap
- [ ] File System implementation
  - [ ] Install time file system selection
- [ ] Modularity
- [ ] URI Based user land syscall system
- [ ] Cryptographically secure random syscall
  - [ ] Able to randomly provide all/most types

## Research && Resources

### File Systems
[File System Design](http://web.cs.ucla.edu/classes/fall10/cs111/scribe/11a/)

[Hackaday Article on file systems](https://hackaday.com/2019/01/24/cool-tools-a-little-filesystem-that-keeps-your-bits-on-lock/)

## An aside on RISC-V as a potential GPU
ok so a vector based processor? Well guess what? Images are vector Based
I can represent a picture with a `[[u8; 400]; 600]` and we have a 600x400 vector (well actually thats an array but shut up and let me talk). Now we have a CPU that can handle that natively? and if we optimize it for the GPU space and pray to ferris we can write a good driver that even half works we could beat *some* other GPUs and from there we tweak and turn and improve

An arm SOC with a RISC-V  (RISCPi?) would be dope and I'd start work on a port now, Hell if I knew anything about jack shit hardware stuff I'd do it myself and learn to write the firmware to tie the two together. Preferable more logically than the RPi, with you know a serial bus? maybe a daughter board so I can pop my RISC-V GPU out like a bad stick of ram when I buy an upgrade

Make a rust crate for the board and its daughter boards so that development is dead simple and dogfeed design a reference kernel or something to get some linux kernel guy to say "Dang thats cool" and port linux, and now we at least have a way in

Does QEMU have a simple way to add a machine? I could mock one up *Pausing to search* kind of

a power led for each board and an `OK` led for each as well. Should it support hot swapping the graphics? Probably not! Would it be cool? Yes

TODO: Split this off into a separate repo and develop the idea further
