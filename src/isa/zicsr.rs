#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsrInstruction {
	Csrrw { rd: u8, rs1_or_uimm: u8, csr: u16 },
	Csrrs { rd: u8, rs1_or_uimm: u8, csr: u16 },
	Csrrc { rd: u8, rs1_or_uimm: u8, csr: u16 },
	Csrrwi { rd: u8, rs1_or_uimm: u8, csr: u16 },
	Csrrsi { rd: u8, rs1_or_uimm: u8, csr: u16 },
	Csrrci { rd: u8, rs1_or_uimm: u8, csr: u16 },
}
