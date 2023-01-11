# Cross compiling `pgx`

*Warning: this guide is still a work in progress!*

## Caveats

Note that guide is fairly preliminary and does not cover many cases, most notably:

1. This does not (yet) cover cross compiling with `cargo pgx` (planned). Note that this means this documentation may only be useful to a small set of users.

2. This is assuming that you are cross compiling between `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` (either direction works). Compiling to other targets will likely be similar, but are left as an exercise for the reader.

3. Cross-compiling the `cshim` is possible but difficult and not fully documented here. You should ensure that the `pgx/cshim` is disabled when you perform the cross-build.

# Distributions

Unfortunately, the cross-compilation process is quite distribution specific. We'll cover two cases:

1. Debian-based distributions, where this is very easy.
2. Distributions where userspace cross-compilation is not directly supported (such as the Fedora-family). This is much more difficult, so if you have a choice you should not go this route.

## Debian

Of the mainstream distributions (that is, excluding things like NixOS which probably do also make this easy) the easiest path available is likely to be on Debian-family systems. This is for two reasons:

1. The cross compilation tools can be installed via an easy package like `crossbuild-essential-arm64` (when targetting `aarch64`) or `crossbuild-essential-amd64` (when targetting `x86_64`)

2. The cross compilation sysroot is the same as the normal sysroot -- they're both `/`.

3. Many tools in the Rust ecosystem (the `bindgen` and `cc` crates) know where many things are located, out of the box.

And a few other aspects which are less critical (if you get the tools on Debian 11, then you know they'll run on any machine has a Debian 11 install -- no need to worry about glibc versions, for example).

### The Steps

On the steps on Debian-family are as follows:

1. Set up everything you'd need to perform non-cross builds.

2. Install a Rust toolchain for the target:
    - *`target=aarch64`*: `rustup toolchain add aarch64-unknown-linux-gnu`.
    - *`target=x86_64`*: `rustup toolchain add x86_64-unknown-linux-gnu`.

3. Install the `crossbuild-essential-<arch>` package for the architecture you are targetting
    - *`target=aarch64`*: `sudo apt install crossbuild-essential-arm64`.
    - *`target=x86_64`*: `sudo apt install crossbuild-essential-amd64`.

4. Set some relevant environment vars:
    - *`target=aarch64`*: `export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc`.
    - *`target=x86_64 `*: `export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc`.

    Note: It's also possible to set these in your `.cargo/config.toml`, but note that they're distribution-specific (and on other distros, potentially machine-specific), so I would recommend against checking them into version control.
    ```toml
    # Replace `<arch>` with the target arch.
    [target.<arch>-linux-gnu-gcc]
    linker = "<arch>-linux-gnu-gcc"
    ```

5. Build your extension.
    - *`target=aarch64`*: `cargo build --target=aarch64-unknown-linux-gnu --release`.
    - *`target=x86_64 `*: `cargo build --target=x86_64-unknown-linux-gnu --release`.

This will produce a `.so` in `./target/<target>/release/lib$yourext.so`, which you can use.

> *TODO: this seems like it is not quite complete -- we may need things like this (when targetting `aarch64` from `x86_64`)? Needs some slightly further investigation for _why_, though, since most of this should be auto-detected (notably the target and isystem paths...)*
>
> ```sh
> export BINDGEN_EXTRA_CLANG_ARGS_aarch64-unknown-linux-gnu="-target aarch64-unknown-linux-gnu -isystem /usr/aarch64-linux-gnu/include/ -ccc-gcc-name aarch64-linux-gnu-gcc"
> ```

# Other Distributions

*Note: these steps are still somewhat experimental, and may be missing some pieces.*

The first few steps are the same as under debian.

1. Set up everything you'd need to perform non-cross builds.

2. Install a Rust toolchain for the target:
    - *`target=aarch64`*: `rustup toolchain add aarch64-unknown-linux-gnu`.
    - *`target=x86_64`*: `rustup toolchain add x86_64-unknown-linux-gnu`.

After this you need a cross compilation toolchain, which can be challenging.

## Get a cross-compilation toolchain

To cross compile, you need a toolchain. This is basically two parts:

- The cross-compile tools: suite of tools, scripts, libraries (and the like) which are compiled for the host architecture (so that you can run them) which are capable of building for the target. Specifically, this includes things like the compiler, linker, assembler, archiver, ... as well as any libraries they link to, and tools they invoke.

- A sysroot, which is basically an emulated unix filesystem root, with `<sysroot>/usr`, `<sysroot>/etc` and so-on. The important thing here is that it has libraries built for the target, headers configured for the target, and so on.

Pick well here, since getting a bad one may cause builds that succeed but fail at runtime.

An easy option for targetting `aarch64` (or several other architectures) from `x86_64` is to use one of the ones on <https://toolchains.bootlin.com/releases_aarch64.html> (not an endorsement: they're something I've used for development, I don't know how well-made they are, and they honestly seem kind of idiosyncratic. IOW, I'd want to do a lot more research before putting them into production).

Sadly, I don't have a good option for an easily downloaded x86_64 toolchain that has tools built for aarch64. I've been using a manually built one, which isn't covered in this guide (TODO?).

So, assuming you're getting one from the link above, you need to find a toolchain:
- Which uses glibc (not musl/uclibc), since we're compiling to a `-linux-gnu` Rust target.
- Contains a version of glibc which is:
    - Has a version of glibc which is no newer than the ones which will be used on any of the target machines.
    - But still new enough to contain any symbols you link to non-weakly (don't worry about this part unless it becomes a problem).
- Has linux headers which are similarly no newer than the version on any of the target machines. This *probably* doesn't matter for you, and it might not be a thing you have to worry about with toolchains from elsewhere (most don't contain kernel headers, since most applications don't need them to build).

If you can't find one with an old enough version of glibc, try to get as close as possible, and we'll just have to hope for the best -- there's a chance it will work still, it depends on what system functions you call in the compiled binary.

Anyway, once you have one of these you may need to put it somewhere specific -- not all of them are relocatable (the ones from `toolchains.bootlin.com` above are, however). An easy way to do this is by running `bin/aarch64-linux-gcc --print-sysroot`, and seeing if it prints the a path inside it's directory. If not, you may have to move things around so that the answer it gives is correct.

## Use the cross compilation toolchain

Continuing from above, I will assume (without loss of generality) that you're targetting aarch64, have a toolchain directory at `$toolchain_dir` and your sysroot is at `$sysroot_dir` -- try `$toolchain_dir/bin/aarch64-linux-gnu-gcc --print-sysroot`.

Anyway, set

- `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=$toolchain_dir/bin/aarch64-linux-gnu-gcc`
- `BINDGEN_EXTRA_CLANG_ARGS_aarch64-unknown-linux-gnu=--sysroot=\"$sysroot_dir\"`

You may also need:
- `CC_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-gcc`
- `AR_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-ar`
- `CXX_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-g++`
- `LD_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-ld`

And sometimes you may need to add `$toolchain_dir/bin` to path and set `CROSS_COMPILE=aarch64-linux-gnu-` can help. Sadly, this can break things depending on what's in your toolchain's path, or if you have a tool which doesn't check if it's actually a cross-compile before looking at the `CROSS_COMPILE` var.

TODO(thom): that's sometimes enough to complete a build but this is very WIP.

TODO(thom): flags to make the cshim build.
