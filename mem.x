/*STM32F100RB (from stm32vldiscovery)*/
//TODO: replace with a Cortex-M4 chip or, preferably, STM32F411CE

MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 128K
    RAM   : ORIGIN = 0x20000000, LENGTH = 8K
}
 ENTRY(Reset);

 EXTERN(RESET_VECTOR);

 SECTIONS
 {
    .vector_table ORIGIN(FLASH) :
    {
        /* SP (end of RAM) */
        LONG(ORIGIN(RAM) + LENGTH(RAM));

        /* Reset */
        KEEP(*(.vector_table.reset_vector));
    } > FLASH
    
    .text :
    {
        *(.text .text.*);
    } > FLASH

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
    }
 }
