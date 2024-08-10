OUTPUT_FORMAT ("elf32-ntradbigmips")
OUTPUT_ARCH (mips)

MEMORY {
    KERN : ORIGIN = 0xA0000000, LENGTH = 0x00400000
    CACHED : ORIGIN = 0x80000000, LENGTH = 0x00400000
    DUMMY : ORIGIN = 0x00000000, LENGTH = 0xFFFFFFFF
}

__entry = 0x80000400;

SECTIONS {
    __romPos = 0;

    header_ROM_START = __romPos;
    .hdr : AT(header_ROM_START) {
        . = 8;
        LONG(__entry)
    } > DUMMY
    __romPos = 0x1000;
    header_ROM_END = __romPos;

    boot_ROM_START = __romPos;
    .boot __entry : AT(boot_ROM_START) {
        __boot_start = .;
        KEEP(*(.entry*));
        KEEP(*(.boot*));
        . = ALIGN(8);
        __boot_end = .;
    } > CACHED
    __romPos += SIZEOF(.boot);
    __romPos = ALIGN(__romPos, 16);
    boot_ROM_END = __romPos;

    text_ROM_START = __romPos;
    .text : AT(text_ROM_START) {
        __text_start = .;
        *(.text*);
        . = ALIGN(8);
        __text_end = .;
    } > CACHED
    __romPos += SIZEOF(.text);
    __romPos = ALIGN(__romPos, 16);
    text_ROM_END = __romPos;

    data_ROM_START = __romPos;
    .data ALIGN(8) : AT(data_ROM_START) SUBALIGN(8) {
        *(.data*);
        . = ALIGN(8);
    } > CACHED
    __romPos += SIZEOF(.data);
    __romPos = ALIGN(__romPos, 16);
    data_ROM_END = __romPos;

    rodata_ROM_START = __romPos;
    .rodata ALIGN(16) : AT(rodata_ROM_START) SUBALIGN(16) {
        TEST_IMAGE_ROM_START = .;
        KEEP(*(.rodata.TEST_IMAGE));
        *(.rodata*);
        . = ALIGN(16);
    } > CACHED
    __romPos += SIZEOF(.rodata);
    __romPos = ALIGN(__romPos, 16);
    rodata_ROM_END = __romPos;

    . = ALIGN(8);
    .bss (NOLOAD) : {
        __bss_start = .;
        *(.bss)
        *(.bss*)
        *(.gnu.linkonce.b.*)
        *(COMMON)
        . = ALIGN(8);
         __bss_end = .;
         __bss_size = (__bss_end - __bss_start);
    } > CACHED

    _heap_start = __bss_end;
    _heap_len = (ORIGIN(CACHED) + LENGTH(CACHED)) - __bss_end;

    /DISCARD/ : {
        *(.MIPS.*)
        *(.comment)
        *(.mdebug.*)
        *(.pdr)
        *(.reginfo)
        *(.eh_frame)

        /*
         * We may need the global offset table some day.
         * Our target is currently set with a static relocation-model, so this
         * might not be needed after all.
         */
        *(.got)
    }
}