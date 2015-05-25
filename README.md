# rraf

The Rimraf utility, written in Node (https://github.com/isaacs/rimraf), is a handy way to delete entire directory trees on Windows.  "Normal" deletion tools provided by CMD, Powershell, Python or indeed window's built in file explorer GUI are unreliable as they choke on long path names (node_modules, I'm looking at you!), read only attributes or transient file locks.

Rimraf has the problem that it depends on Nodejs, which can be unacceptable in many environments. Furthermore, 
rimraf (and obviously nodejs itself) sprinkle unwanted files around the file system.

rraf, a stanalone Rust "port" of rimraf, is a single statically linked .exe file, so you can drop it anywhere.

Basic usage is just 

```
rraf path_to_delete
```

In case the files are locked, rraf can use systinternals [handle.exe](https://technet.microsoft.com/en-us/sysinternals/bb896655.aspx) to force the applications release the file handles (without killing the applications!). To use this feature, install "handle" from Chocolatey ("choco install handle") and provide "-c" argument (for "close"), like so:

```
rraf -c path_to_delete
```

rraf only works in Windows, as unix platforms don't generally have need for it due to less restricted PATH length and file system locking strategies.

To build it, you need to grab a nightly build of Rust and do "cargo build".

