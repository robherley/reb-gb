use super::Error;

// https://gbdev.io/pandocs/Interrupts.html
pub const VBLANK: u8 = 0b0000_0001;
pub const LCD_STAT: u8 = 0b0000_0010;
pub const TIMER: u8 = 0b0000_0100;
pub const SERIAL: u8 = 0b0000_1000;
pub const JOYPAD: u8 = 0b0001_0000;

pub const HANDLERS: [u8; 5] = [VBLANK, LCD_STAT, TIMER, SERIAL, JOYPAD];

#[derive(Debug, Clone, Copy, PartialEq)]
enum StateChange {
    Delayed,
    Setting,
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
    /// Enables interrupt handling. Normally delayed one instruction. Optionally, it can be set to immediate.
    pub fn enable(&mut self, immediate: bool) {
        self.ei = if immediate {
            Some(StateChange::Setting)
        } else {
            Some(StateChange::Delayed)
        };
    }

    /// Disables interrupt handling. Delayed one instruction.
    pub fn disable(&mut self) {
        self.di = Some(StateChange::Delayed);
    }

    /// Updates the interrupt state. This should be called every instruction cycle.
    pub fn update(&mut self) {
        self.ei = match self.ei {
            Some(StateChange::Delayed) => Some(StateChange::Setting),
            Some(StateChange::Setting) => {
                self.ime = true;
                None
            }
            _ => None,
        };

        self.di = match self.di {
            Some(StateChange::Delayed) => Some(StateChange::Setting),
            Some(StateChange::Setting) => {
                self.ime = false;
                None
            }
            _ => None,
        };
    }

    /// Returns the first interrupt handler address that is enabled (IE) and requested (IF).
    pub fn requested(&self, ienable: u8, iflag: u8) -> Option<u8> {
        for handler in HANDLERS {
            // check if the interrupt is enabled
            if (ienable & handler) == 0 {
                continue;
            }

            // check if the interrupt is being requested
            if (iflag & handler) == 0 {
                continue;
            }
            // only one interrupt can be handled at a time
            return Some(handler);
        }

        None
    }
}

