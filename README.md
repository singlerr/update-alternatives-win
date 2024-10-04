# update-alternatives for Windows

----

This project is an unofficial port of update-alternatives-java for Windows. 
On Linux, a user can switch his/her current JDK version easily, but there's no features same with it on Windows.
So it is why I coded this tool.

## Getting started

----

1. Clone a repository:

```shell
git clone https://github.com/singlerr/update-alternatives-win.git
cd update-alternatives-win
```

2. Build it using cargo:

```shell
cargo build --release
```

There will be some binaries in target/release directory. Just copy update-alternatives.exe file into somewhere you can locate.

3. Add binary to your System Path

Go to System Path settings, add file path you located above

----

### Note

Giving modifications to system environment variables requires administrator permissions. Don't forge to run it as Administrator!