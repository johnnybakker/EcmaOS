#![no_std]
#![no_main]
use ecma_pi::{prelude::*, cpu::delay};

// const RED: u32 = 23;
// const GREEN: u32 = 24;
// const BLUE: u32 = 25;
// const YELLOW: u32 = 26;
const BAUDRATE: usize = 115200;

#[no_mangle]
pub fn main() -> ! {

	// Initialize mini uart with a baudrate of 115200
	auxiliary::mini_uart_init(BAUDRATE);
	auxiliary::mini_uart_send_string("Mini uart initialized with baudrate: ");
	auxiliary::mini_uart_send_num(BAUDRATE);
	auxiliary::mini_uart_send('\n');

	auxiliary::mini_uart_send_num(usize::MAX);
	auxiliary::mini_uart_send('\n');

	// Configure all LED pin's as output
	// gpio::gpio_select_function(RED, gpio::GpioFn::OUTPUT);
	// gpio::gpio_select_function(GREEN, gpio::GpioFn::OUTPUT);
	// gpio::gpio_select_function(BLUE, gpio::GpioFn::OUTPUT);
	// gpio::gpio_select_function(YELLOW, gpio::GpioFn::OUTPUT);

	// Turn off all LED's
	// gpio::gpio_output_clear(RED);
	// gpio::gpio_output_clear(GREEN);
	// gpio::gpio_output_clear(BLUE);
	// gpio::gpio_output_clear(YELLOW);

	// Write Hello, world!
	auxiliary::mini_uart_send_string("Hello, world!\n");
	
	loop {
		auxiliary::mini_uart_send_string("Hello, world!\n");
		delay(250_000);

		// auxiliary::mini_uart_send_string("RED");
		// gpio::gpio_output_clear(YELLOW);
		// gpio::gpio_output_set(RED);
		// auxiliary::mini_uart_receive();
	
		// auxiliary::mini_uart_send_string(" GREEN");
		// gpio::gpio_output_clear(RED);
		// gpio::gpio_output_set(GREEN);
		// auxiliary::mini_uart_receive();

		// auxiliary::mini_uart_send_string(" BLUE");
		// gpio::gpio_output_clear(GREEN);
		// gpio::gpio_output_set(BLUE);
		// auxiliary::mini_uart_receive();

		// auxiliary::mini_uart_send_string(" YELLOW\n");
		// gpio::gpio_output_clear(BLUE);
		// gpio::gpio_output_set(YELLOW);
		// auxiliary::mini_uart_receive();
	}
}