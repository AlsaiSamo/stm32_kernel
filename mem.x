/*STM32F100RB (from stm32vldiscovery)*/
/* TODO: replace with a Cortex-M4 chip or, preferably, STM32F411CE */

MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 128K
    RAM   : ORIGIN = 0x20000000, LENGTH = 8K
}
 ENTRY(Reset);

 EXTERN(RESET_VECTOR);
 EXTERN(EXCEPTIONS);

 SECTIONS
 {
    .vector_table ORIGIN(FLASH) :
    {
        /* SP (end of RAM) */
        LONG(ORIGIN(RAM) + LENGTH(RAM));
        KEEP(*(.vector_table.reset_vector));
        KEEP(*(.vector_table.exceptions));
    } > FLASH
    
    .text :
    {
        *(.text .text.*);
    } > FLASH

    .rodata :
    {
        *(.rodata .rodata.*);
    } > FLASH

    .bss :
    {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > RAM

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
    {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > RAM

    _sidata = LOADADDR(.data);

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
    }
 }

 PROVIDE(NMI = DefaultExceptionHandler);
 PROVIDE(HardFault = DefaultExceptionHandler);
 PROVIDE(MemManage = DefaultExceptionHandler);
 PROVIDE(BusFault = DefaultExceptionHandler);
 PROVIDE(UsageFault = DefaultExceptionHandler);
 PROVIDE(SVCall = DefaultExceptionHandler);
 PROVIDE(PendSV = DefaultExceptionHandler);
 PROVIDE(SysTick = DefaultExceptionHandler);
