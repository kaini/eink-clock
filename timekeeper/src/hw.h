#pragma once

// SYSCON
#define PRESETCTRL   (*(volatile unsigned int*)0x40048004)
#define SYSPLLCTRL   (*(volatile unsigned int*)0x40048008)
#define SYSPLLSTAT   (*(volatile unsigned int*)0x4004800C)
#define SYSOSCCTRL   (*(volatile unsigned int*)0x40048020)
#define SYSPLLCLKSEL (*(volatile unsigned int*)0x40048040)
#define SYSPLLCLKUEN (*(volatile unsigned int*)0x40048044)
#define MAINCLKSEL   (*(volatile unsigned int*)0x40048070)
#define MAINCLKUEN   (*(volatile unsigned int*)0x40048074)
#define SYSAHBCLKDIV (*(volatile unsigned int*)0x40048078)
#define PDRUNCFG     (*(volatile unsigned int*)0x40048238)
