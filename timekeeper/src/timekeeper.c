#include "debug.h"
#include "hw.h"
#include "bitops.h"
#include <stdlib.h>

static void init_system_clock() {
	DEBUG_PRINT("init_clock\n");

	// See Section 4.10.4.1 on how to calculate these values.
	// The chosen values result in a output clock of 36 MHz.
	SET_BITS(SYSPLLCTRL, 0, 4, 0b10);  // MSEL = 2 (feedback divide by 3)
	SET_BITS(SYSPLLCTRL, 5, 6, 0b10);  // PSEL = 2 (post divide by 4)
	SET_BITS(SYSPLLCLKSEL, 0, 1, 0);  // use IRC (12 MHz) as clock source
	CLEAR_BIT(SYSPLLCLKUEN, 0);  // apply new PLL clock source
	SET_BIT(SYSPLLCLKUEN, 0);
	CLEAR_BIT(PDRUNCFG, 7);  // enable PLL
	while (!BIT_VALUE(SYSPLLSTAT, 0))  // wait for PLL to be ready
		;

	// Sets the CPU clock to 36 MHz.
	CLEAR_BIT(PRESETCTRL, 15);  // FLASH_OVERRIDE = 0 (multi-cycle reads)
	SET_BITS(SYSAHBCLKDIV, 0, 7, 1);  // DIV = 1 (divide system clock by 1)
	SET_BITS(MAINCLKSEL, 0, 1, 0x3);  // use PLL output as clock source
	CLEAR_BIT(MAINCLKUEN, 0);  // apply new clock source
	SET_BIT(MAINCLKUEN, 0);
}

int main() {
	DEBUG_PRINT("Timekeeper\n");
	init_system_clock();

	DEBUG_PRINT("I'm done!\n");
    return EXIT_SUCCESS;
}
