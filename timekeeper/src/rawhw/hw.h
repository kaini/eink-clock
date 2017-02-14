#pragma once
#include "workarounds.h"
#include <stddef.h>

// IOCONFIG
#define PIO0_19 (*(volatile unsigned int*)0x40044008)
#define PIO0_29 (*(volatile unsigned int*)0x40044040)
#define PIO0_11 (*(volatile unsigned int*)0x40044096)

// GPIO
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

_Static_assert(offsetof(struct GPIO, MASK) == 0x00, "");
_Static_assert(offsetof(struct GPIO, PIN)  == 0x04, "");
_Static_assert(offsetof(struct GPIO, OUT)  == 0x08, "");
_Static_assert(offsetof(struct GPIO, SET)  == 0x0C, "");
_Static_assert(offsetof(struct GPIO, CLR)  == 0x10, "");
_Static_assert(offsetof(struct GPIO, NOT)  == 0x14, "");
_Static_assert(offsetof(struct GPIO, DIR)  == 0x20, "");
_Static_assert(offsetof(struct GPIO, IS)   == 0x24, "");
_Static_assert(offsetof(struct GPIO, IBE)  == 0x28, "");
_Static_assert(offsetof(struct GPIO, IEV)  == 0x2C, "");
_Static_assert(offsetof(struct GPIO, IE)   == 0x30, "");
_Static_assert(offsetof(struct GPIO, RIS)  == 0x34, "");
_Static_assert(offsetof(struct GPIO, MIS)  == 0x38, "");
_Static_assert(offsetof(struct GPIO, IC)   == 0x3C, "");

#define GPIO0 (*(volatile struct GPIO*)0x50000000)
#define GPIO1 (*(volatile struct GPIO*)0x50010000)

// 16-bit counter
struct CT16 {
	unsigned int IR;
	unsigned int TCR;
	unsigned int TC;
	unsigned int PR;
	unsigned int PC;
	unsigned int MCR;
	unsigned int MR0;
	unsigned int MR1;
	unsigned int MR2;
	unsigned int MR3;
	unsigned int CCR;
	const unsigned int CR0;
	const unsigned int CR1;
	const unsigned int CR2;
	const unsigned int CR3;
	unsigned int EMR;
	const char _PAD0[0x30];
	unsigned int CTCR;
	unsigned int PWMC;
};

_Static_assert(offsetof(struct CT16, IR)   == 0x00, "");
_Static_assert(offsetof(struct CT16, TCR)  == 0x04, "");
_Static_assert(offsetof(struct CT16, TC)   == 0x08, "");
_Static_assert(offsetof(struct CT16, PR)   == 0x0C, "");
_Static_assert(offsetof(struct CT16, PC)   == 0x10, "");
_Static_assert(offsetof(struct CT16, MCR)  == 0x14, "");
_Static_assert(offsetof(struct CT16, MR0)  == 0x18, "");
_Static_assert(offsetof(struct CT16, MR1)  == 0x1C, "");
_Static_assert(offsetof(struct CT16, MR2)  == 0x20, "");
_Static_assert(offsetof(struct CT16, MR3)  == 0x24, "");
_Static_assert(offsetof(struct CT16, CCR)  == 0x28, "");
_Static_assert(offsetof(struct CT16, CR0)  == 0x2C, "");
_Static_assert(offsetof(struct CT16, CR1)  == 0x30, "");
_Static_assert(offsetof(struct CT16, CR2)  == 0x34, "");
_Static_assert(offsetof(struct CT16, CR3)  == 0x38, "");
_Static_assert(offsetof(struct CT16, EMR)  == 0x3C, "");
_Static_assert(offsetof(struct CT16, CTCR) == 0x70, "");
_Static_assert(offsetof(struct CT16, PWMC) == 0x74, "");

#define CT16B0 (*(volatile struct CT16*)0x40010000)
#define CT16B1 (*(volatile struct CT16*)0x40014000)

// Interrupts
#define ISER (*(volatile unsigned int*)0xE000E100)
#define ICER (*(volatile unsigned int*)0xE000E180)

