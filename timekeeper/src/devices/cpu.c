#include "cpu.h"
#include "rawhw/syscon.h"
#include "debug.h"

static void wait_for_pll_lock() {
	while (!SYSCON.SYSPLLSTAT.LOCK)
		;
}

static void init_pll() {
	// See Section 4.10.4.1 on how to calculate these values.
	// The chosen values result in a output clock of 36 MHz.
	SYSCON.SYSPLLCTRL.MSEL = 2;
	SYSCON.SYSPLLCTRL.PSEL = 2;
	SYSCON.SYSPLLCLKSEL.SEL = SYSPLLCLKSEL_SEL_IRC;
	SYSCON.SYSPLLCLKUEN.ENA = 0;
	SYSCON.SYSPLLCLKUEN.ENA = 1;
	SYSCON.PDRUNCFG.SYSPLL_PD = 0;
	wait_for_pll_lock();
}

static void init_cpu() {
	// Sets the CPU clock to 36 MHz.
	SYSCON.PRESETCTRL.FLASH_OVERRIDE = PRESETCTRL_FLASH_OVERRIDE_MULTI_CYCLE_READS;
	SYSCON.SYSAHBCLKDIV.DIV = 1;
	SYSCON.MAINCLKSEL.SEL = MAINCLKSEL_SEL_PLL_OUTPUT;
	SYSCON.MAINCLKUEN.ENA = 0;
	SYSCON.MAINCLKUEN.ENA = 1;
}

void cpu_init() {
	TRACE();
	init_pll();
	init_cpu();
}
