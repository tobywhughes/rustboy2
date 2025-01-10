pub enum MemoryLocation {
    Bank0,
    BankN,
    VRam,
    ExternalRam,
    WorkRamBank0,
    WorkRamBankN,
    EchoRam,
    Oam,
    NotUsed,
    IO,
    HRam,
    InterruptEnableRegister,
}

impl MemoryLocation {
    pub fn parse_address(address: u16) -> MemoryLocation {
        match address {
            0x0000..=0x3FFF => MemoryLocation::Bank0,
            0x4000..=0x7FFF => MemoryLocation::BankN,
            0x8000..=0x9FFF => MemoryLocation::VRam,
            0xA000..=0xBFFF => MemoryLocation::ExternalRam,
            0xC000..=0xCFFF => MemoryLocation::WorkRamBank0,
            0xD000..=0xDFFF => MemoryLocation::WorkRamBankN,
            0xE000..=0xFDFF => MemoryLocation::EchoRam,
            0xFE00..=0xFE9F => MemoryLocation::Oam,
            0xFEA0..=0xFEFF => MemoryLocation::NotUsed,
            0xFF00..=0xFF7F => MemoryLocation::IO,
            0xFF80..=0xFFFE => MemoryLocation::HRam,
            0xFFFF => MemoryLocation::InterruptEnableRegister,
            _ => panic!("Invalid Memory Location")
        }
    }
}