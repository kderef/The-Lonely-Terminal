#!/usr/bin/env python
from os import system, path
import os, platform, pathlib

host = platform.system();
sep = path.sep

bundle = f".{sep}bundle"
exe = f".{sep}target{sep}release{sep}the_lonely_terminal{'.exe' if host == 'Windows' else ''}"
fonts = f".{sep}font"
bundle_exe = path.join(bundle, exe)
bundle_fonts = path.join(bundle, "fonts\\")

def rmdir(path_: str):
    path = pathlib.Path(path_)
    for sub in path.iterdir():
        if sub.is_dir():
            rmdir(sub)
        else:
            sub.unlink()
    path.rmdir()

def cp(a: str, b: str):
    global host
    if host == "Windows":
        system(f"copy {a} {b}")
    elif host in ("Darwin", "Linux"):
        system(f"cp {a} {b}")
    else:
        raise NotImplemented(f"Not implemented for OS '{host}'")

print("$ cargo build --release")
system("cargo build --release")

print(f"> creating {bundle}")
if path.isdir(bundle):
    rmdir(bundle)
os.mkdir(bundle)

print("> creating fonts dir")
os.mkdir(bundle_fonts)

print(f"> copying {exe} to {bundle_exe}")
cp(exe, bundle)

print(f"> copying fonts to {bundle}")
cp(fonts, bundle_fonts)

print(f"> copying DLLs to {bundle}")
cp(f".{sep}dll", bundle)

print("-----------------------")
print("! A bundle has been created in the current directory.")