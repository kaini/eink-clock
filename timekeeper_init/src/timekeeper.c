#include "debug.h"
#include "devices/cpu.h"
#include "devices/dcf77.h"

extern int it_works(int, int);

int main() {
	DEBUG_PRINT("Timekeeper\n");

	it_works(10, 2);

	cpu_init();
	dcf77_init();

	bool payload[DCF77_PAYLOAD_BITS];
	dcf77_receive(payload);
	for (int i = 0; i < DCF77_PAYLOAD_BITS; ++i) {
		DEBUG_PRINT("%d", payload[i]);
	}
	DEBUG_PRINT("\n");

	DEBUG_PRINT("I'm done!\n");
	return 0;
}
