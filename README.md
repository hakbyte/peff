# peff

A small command line tool to list the DLLs imported by a Windows binary. Get a
copy from the [Release](https://github.com/hakbyte/peff/releases) page.

## Usage


```
$ peff.exe --help
Usage: peff [<target...>] [--version] [-q]

Show DLLs imported by a Windows binary (EXE, DLL, OCX, SYS, etc.)

Positional Arguments:
  target            input files to analyze

Options:
  --version         print version and exit
  -q, --quiet       suppress errors
  --help            display usage information
```

> :memo: **Note**: Only *statically* imported symbols are shown. A binary can
> also dynamically load DLLs during runtime via *LoadLibrary()* and other APIs.

Ever wondered which DLLs a binary is importing? Here is an example:

```
$ peff.exe C:\Windows\System32\calc.exe
C:\Windows\System32\calc.exe: [
    ADVAPI32.dll
    KERNEL32.dll
    SHELL32.dll
    api-ms-win-core-libraryloader-l1-2-0.dll
    api-ms-win-core-processthreads-l1-1-0.dll
    api-ms-win-core-synch-l1-2-0.dll
    msvcrt.dll
]
```

If you pass a directory, `peff`  will recursively search all files inside the
directory and analyze them. In this case it's useful to pass the `-q` flag to
avoid seeing errors arising from trying to parse non-binary files.

## Why?

Sometimes during a Red Team exercise I come across unknown binaries and need to
quickly asses what they're doing. Knowing which DLLs the unknown binary imports
is an important first step in this search.

## Compiling

If you wish, you can compile the source code yourself:

```
$ git clone https://github.com/hakbyte/peff.git
$ cd peff
$ cargo build
```

Or use [cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
to build and install in a single step:

```
$ cargo install --git https://github.com/hakbyte/peff.git
```
