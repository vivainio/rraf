# rraf

The Rimraf utility, written in Node (https://github.com/isaacs/rimraf), is a handy way to delete entire directory trees on Windows.  "Normal" deletion tools provided by CMD, Powershell, Python or indeed window's built in file explorer GUI are unreliable as they choke on long path names (node_modules, I'm looking at you!), read only attributes or transient file locks.

Rimraf has the problem that it depends on Nodejs, which can be unacceptable in many environments. Furthermore, 
rimraf (and obviously nodejs itself) sprinkle unwanted files around the file system.

rraf, a stanalone rust "port" of rimraf, is a single statically linked .exe file built in Rust, so you can drop it anywhere.

It only works in Windows, as *nix based operating systems don't have problems with the poweruser scenarios of removing 
files from disk.

To build it, you need to grab a nightly build of Rust and do "cardo build".
