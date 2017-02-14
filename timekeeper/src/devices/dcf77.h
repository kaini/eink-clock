#pragma once
#include <stdbool.h>

#define DCF77_PAYLOAD_BITS (59)

void dcf77_init();
void dcf77_receive(bool payload[static DCF77_PAYLOAD_BITS]);
