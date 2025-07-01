# GSP RPC binding generation for Nova
This is a simple out-of-tree kernel module designed for the sole purpose of generatiing bindings for GSP RPCs based on the open-source GSP headers available from https://github.com/NVIDIA/open-gpu-kernel-modules/. The results will be place in a file called `<OPEN_GPU_KERNEL_MODULES_VERSION>_bindings.rs` 

# Building
First clone the open-gpu-kernel-modules repository and checkout the version you desire building binding for. Currently r570.144 is being used for the nova-core driver and is the only version this has been tested with.
To build run the following:
`NVIDIA_OPEN_GPU_KERNEL_PATH=<open gpu kernel modules directory> KDIR=<kernel output directory> make`
The result for use with nova-core will be in `<OPEN_GPU_KERNEL_MODULES_VERSION>_bindings.rs`. Eg: `r570_144_bindings.rs`

# Generating new bindings
The symbols for which bindings are generated is controlled by the nvidia-open-gpu-symbols.txt file. This is a simple list of symbols to generate bindings for.
Note that the correct header files from open-gpu-kernel-modules will likely need to be included by updating `<OPEN_GPU_KERNEL_MODULES_VERSION>_bindings.h`
