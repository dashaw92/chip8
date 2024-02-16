#[derive(Debug, Default, Clone, Copy)]
pub struct Quirks {
    /// VF is reset to 0 for AND, OR, and XOR opcodes
    pub vf_reset: bool,
    /// PUSHREG and POPREG modify the value of I
    pub memory: bool,
    pub shifting: bool,
}

#[allow(private_interfaces)]
pub static QUIRKS_OLD: Quirks = Quirks {
    vf_reset: true,
    memory: false,
    shifting: false,
};

#[allow(private_interfaces)]
pub static QUIRKS_NEW: Quirks = Quirks {
    vf_reset: true,
    memory: true,
    shifting: false,
};