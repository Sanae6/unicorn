#![allow(non_camel_case_types)]
// For Unicorn Engine. AUTO-GENERATED FILE, DO NOT EDIT

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RegisterARM {
    // ARM registers
    INVALID = 0,
    APSR = 1,
    APSR_NZCV = 2,
    CPSR = 3,
    FPEXC = 4,
    FPINST = 5,
    FPSCR = 6,
    FPSCR_NZCV = 7,
    FPSID = 8,
    ITSTATE = 9,
    LR = 10,
    PC = 11,
    SP = 12,
    SPSR = 13,
    D0 = 14,
    D1 = 15,
    D2 = 16,
    D3 = 17,
    D4 = 18,
    D5 = 19,
    D6 = 20,
    D7 = 21,
    D8 = 22,
    D9 = 23,
    D10 = 24,
    D11 = 25,
    D12 = 26,
    D13 = 27,
    D14 = 28,
    D15 = 29,
    D16 = 30,
    D17 = 31,
    D18 = 32,
    D19 = 33,
    D20 = 34,
    D21 = 35,
    D22 = 36,
    D23 = 37,
    D24 = 38,
    D25 = 39,
    D26 = 40,
    D27 = 41,
    D28 = 42,
    D29 = 43,
    D30 = 44,
    D31 = 45,
    FPINST2 = 46,
    MVFR0 = 47,
    MVFR1 = 48,
    MVFR2 = 49,
    Q0 = 50,
    Q1 = 51,
    Q2 = 52,
    Q3 = 53,
    Q4 = 54,
    Q5 = 55,
    Q6 = 56,
    Q7 = 57,
    Q8 = 58,
    Q9 = 59,
    Q10 = 60,
    Q11 = 61,
    Q12 = 62,
    Q13 = 63,
    Q14 = 64,
    Q15 = 65,
    R0 = 66,
    R1 = 67,
    R2 = 68,
    R3 = 69,
    R4 = 70,
    R5 = 71,
    R6 = 72,
    R7 = 73,
    R8 = 74,
    R9 = 75,
    R10 = 76,
    R11 = 77,
    R12 = 78,
    S0 = 79,
    S1 = 80,
    S2 = 81,
    S3 = 82,
    S4 = 83,
    S5 = 84,
    S6 = 85,
    S7 = 86,
    S8 = 87,
    S9 = 88,
    S10 = 89,
    S11 = 90,
    S12 = 91,
    S13 = 92,
    S14 = 93,
    S15 = 94,
    S16 = 95,
    S17 = 96,
    S18 = 97,
    S19 = 98,
    S20 = 99,
    S21 = 100,
    S22 = 101,
    S23 = 102,
    S24 = 103,
    S25 = 104,
    S26 = 105,
    S27 = 106,
    S28 = 107,
    S29 = 108,
    S30 = 109,
    S31 = 110,
    C1_C0_2 = 111,
    C13_C0_2 = 112,
    C13_C0_3 = 113,
    IPSR = 114,
    MSP = 115,
    PSP = 116,
    CONTROL = 117,
    IAPSR = 118,
    EAPSR = 119,
    XPSR = 120,
    EPSR = 121,
    IEPSR = 122,
    PRIMASK = 123,
    BASEPRI = 124,
    BASEPRI_MAX = 125,
    FAULTMASK = 126,
    APSR_NZCVQ = 127,
    APSR_G = 128,
    APSR_NZCVQG = 129,
    IAPSR_NZCVQ = 130,
    IAPSR_G = 131,
    IAPSR_NZCVQG = 132,
    EAPSR_NZCVQ = 133,
    EAPSR_G = 134,
    EAPSR_NZCVQG = 135,
    XPSR_NZCVQ = 136,
    XPSR_G = 137,
    XPSR_NZCVQG = 138,
    CP_REG = 139,
    ENDING = 140,
}

impl RegisterARM {
    // alias registers
    // (assoc) R13 = 12,
    // (assoc) R14 = 10,
    // (assoc) R15 = 11,
    // (assoc) SB = 75,
    // (assoc) SL = 76,
    // (assoc) FP = 77,
    // (assoc) IP = 78,
    pub const R13: RegisterARM = RegisterARM::SP;
    pub const R14: RegisterARM = RegisterARM::LR;
    pub const R15: RegisterARM = RegisterARM::PC;
    pub const SB: RegisterARM = RegisterARM::R9;
    pub const SL: RegisterARM = RegisterARM::R10;
    pub const FP: RegisterARM = RegisterARM::R11;
    pub const IP: RegisterARM = RegisterARM::R12;
}

impl From<RegisterARM> for i32 {
    fn from(r: RegisterARM) -> Self {
        r as i32
    }
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ArmCpuModel {
    UC_CPU_ARM_926 = 0,
    UC_CPU_ARM_946 = 1,
    UC_CPU_ARM_1026 = 2,
    UC_CPU_ARM_1136_R2 = 3,
    UC_CPU_ARM_1136 = 4,
    UC_CPU_ARM_1176 = 5,
    UC_CPU_ARM_11MPCORE = 6,
    UC_CPU_ARM_CORTEX_M0 = 7,
    UC_CPU_ARM_CORTEX_M3 = 8,
    UC_CPU_ARM_CORTEX_M4 = 9,
    UC_CPU_ARM_CORTEX_M7 = 10,
    UC_CPU_ARM_CORTEX_M33 = 11,
    UC_CPU_ARM_CORTEX_R5 = 12,
    UC_CPU_ARM_CORTEX_R5F = 13,
    UC_CPU_ARM_CORTEX_A7 = 14,
    UC_CPU_ARM_CORTEX_A8 = 15,
    UC_CPU_ARM_CORTEX_A9 = 16,
    UC_CPU_ARM_CORTEX_A15 = 17,
    UC_CPU_ARM_TI925T = 18,
    UC_CPU_ARM_SA1100 = 19,
    UC_CPU_ARM_SA1110 = 20,
    UC_CPU_ARM_PXA250 = 21,
    UC_CPU_ARM_PXA255 = 22,
    UC_CPU_ARM_PXA260 = 23,
    UC_CPU_ARM_PXA261 = 24,
    UC_CPU_ARM_PXA262 = 25,
    UC_CPU_ARM_PXA270 = 26,
    UC_CPU_ARM_PXA270A0 = 27,
    UC_CPU_ARM_PXA270A1 = 28,
    UC_CPU_ARM_PXA270B0 = 29,
    UC_CPU_ARM_PXA270B1 = 30,
    UC_CPU_ARM_PXA270C0 = 31,
    UC_CPU_ARM_PXA270C5 = 32,
}

impl From<ArmCpuModel> for i32 {
    fn from(value: ArmCpuModel) -> Self {
        value as i32
    }
}

impl From<&ArmCpuModel> for i32 {
    fn from(value: &ArmCpuModel) -> Self {
        *value as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
#[repr(C)]
pub struct ArmCpRegister {
  pub crn: u32,
  pub crm: u32,
  pub op0: u32,
  pub op1: u32,
  pub op2: u32,
}

impl ArmCpRegister {
    pub const fn from_manual(
        op0: u32,
        op1: u32,
        crn: u32,
        crm: u32,
        op2: u32,
    ) -> Self {
        Self { crn, crm, op0, op1, op2 }
    }
}

#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
#[repr(C)]
pub(crate) struct ArmCpRegisterInput {
  pub reg: ArmCpRegister,
  pub _pad: u32,
  // value to be written or read
  pub value: u64,
}
