bitflags! {
    pub struct Trigger: u64 {
        const MINIMUM_BIAS = 0b00000001;
        const HIGH_MULT =   0b00000010;
    }
}

impl Trigger {
    pub fn new_from_str(s: &str, run_number: u32) -> Trigger {
        // LHC10h
        if 136851 <= run_number && run_number <= 139517 {
            match s {
                "CMBAC-B-NOPF-ALL" => MINIMUM_BIAS,
                "CMBS2A-B-NOPF-ALL"  => MINIMUM_BIAS,
                "CMBS2C-B-NOPF-ALL"  => MINIMUM_BIAS,
                "CMBACS2-B-NOPF-ALL" => MINIMUM_BIAS,
                "CMBACS2-B-NOPF-ALLNOTRD" => MINIMUM_BIAS,
                "C0SMH-B-NOPF-ALL" => HIGH_MULT,
                "C0SMH-B-NOPF-ALLNOTRD" => HIGH_MULT,
                _ => Trigger::empty(),
            }
        } else {
            Trigger::empty()
        }
    }
}
