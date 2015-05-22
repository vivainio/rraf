# rraf

rimraf utility (https://github.com/isaacs/rimraf) is a handy way to delete entire directory trees on Windows. 
"Normal" deletion tools provided by CMD, Powershell, Python or indeed window's built in file explorer GUI are unreliable as
they choke on long path names (node_modules, I'm looking at you!), read only attributes or transient file locks.

Rimraf has the problem that it depends on Nodejs, which can be unacceptable in many environments. Furthermore, 
rimraf (or node) sprinkles unwanted files around the file system.

rraf is a standalone .exe written in Rust, so you can drop it anywhere.

It only works in Windows, as *nix based operating systems don't have problems with the poweruser scenarios of removing 
files from disk.
