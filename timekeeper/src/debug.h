#pragma once
#include <stdio.h>

#ifdef DEBUG
#define DEBUG_PRINT(...) do { printf(__VA_ARGS__); } while (0)
#else
#define DEBUG_PRINT(...) do { } while (0)
#endif
