## berttagr

The release folder contains a build of the **rustlib** executable for Mac/Unix OS.

The executable can be called from the terminal or from R using the **system2( ) function.

The **rustlib** is a proof-of-concept part-of-speech tagger. To run, it also requires the following compiled PyTorch dylibs:

* libc10.dylib
* libiomp5.dylib
* libtorch_cpu.dylib
* libtorch.dylib

These can be downloaded from [PyTorch](https://pytorch.org/resources/). The link to **libtorch** for MacOS is here:

[https://download.pytorch.org/libtorch/cpu/libtorch-macos-1.9.0.zip](https://download.pytorch.org/libtorch/cpu/libtorch-macos-1.9.0.zip ) 

Download and unzip the contents. The compiled dylibs are located in the **lib** folder.

Before calling **rustlib** symlinks need to be set for the required dylibs. This can be done from the Terminal or in R.

```r
R.utils::createLink(link="/usr/local/lib/libtorch.dylib", "/Users/user/Downloads/libtorch/lib/libtorch.dylib", method="unix-symlink")
R.utils::createLink(link="/usr/local/lib/libtorch_cpu.dylib", "/Users/user/Downloads/libtorch/lib/libtorch_cpu.dylib", method="unix-symlink")
R.utils::createLink(link="/usr/local/lib/libc10.dylib", "/Users/user/Downloads/libtorch/lib/libc10.dylib", method="unix-symlink")
R.utils::createLink(link="/usr/local/lib/libiomp5.dylib", "/Users/user/Downloads/libtorch/lib/libiomp5.dylib", method="unix-symlink")
```

