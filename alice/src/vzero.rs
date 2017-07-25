use alice_sys::ESD;

#[derive(Debug)]
pub struct V0 {
    multiplicity_v0a: [f32; 32],
    multiplicity_v0c: [f32; 32],
    beam_beam_flag_v0a: [bool; 32],
    beam_beam_flag_v0c: [bool; 32],
    beam_gas_flag_v0a: [bool; 32],
    beam_gas_flag_v0c: [bool; 32],
}

impl V0 {
    pub fn from_esd(esd: &ESD) -> V0 {
        let mut mult_v0a: [f32; 32] = Default::default();
        mult_v0a.copy_from_slice(&esd.AliESDVZERO_fMultiplicity[..32]);

        let mut mult_v0c: [f32; 32] = Default::default();
        mult_v0c.copy_from_slice(&esd.AliESDVZERO_fMultiplicity[32..]);
        
        let mut bb_v0a: [bool; 32] = Default::default();
        bb_v0a.copy_from_slice(&esd.AliESDVZERO_fBBFlag[..32]);

        let mut bb_v0c: [bool; 32] = Default::default();
        bb_v0c.copy_from_slice(&esd.AliESDVZERO_fBBFlag[32..]);

        let mut bg_v0a: [bool; 32] = Default::default();
        bg_v0a.copy_from_slice(&esd.AliESDVZERO_fBGFlag[..32]);

        let mut bg_v0c: [bool; 32] = Default::default();
        bg_v0c.copy_from_slice(&esd.AliESDVZERO_fBGFlag[32..]);

        V0 {
            multiplicity_v0a: mult_v0a,
            multiplicity_v0c: mult_v0c,
            beam_beam_flag_v0a: bb_v0a,
            beam_beam_flag_v0c: bb_v0c,
            beam_gas_flag_v0a: bg_v0a,
            beam_gas_flag_v0c: bg_v0c,
        }
    }

    pub fn is_beam_beam(&self) -> bool {
        // Any true is good enough; see AliTriggerAnalysis:814
        self.beam_beam_flag_v0a.iter()
            .chain(self.beam_beam_flag_v0c.iter())
            .any(|p| *p)
    }
    pub fn is_beam_gas(&self) -> bool {
        // Any true is good enough; see AliTriggerAnalysis:814
        self.beam_gas_flag_v0a.iter()
            .chain(self.beam_gas_flag_v0c.iter())
            .any(|p| *p)
    }

    pub fn multiplicity_v0a(&self) ->f32 {
        self.multiplicity_v0a.iter().fold(0., |sum, seg| sum + seg)
    }
    pub fn multiplicity_v0c(&self) ->f32 {
        self.multiplicity_v0c.iter().fold(0., |sum, seg| sum + seg)
    }
}
