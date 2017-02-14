#pragma once
#include <stdio.h>

#ifdef DEBUG
#define DEBUG_PRINT(...) do { printf(__VA_ARGS__); } while (0)
#else
#define DEBUG_PRINT(...) do { } while (0)
#endif

#define TRACE() do { DEBUG_PRINT("%s (%s:%d)\n", __func__, __FILE__, __LINE__); } while (0)
