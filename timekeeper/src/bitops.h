#pragma once

#define BIT_VALUE(VAR, BIT) (VAR & (1u << (BIT)))

#define SET_BIT(VAR, BIT) do { VAR |= 1u << (BIT); } while (0)
#define CLEAR_BIT(VAR, BIT) do { VAR &= ~(1u << (BIT)); } while (0)
#define TOGGLE_BIT(VAR, BIT) do { VAR ^= 1u << (BIT); } while (0)
#define SET_BITS(VAR, START_BIT, END_BIT, VALUE) \
	do { \
		VAR = \
			(VAR & (~(((1u << ((END_BIT) - (START_BIT) + 1)) - 1) << (START_BIT)))) | \
			(((unsigned)(VALUE)) << (START_BIT)); \
	} while (0)
