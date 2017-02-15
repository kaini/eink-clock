#pragma once
#include "workarounds.h"
#include <stddef.h>
#include <assert.h>

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

static_assert(offsetof(struct CT16, IR)   == 0x00, "");
static_assert(offsetof(struct CT16, TCR)  == 0x04, "");
static_assert(offsetof(struct CT16, TC)   == 0x08, "");
static_assert(offsetof(struct CT16, PR)   == 0x0C, "");
static_assert(offsetof(struct CT16, PC)   == 0x10, "");
static_assert(offsetof(struct CT16, MCR)  == 0x14, "");
static_assert(offsetof(struct CT16, MR0)  == 0x18, "");
static_assert(offsetof(struct CT16, MR1)  == 0x1C, "");
static_assert(offsetof(struct CT16, MR2)  == 0x20, "");
static_assert(offsetof(struct CT16, MR3)  == 0x24, "");
static_assert(offsetof(struct CT16, CCR)  == 0x28, "");
static_assert(offsetof(struct CT16, CR0)  == 0x2C, "");
static_assert(offsetof(struct CT16, CR1)  == 0x30, "");
static_assert(offsetof(struct CT16, CR2)  == 0x34, "");
static_assert(offsetof(struct CT16, CR3)  == 0x38, "");
static_assert(offsetof(struct CT16, EMR)  == 0x3C, "");
static_assert(offsetof(struct CT16, CTCR) == 0x70, "");
static_assert(offsetof(struct CT16, PWMC) == 0x74, "");

#define CT16B0 (*(volatile struct CT16*)0x40010000)
#define CT16B1 (*(volatile struct CT16*)0x40014000)

// Interrupts
#define ISER (*(volatile unsigned int*)0xE000E100)
#define ICER (*(volatile unsigned int*)0xE000E180)

