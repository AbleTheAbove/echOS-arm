# EchOS

## Tooling
I can't exactly remember all the tools I installed, feel free to pr with any additions
`cargo install cargo-xbuild`


## Roadmap
- [ ] File System implimentation
  - [ ] A KernelFileSystem seperate from the OSFS (Operating System File System) 
    - [ ] KFS standard size and location (I.E. 512 Kilos at cold store offset x)
  - [ ] Modularity
  - [ ] URI Based user land syscall system
  - [ ] Install time file system selection
- [ ] Cryptographically secure random syscall
  - [ ] Able to randomly provide all/most types 
