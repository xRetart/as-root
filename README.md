# as-root
## description
as-root is a project developing the `as-root` executable.
The `as-root` executable is a command for elevating permissions (like `sudo`).
It hopes to improve on `sudo` by being more minimalistic and on `doas`
(another alternative for `sudo` made by the OpenBSD devs) by being written in a higher level language than C.
This project is written entirely in `rust` making the binary fast and lightweight
while keeping the source readable and easily maintained.


## installation
### prerequesites
* [GNU Make](https://www.gnu.org/software/make/)
* [Rust suite with Cargo](https://www.rust-lang.org/tools/install)


### command
With root permissions run:  
```shell
make install
```


## usage
Put the user authorized for use of `as-root` into `/etc/conf.d/as-root` separated by line feeds.  
Then you will be able to run `<command>` with root permissions like so:  
```shell
as-root <command>
```
