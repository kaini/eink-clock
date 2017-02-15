#pragma once
#include "workarounds.h"
#include <assert.h>

typedef struct IOCON_REG {
	unsigned FUNC : 3;
	const unsigned _RES0 : 1;
	unsigned MODE : 1;
	const unsigned _RES1 : 1;
	unsigned INV : 1;
	unsigned ADMODE : 1;
	const unsigned _RES2 : 1;
	unsigned DRV : 1;
	unsigned OD : 1;
	unsigned S_MODE : 2;
	unsigned CLK_DIV : 3;
	const unsigned _RES3 : 16;
} IOCON_REG;
static_assert(sizeof(IOCON_REG) == 4, "");

typedef struct IOCON_I2C_REG {
	unsigned FUNC : 3;
	const unsigned _RES0 : 3;
	unsigned INV : 1;
	const unsigned _RES1 : 3;
	unsigned TOD : 1;
	unsigned S_MODE : 2;
	unsigned CLK_DIV : 3;
	const unsigned _RES2 : 16;
} IOCON_I2C_REG;
static_assert(sizeof(IOCON_I2C_REG) == 4, "");

typedef struct IOCONFIG_BLOCK {
	const unsigned _RES0[16];
	IOCON_REG PIO0_29;
	const unsigned _RES1[19];
	IOCON_I2C_REG PIO0_10;
	IOCON_I2C_REG PIO0_11;
} IOCONFIG_BLOCK;
static_assert(offsetof(IOCONFIG_BLOCK, PIO0_29) == 0x40, "");
static_assert(offsetof(IOCONFIG_BLOCK, PIO0_10) == 0x90, "");
static_assert(offsetof(IOCONFIG_BLOCK, PIO0_11) == 0x94, "");

#define IOCONFIG (*(volatile IOCONFIG_BLOCK*)0x40044000)
