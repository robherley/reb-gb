// https://gbdev.io/pandocs/Interrupts.html
pub enum Handler {
    VBlank = 1,
    Lcd = 2,
    Timer = 4,
    Serial = 8,
    Joypad = 16,
}

impl Handler {
    // https://gbdev.io/pandocs/Interrupt_Sources.html#interrupt-sources
    pub fn src(&self) -> u16 {
        match self {
            Handler::VBlank => 0x40,
            Handler::Lcd => 0x48,
            Handler::Timer => 0x50,
            Handler::Serial => 0x58,
            Handler::Joypad => 0x60,
        }
    }
}

enum StateChange {
    Delayed,
    Set,
}

pub struct Interrupts {
    /// IME: InterruptMasterEnable is used to disabled all interrupts on the IE register
    pub ime: bool,
    /// EI: sets ime to be enabled (delayed one instruction)
    ei: Option<StateChange>,
    /// DI: sets ime to be disabled (delayed one instruction)
    di: Option<StateChange>,
}

impl Default for Interrupts {
    fn default() -> Self {
        Self {
            ime: false,
            di: None,
            ei: None,
        }
    }
}

impl Interrupts {
    pub fn enable(&mut self, immediate: bool) {
        self.ei = if immediate {
            Some(StateChange::Set)
        } else {
            Some(StateChange::Delayed)
        };
    }

    pub fn disable(&mut self) {
        self.di = Some(StateChange::Delayed);
    }

    pub fn update(&mut self) {
        self.ei = match self.ei {
            Some(StateChange::Delayed) => Some(StateChange::Set),
            Some(StateChange::Set) => {
                self.ime = true;
                None
            }
            _ => None,
        };

        self.di = match self.di {
            Some(StateChange::Delayed) => Some(StateChange::Set),
            Some(StateChange::Set) => {
                self.ime = false;
                None
            }
            _ => None,
        };
    }
}