pub fn handler_address(interrupt: u8) -> Result<u16, Error> {
    match interrupt {
        VBLANK => Ok(0x0040),
        LCD_STAT => Ok(0x0048),
        TIMER => Ok(0x0050),
        SERIAL => Ok(0x0058),
        JOYPAD => Ok(0x0060),
        _ => Err(Error::InvalidInterrupt(interrupt)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interrupts_default() {
        let interrupts = Interrupts::default();
        assert!(!interrupts.ime);
        assert_eq!(interrupts.ei, None);
        assert_eq!(interrupts.di, None);
    }

    #[test]
    fn test_enable_immediate() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(true);
        assert_eq!(interrupts.ei, Some(StateChange::Setting));

        interrupts.update();
        assert!(interrupts.ime);
        assert_eq!(interrupts.ei, None);
    }

    #[test]
    fn test_enable_delayed() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(false);
        assert_eq!(interrupts.ei, Some(StateChange::Delayed));
        assert!(!interrupts.ime);

        interrupts.update();
        assert_eq!(interrupts.ei, Some(StateChange::Setting));
        assert!(!interrupts.ime);

        interrupts.update();
        assert!(interrupts.ime);
        assert_eq!(interrupts.ei, None);
    }

    #[test]
    fn test_disable() {
        let mut interrupts = Interrupts::default();
        interrupts.ime = true;
        interrupts.disable();
        assert_eq!(interrupts.di, Some(StateChange::Delayed));
        assert!(interrupts.ime);

        interrupts.update();
        assert_eq!(interrupts.di, Some(StateChange::Setting));
        assert!(interrupts.ime);

        interrupts.update();
        assert!(!interrupts.ime);
        assert_eq!(interrupts.di, None);
    }

    #[test]
    fn test_multiple_enable_calls() {
        let mut interrupts = Interrupts::default();

        // First enable call
        interrupts.enable(false);
        assert_eq!(interrupts.ei, Some(StateChange::Delayed));

        // Second enable call should overwrite
        interrupts.enable(true);
        assert_eq!(interrupts.ei, Some(StateChange::Setting));

        interrupts.update();
        assert!(interrupts.ime);
        assert_eq!(interrupts.ei, None);
    }

    #[test]
    fn test_multiple_disable_calls() {
        let mut interrupts = Interrupts::default();
        interrupts.ime = true;

        // First disable call
        interrupts.disable();
        assert_eq!(interrupts.di, Some(StateChange::Delayed));

        // Second disable call should overwrite (but same value)
        interrupts.disable();
        assert_eq!(interrupts.di, Some(StateChange::Delayed));

        interrupts.update();
        assert_eq!(interrupts.di, Some(StateChange::Setting));

        interrupts.update();
        assert!(!interrupts.ime);
    }

    #[test]
    fn test_enable_when_already_enabled() {
        let mut interrupts = Interrupts::default();
        interrupts.ime = true;

        interrupts.enable(false);
        assert_eq!(interrupts.ei, Some(StateChange::Delayed));

        interrupts.update();
        interrupts.update();
        assert!(interrupts.ime);
        assert_eq!(interrupts.ei, None);
    }

    #[test]
    fn test_disable_when_already_disabled() {
        let mut interrupts = Interrupts::default();

        interrupts.disable();
        assert_eq!(interrupts.di, Some(StateChange::Delayed));

        interrupts.update();
        interrupts.update();
        assert!(!interrupts.ime);
        assert_eq!(interrupts.di, None);
    }

    #[test]
    fn test_update_with_no_pending_changes() {
        let mut interrupts = Interrupts::default();
        let initial_ime = interrupts.ime;

        interrupts.update();
        assert_eq!(interrupts.ime, initial_ime);
        assert_eq!(interrupts.ei, None);
        assert_eq!(interrupts.di, None);
    }

    #[test]
    fn test_simultaneous_enable_disable() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(false);
        interrupts.disable();

        // Both pending, update should process both
        interrupts.update();
        assert_eq!(interrupts.ei, Some(StateChange::Setting));
        assert_eq!(interrupts.di, Some(StateChange::Setting));

        interrupts.update();
        assert!(!interrupts.ime); // disable takes effect
        assert_eq!(interrupts.ei, None);
        assert_eq!(interrupts.di, None);
    }

    #[test]
    fn test_requested() {
        let mut interrupts = Interrupts::default();
        interrupts.ime = true;

        let result = interrupts.requested(VBLANK, VBLANK);
        assert_eq!(result, Some(VBLANK));

        let result = interrupts.requested(LCD_STAT, LCD_STAT);
        assert_eq!(result, Some(LCD_STAT));
    }

    #[test]
    fn test_requested_partial_match() {
        let interrupts = Interrupts::default();

        // Enable VBLANK and LCD_STAT, but only flag VBLANK
        let result = interrupts.requested(VBLANK | LCD_STAT, VBLANK);
        assert_eq!(result, Some(VBLANK));

        // Enable VBLANK and LCD_STAT, but only flag LCD_STAT
        let result = interrupts.requested(VBLANK | LCD_STAT, LCD_STAT);
        assert_eq!(result, Some(LCD_STAT));
    }

    #[test]
    fn test_requested_priority_order() {
        let interrupts = Interrupts::default();

        // Multiple interrupts enabled and flagged - should return VBLANK (highest priority)
        let ienable = VBLANK | LCD_STAT | TIMER;
        let iflag = VBLANK | LCD_STAT | TIMER;
        let result = interrupts.requested(ienable, iflag);
        assert_eq!(result, Some(VBLANK));

        // Only lower priority interrupts
        let ienable = LCD_STAT | TIMER | JOYPAD;
        let iflag = LCD_STAT | TIMER | JOYPAD;
        let result = interrupts.requested(ienable, iflag);
        assert_eq!(result, Some(LCD_STAT));

        // Test individual priorities
        let result = interrupts.requested(TIMER | JOYPAD, TIMER | JOYPAD);
        assert_eq!(result, Some(TIMER));

        let result = interrupts.requested(SERIAL | JOYPAD, SERIAL | JOYPAD);
        assert_eq!(result, Some(SERIAL));

        let result = interrupts.requested(JOYPAD, JOYPAD);
        assert_eq!(result, Some(JOYPAD));
    }

    #[test]
    fn test_requested_all_interrupts_at_once() {
        let interrupts = Interrupts::default();
        let all_interrupts = VBLANK | LCD_STAT | TIMER | SERIAL | JOYPAD;
        let result = interrupts.requested(all_interrupts, all_interrupts);
        assert_eq!(result, Some(VBLANK));
    }

    #[test]
    fn test_requested_no_interrupts() {
        let interrupts = Interrupts::default();

        let result = interrupts.requested(0x00, 0x00);
        assert_eq!(result, None);
    }

    #[test]
    fn test_requested_unknown_interrupt_bits() {
        let interrupts = Interrupts::default();

        let result = interrupts.requested(0b1110_0000, 0b1110_0000);
        assert_eq!(result, None);
    }
}
