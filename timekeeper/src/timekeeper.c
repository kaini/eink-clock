#include "debug.h"
#include "rawhw/hw.h"
#include "rawhw/syscon.h"
#include "bitops.h"
#include "devices/dcf77.h"
#include <stdlib.h>

static void system_clock_init() {
	// See Section 4.10.4.1 on how to calculate these values.
	// The chosen values result in a output clock of 36 MHz.
	SYSCON.SYSPLLCTRL.MSEL = 2;
	SYSCON.SYSPLLCTRL.PSEL = 2;
	SYSCON.SYSPLLCLKSEL.SEL = SYSPLLCLKSEL_SEL_IRC;
	SYSCON.SYSPLLCLKUEN.ENA = 0;
	SYSCON.SYSPLLCLKUEN.ENA = 1;
	SYSCON.PDRUNCFG.SYSPLL_PD = 0;
	while (!SYSCON.SYSPLLSTAT.LOCK)
		;

	// Sets the CPU clock to 36 MHz.
	SYSCON.PRESETCTRL.FLASH_OVERRIDE = PRESETCTRL_FLASH_OVERRIDE_MULTI_CYCLE_READS;
	SYSCON.SYSAHBCLKDIV.DIV = 1;
	SYSCON.MAINCLKSEL.SEL = MAINCLKSEL_SEL_PLL_OUTPUT;
	SYSCON.MAINCLKUEN.ENA = 0;
	SYSCON.MAINCLKUEN.ENA = 1;
}

int main() {
	DEBUG_PRINT("Timekeeper\n");
	system_clock_init();
	dcf77_init();

	bool payload[DCF77_PAYLOAD_BITS];
	dcf77_receive(payload);
	for (int i = 0; i < DCF77_PAYLOAD_BITS; ++i) {
		DEBUG_PRINT("%d", payload[i]);
	}
	DEBUG_PRINT("\n");

	DEBUG_PRINT("I'm done!\n");
    return EXIT_SUCCESS;
}
