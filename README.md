# Amiga ROM Tool

This is a simple tool (not as the title suggests) written in Rust to split or merge Amiga even/odd ROM files. It supports byteswapping.

Just run ``cargo build`` to build it. Use the --help parameter to get further info on usage.

Examples:
``./amirom merge kick31_even.bin kick31_odd.bin kick31.bin``
``./amirom merge --byteswap kick31_even.bin kick31_odd.bin kick31.bin``
``./amirom split kick31.bin kick31_even.bin kick31_odd.bin``
