#pragma once
#include "workarounds.h"
#include <assert.h>
#include <stddef.h>

struct GPIO {
	unsigned int MASK;
	const unsigned int PIN;
	unsigned int OUT;
	unsigned int SET;
	unsigned int CLR;
	unsigned int NOT;
	const char _PAD0[8];
	unsigned int DIR;
	unsigned int IS;
	unsigned int IBE;
	unsigned int IEV;
	unsigned int IE;
	const unsigned int RIS;
	const unsigned int MIS;
	unsigned int IC;
};
static_assert(offsetof(struct GPIO, MASK) == 0x00, "");
static_assert(offsetof(struct GPIO, PIN)  == 0x04, "");
static_assert(offsetof(struct GPIO, OUT)  == 0x08, "");
static_assert(offsetof(struct GPIO, SET)  == 0x0C, "");
static_assert(offsetof(struct GPIO, CLR)  == 0x10, "");
static_assert(offsetof(struct GPIO, NOT)  == 0x14, "");
static_assert(offsetof(struct GPIO, DIR)  == 0x20, "");
static_assert(offsetof(struct GPIO, IS)   == 0x24, "");
static_assert(offsetof(struct GPIO, IBE)  == 0x28, "");
static_assert(offsetof(struct GPIO, IEV)  == 0x2C, "");
static_assert(offsetof(struct GPIO, IE)   == 0x30, "");
static_assert(offsetof(struct GPIO, RIS)  == 0x34, "");
static_assert(offsetof(struct GPIO, MIS)  == 0x38, "");
static_assert(offsetof(struct GPIO, IC)   == 0x3C, "");

#define GPIO0 (*(volatile struct GPIO*)0x50000000)
#define GPIO1 (*(volatile struct GPIO*)0x50010000)
