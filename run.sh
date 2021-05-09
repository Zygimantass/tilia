#!/bin/sh
cargo build && cargo bootimage && \
qemu-system-x86_64 -drive format=raw,file=target/x86_64-tilia/debug/bootimage-tilia.bin -debugcon stdio -machine smm=off -no-reboot \
	                 -drive file=fat.bs,if=none,id=disk -device ich9-ahci,id=ahci -device ide-drive,drive=disk,bus=ahci.0
