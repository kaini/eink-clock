#pragma once
#include "workarounds.h"
#include <stdbool.h>

typedef enum PRESETCTRL_RST_N {
	PRESETCTRL_RST_N_ENABLED = 0,
	PRESETCTRL_RST_N_DEASSERTED = 1,
} PRESETCTRL_RST_N;

typedef enum PRESETCTRL_FLASH_OVERRIDE {
	PRESETCTRL_FLASH_OVERRIDE_MULTI_CYCLE_READS = 0,
	PRESETCTRL_FLASH_OVERRIDE_ONE_CYCLE_READS = 1,
} PRESETCTRL_FLASH_OVERRIDE;

typedef struct PRESETCTRL_REG {
	PRESETCTRL_RST_N SSP_RST_N : 1;
	PRESETCTRL_RST_N I2C_RST_N : 1;
	PRESETCTRL_RST_N UART0_RST_N : 1;
	PRESETCTRL_RST_N UART1_RST_N : 1;
	PRESETCTRL_RST_N CT16B0_RST_N : 1;
	PRESETCTRL_RST_N CT16B1_RST_N : 1;
	PRESETCTRL_RST_N CT32B0_RST_N : 1;
	PRESETCTRL_RST_N CT32B1_RST_N : 1;
	PRESETCTRL_RST_N CMP_RST_N : 1;
	PRESETCTRL_RST_N CRC_RST_N : 1;
	PRESETCTRL_RST_N DMA_RST_N : 1;
	const unsigned _RES1 : 4;
	PRESETCTRL_FLASH_OVERRIDE FLASH_OVERRIDE : 1;
	const unsigned _RES2 : 16;
} PRESETCTRL_REG;
_Static_assert(sizeof(PRESETCTRL_REG) == 4, "");

typedef struct SYSPLLCTRL_REG {
	unsigned MSEL : 5;
	unsigned PSEL : 2;
	const unsigned _RES0 : 25;
} SYSPLLCTRL_REG;
_Static_assert(sizeof(SYSPLLCTRL_REG) == 4, "");

typedef struct SYSPLLSTAT_REG {
	const bool LOCK : 1;
	const unsigned _RES0 : 31;
} SYSPLLSTAT_REG;
_Static_assert(sizeof(SYSPLLSTAT_REG) == 4, "");

typedef enum SYSOSCCTRL_FREQRANGE {
	SYSOSCCTRL_FREQRANGE_1_20_MHZ = 0,
	SYSOSCCTRL_FREQRANGE_15_25_MHZ = 1,
} SYSOSCCTRL_FREQRANGE;

typedef struct SYSOSCCTRL_REG {
	bool BYPASS : 1;
	SYSOSCCTRL_FREQRANGE FREQRANGE : 1;
	const unsigned _RES0 : 30;
} SYSOSCCTRL_REG;
_Static_assert(sizeof(SYSOSCCTRL_REG) == 4, "");

typedef enum SYSPLLCLKSEL_SEL {
	SYSPLLCLKSEL_SEL_IRC = 0,
	SYSPLLCLKSEL_SEL_SYSTEM = 1,
} SYSPLLCLKSEL_SEL;

typedef struct SYSPLLCLKSEL_REG {
	SYSPLLCLKSEL_SEL SEL : 2;
	const unsigned _RES0 : 30;
} SYSPLLCLKSEL_REG;
_Static_assert(sizeof(SYSPLLCLKSEL_REG) == 4, "");

typedef struct SYSPLLCLKUEN_REG {
	bool ENA : 1;
	const unsigned _RES0 : 31;
} SYSPLLCLKUEN_REG;
_Static_assert(sizeof(SYSPLLCLKUEN_REG) == 4, "");

typedef enum MAINCLKSEL_SEL {
	MAINCLKSEL_SEL_IRC = 0,
	MAINCLKSEL_SEL_PLL_INPUT = 1,
	MAINCLKSEL_SEL_WDT = 2,
	MAINCLKSEL_SEL_PLL_OUTPUT = 3,
} MAINCLKSEL_SEL;

typedef struct MAINCLKSEL_REG {
	MAINCLKSEL_SEL SEL : 2;
	const unsigned _RES0 : 30;
} MAINCLKSEL_REG;
_Static_assert(sizeof(MAINCLKSEL_REG) == 4, "");

typedef struct MAINCLKUEN_REG {
	bool ENA : 1;
	const unsigned _RES0 : 31;
} MAINCLKUEN_REG;
_Static_assert(sizeof(MAINCLKUEN_REG) == 4, "");

typedef struct SYSAHBCLKDIV_REG {
	unsigned DIV : 8;
	const unsigned _RES0 : 24;
} SYSAHBCLKDIV_REG;
_Static_assert(sizeof(SYSAHBCLKDIV_REG) == 4, "");

typedef struct PDRUNCFG_REG {
	bool IRCOUT_PD : 1;
	bool IRC_PD : 1;
	bool FLASH_PD : 1;
	bool BOD_PD : 1;
	bool ADC_PD : 1;
	bool SYSOSC_PD : 1;
	bool WDTOSC_PD : 1;
	bool SYSPLL_PD : 1;
	const unsigned _RES0 : 7;
	bool COMP_PD : 1;
	const unsigned _RES1 : 16;
} PDRUNCFG_REG;
_Static_assert(sizeof(PDRUNCFG_REG) == 4, "");

typedef struct SYSCON_BLOCK {
	const unsigned _RES0;
	PRESETCTRL_REG PRESETCTRL;
	SYSPLLCTRL_REG SYSPLLCTRL;
	const SYSPLLSTAT_REG SYSPLLSTAT;
	const unsigned _RES1[4];
	SYSOSCCTRL_REG SYSOSCCTRL;
	const unsigned _RES2[7];
	SYSPLLCLKSEL_REG SYSPLLCLKSEL;
	SYSPLLCLKUEN_REG SYSPLLCLKUEN;
	const unsigned _RES3[10];
	MAINCLKSEL_REG MAINCLKSEL;
	MAINCLKUEN_REG MAINCLKUEN;
	SYSAHBCLKDIV_REG SYSAHBCLKDIV;
	const unsigned _RES4[111];
	PDRUNCFG_REG PDRUNCFG;
} SYSCON_BLOCK;
_Static_assert(offsetof(SYSCON_BLOCK, PRESETCTRL)   == 0x004, "");
_Static_assert(offsetof(SYSCON_BLOCK, SYSPLLCTRL)   == 0x008, "");
_Static_assert(offsetof(SYSCON_BLOCK, SYSPLLSTAT)   == 0x00C, "");
_Static_assert(offsetof(SYSCON_BLOCK, SYSOSCCTRL)   == 0x020, "");
_Static_assert(offsetof(SYSCON_BLOCK, SYSPLLCLKSEL) == 0x040, "");
_Static_assert(offsetof(SYSCON_BLOCK, SYSPLLCLKUEN) == 0x044, "");
_Static_assert(offsetof(SYSCON_BLOCK, MAINCLKSEL)   == 0x070, "");
_Static_assert(offsetof(SYSCON_BLOCK, MAINCLKUEN)   == 0x074, "");
_Static_assert(offsetof(SYSCON_BLOCK, SYSAHBCLKDIV) == 0x078, "");
_Static_assert(offsetof(SYSCON_BLOCK, PDRUNCFG)     == 0x238, "");

#define SYSCON (*(volatile SYSCON_BLOCK*)0x40048000)
