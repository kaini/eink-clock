#include "dcf77.h"
#include "bitops.h"
#include "debug.h"
#include "rawhw/hw.h"
#include <stdbool.h>

// Signal pin. This is CT16B0_CAP0.
#define SIGNAL_IOCONFIG (PIO0_11)
#define SIGNAL_GPIO (GPIO0)
#define SIGNAL_BIT (11)

// Power pin. Must be a high drive PIO pin.
#define POWER_IOCONFIG (PIO0_29)
#define POWER_GPIO (GPIO0)
#define POWER_BIT (29)

// The 16 bit counter to use
#define COUNTER (CT16B0)

static volatile bool interrupt_done = false;

static void enable_module() {
	SET_BIT(POWER_GPIO.OUT, POWER_BIT);
	WRITE_ONE(ISER, 13);  // Enable 16 bit timer 0 interrupts
	SET_BIT(COUNTER.TCR, 0);  // Start counter
}

static void disable_module() {
	CLEAR_BIT(COUNTER.TCR, 0);  // stop the counter
	WRITE_ONE(ICER, 13);  // disable the interrupt
	CLEAR_BIT(POWER_GPIO.OUT, POWER_BIT);
}

static void wait_for_interrupt() {
	for (interrupt_done = false; !interrupt_done;) {
		__asm("WFI");
	}
}

static int wait_for_raising_edge() {
	SET_BIT(COUNTER.CCR, 0);  // Capture on raising edge in CR0
	CLEAR_BIT(COUNTER.CCR, 1);  // Do not capture on falling edge in CR0
	wait_for_interrupt();
	return COUNTER.CR0 & 0xFFFF;
}

static int wait_for_falling_edge() {
	CLEAR_BIT(COUNTER.CCR, 0);  // Do not capture on raising edge in CR0
	SET_BIT(COUNTER.CCR, 1);  // Capture on falling edge in CR0
	wait_for_interrupt();
	return COUNTER.CR0 & 0xFFFF;
}

static int measure_next_low_ms() {
	SET_BIT(COUNTER.TCR, 1);  // Reset & start counter
	CLEAR_BIT(COUNTER.TCR, 1);
	const int fall_time = wait_for_falling_edge();
	const int raise_time = wait_for_raising_edge();
	return raise_time - fall_time;
}

static int measure_next_high_ms() {
	SET_BIT(COUNTER.TCR, 1);  // Reset & start counter
	CLEAR_BIT(COUNTER.TCR, 1);
	const int raise_time = wait_for_raising_edge();
	const int fall_time = wait_for_falling_edge();
	return fall_time - raise_time;
}

static void wait_for_module() {
	DEBUG_PRINT("wait_for_module\n");
	wait_for_raising_edge();
	wait_for_raising_edge();
	wait_for_raising_edge();
}

static void wait_for_signal_start() {
	DEBUG_PRINT("wait_for_signal_start\n");
	while (measure_next_low_ms() < 1600)
		;
}

static void receive_payload(bool result[static DCF77_PAYLOAD_BITS]) {
	DEBUG_PRINT("receive_payload\n");
	for (int i = 0; i < DCF77_PAYLOAD_BITS; ++i) {
		result[i] = measure_next_high_ms() > 170;
	}
}

void dcf77_init() {
	DEBUG_PRINT("init_dcf77\n");

	// Configure SIGNAL pin
	SET_BITS(SIGNAL_IOCONFIG, 0, 2, 0x3);  // use as CT16B0_CAP0

	// Configure POWER pin
	CLEAR_BIT(POWER_IOCONFIG, 4);  // disable pullup
	SET_BIT(POWER_IOCONFIG, 9);  // high drive mode
	SET_BIT(POWER_GPIO.DIR, POWER_BIT);  // output

	// Setup the counter
	SET_BITS(COUNTER.PR, 0, 15, 35999);  // prescale by 36000, this leads to 1000 ticks per second
	SET_BIT(COUNTER.CCR, 2);  // Interrupt on CR0 writes

	disable_module();
}

// TODO timeout
void dcf77_receive(bool payload[static DCF77_PAYLOAD_BITS]) {
	DEBUG_PRINT("dcf77_receive\n");
	enable_module();
	wait_for_module();
	wait_for_signal_start();
	receive_payload(payload);
	disable_module();
}

void TIMER16_0_IRQHandler() {
	if (BIT_VALUE(COUNTER.IR, 4)) {  // if CR0INT
		interrupt_done = true;
		WRITE_ONE(COUNTER.IR, 4);  // clear the interrupt
	}
}
