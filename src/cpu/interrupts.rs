// https://gbdev.io/pandocs/Interrupts.html
const VBLANK: (u8, u16) = (0b0000_0001, 0x40);
const LCD_STAT: (u8, u16) = (0b0000_0010, 0x48);
const TIMER: (u8, u16) = (0b0000_0100, 0x50);
const SERIAL: (u8, u16) = (0b0000_1000, 0x58);
const JOYPAD: (u8, u16) = (0b0001_0000, 0x60);

const HANDLERS: [(u8, u16); 5] = [VBLANK, LCD_STAT, TIMER, SERIAL, JOYPAD];

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
    pub fn requested(&self, ienable: u8, iflag: u8) -> Option<(u8, u16)> {
        if !self.ime {
            return None;
        }

        for (handler, addr) in HANDLERS {
            // check if the interrupt is enabled
            if (ienable & handler) == 0 {
                continue;
            }

            // check if the interrupt is being requested
            if (iflag & handler) == 0 {
                continue;
            }

            // only one interrupt can be handled at a time
            return Some((handler, addr));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interrupts_default() {
        let interrupts = Interrupts::default();
        assert!(!interrupts.ime);
        assert!(interrupts.ei.is_none());
        assert!(interrupts.di.is_none());
    }

    #[test]
    fn test_enable_interrupts_immediate() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(true);
        assert_eq!(interrupts.ei, Some(StateChange::Setting));
    }

    #[test]
    fn test_enable_interrupts_delayed() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(false);
        assert_eq!(interrupts.ei, Some(StateChange::Delayed));
    }

    #[test]
    fn test_disable_interrupts() {
        let mut interrupts = Interrupts::default();
        interrupts.disable();
        assert_eq!(interrupts.di, Some(StateChange::Delayed));
    }

    #[test]
    fn test_update_interrupts_enable() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(false);
        interrupts.update();
        assert_eq!(interrupts.ei, Some(StateChange::Setting));
        interrupts.update();
        assert!(interrupts.ei.is_none());
        assert!(interrupts.ime);
    }

    #[test]
    fn test_update_interrupts_disable() {
        let mut interrupts = Interrupts::default();
        interrupts.disable();
        interrupts.update();
        assert_eq!(interrupts.di, Some(StateChange::Setting));
        interrupts.update();
        assert!(interrupts.di.is_none());
        assert!(!interrupts.ime);
    }

    #[test]
    fn test_requested_interrupt_none_when_ime_disabled() {
        let interrupts = Interrupts::default();
        let result = interrupts.requested(0b1111_1111, 0b1111_1111);
        assert!(result.is_none());
    }

    #[test]
    fn test_requested_interrupt_none_when_no_interrupts_requested() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(true);
        interrupts.update();
        let result = interrupts.requested(0b1111_1111, 0b0000_0000);
        assert!(result.is_none());
    }

    #[test]
    fn test_requested_interrupt_none_when_no_interrupts_enabled() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(true);
        interrupts.update();
        let result = interrupts.requested(0b0000_0000, 0b1111_1111);
        assert!(result.is_none());
    }

    #[test]
    fn test_requested_interrupt_vblank() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(true);
        interrupts.update();
        let result = interrupts.requested(0b0000_0001, 0b0000_0001);
        assert_eq!(result, Some(VBLANK));
    }

    #[test]
    fn test_requested_interrupt_lcd_stat() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(true);
        interrupts.update();
        let result = interrupts.requested(0b0000_0010, 0b0000_0010);
        assert_eq!(result, Some(LCD_STAT));
    }

    #[test]
    fn test_requested_interrupt_prioritization() {
        let mut interrupts = Interrupts::default();
        interrupts.enable(true);
        interrupts.update();
        let result = interrupts.requested(0b1111_1111, 0b0000_0011); // Both VBLANK and LCD_STAT requested
        assert_eq!(result, Some(VBLANK)); // VBLANK has higher priority
    }
}
