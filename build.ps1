$OBJCOPY = "llvm-objcopy"

$BINARY = "./target/mips-ultra64-cpu/release/hack_flags_rs"

cargo build --release

& $OBJCOPY -O binary $BINARY rom.bin
& dd if=rom.bin of=rom.z64 bs=16K conv=sync status=none
& spimdisasm singleFileDisasm rom.bin disasm --start 0x1000 --vram 0x80000400 --disasm-unknown
