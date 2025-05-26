use super::Memory;

/// Timer (and divider) registers
/// https://gbdev.io/pandocs/Timer_and_Divider_Registers.html
pub struct Timer {
    /// If true, the timer interrupt is requested.
    pub interrupt: bool,
    /// FF04 — DIV: Divider register.
    div: Counter,
    /// FF05 - TIMA: Timer counter.
    /// This also holds the FF07 TAC (timer control) state.
    tima: Counter,
    /// FF06 — TMA: Timer modulo.
    tma: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            interrupt: false,
            div: Counter::new(256, true),
            tima: Counter::new(1024, false),
            tma: 0,
        }
    }

    pub fn ticks(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.tick();
        }
    }

    /// Advances the timer by one tick (1 t-state).
    /// If the TIMA register overflows, it is reset to the TMA value and an interrupt is requested.
    fn tick(&mut self) {
        self.div.tick();
        if self.tima.tick() {
            // tima overflowed, reset to tma
            self.tima.value = self.tma;
            // on tima overflow, request timer interrupt
            self.interrupt = true;
        }
    }

    /// Timer control register at 0xFF07
    ///
    /// | 7 | 6 | 5 | 4 | 3 | 2       | 1            | 0            |
    /// | - | - | - | - | - | ------- | ------------ | ------------ |
    /// |   |   |   |   |   | Enable  | Clock select | Clock select |
    ///
    /// - Bits 7-3: Unused
    /// - Bit 2: Timer Enable (1 = Enable, 0 = Disable)
    /// - Bits 1-0: Clock Select
    ///   - 00: 4096 Hz (1024 t-states)
    ///   - 01: 262144 Hz (16 t-states)
    ///   - 10: 65536 Hz (64 t-states)
    ///   - 11: 16384 Hz (256 t-states)
    fn write_tac(&mut self, value: u8) {
        self.tima.enabled = (value & 0b0100) != 0;
        self.tima.every = match value & 0b0011 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!(),
        }
    }

    fn read_tac(&self) -> u8 {
        let mut value = 0b1111_0000;
        if self.tima.enabled {
            value |= 0b0100;
        }

        value
            | match self.tima.every {
                1024 => 0b00,
                16 => 0b01,
                64 => 0b10,
                256 => 0b11,
                _ => unreachable!(),
            }
    }
}

impl Memory for Timer {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div.value,
            0xFF05 => self.tima.value,
            0xFF06 => self.tma,
            0xFF07 => self.read_tac(),
            _ => panic!("invalid timer read: {:#06x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div.value = 0,
            0xFF05 => self.tima.value = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.write_tac(value),
            _ => panic!("invalid timer write: {:#06x}", address),
        }
    }
}

/// A simple counter that increments every `every` ticks and overflows back to 0.
/// It can be enabled or disabled, and it will not increment when disabled.
struct Counter {
    count: u16,
    pub value: u8,
    pub every: u16,
    pub enabled: bool,
}

impl Counter {
    pub fn new(every: u16, enabled: bool) -> Self {
        Counter {
            count: 0,
            value: 0,
            every,
            enabled,
        }
    }

    /// Increments the counter and returns true if it has overflowed
    pub fn tick(&mut self) -> bool {
        if !self.enabled {
            return false;
        }

        self.count += 1;
        if self.count >= self.every {
            self.count = 0;
            self.value = self.value.wrapping_add(1);
            if self.value == 0x00 {
                return true;
            }
        }
        return false;
    }
}
