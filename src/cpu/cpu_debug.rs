#![rustfmt::skip]
pub const INSTRUCTION_SIZES: [u16; 256] = [
    1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 3, 3, 3, 3, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 3, 3, 3, 3, 3,
    1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 3, 3, 3, 3, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 3, 3, 3, 3, 3,
    1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 1, 3, 3, 3, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 3, 3, 3, 3, 3,
    1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 1, 3, 3, 3, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2, 3, 3, 3, 3, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2, 3, 3, 3, 3, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2, 3, 3, 3, 3, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2, 3, 3, 3, 3, 2, 2, 1, 2, 2, 2, 2, 2, 1, 3, 1, 3, 3, 3, 3, 3,
];

pub const INSTRUCTION_NAMES: [&'static str; 256] = [
    //      00          01              02          03     04           05            06              07            08          09              0A          0B          0C              0D              0E              0F
    /*00*/ "BRK_IMPL",  "ORA_X_IND",    "NOP",      "NOP", "NOP",       "ORA_ZPG",    "ASL_ZPG",      "NOP",        "PHP_IMPL", "ORA_IMM",      "ASL_A",    "NOP",      "NOP",          "ORA_ABS",      "ASL_ABS",      "NOP",
    /*10*/ "BPL_REL",   "ORA_Y_IND",    "NOP",      "NOP", "NOP",       "ORA_X_ZPG",  "ASL_X_ZPG",    "NOP",        "CLC_IMPL", "ORA_Y_ABS",    "NOP",      "NOP",      "NOP",          "ORA_X_ABS",    "ASL_X_ABS",    "NOP", 
    /*20*/ "JSR_ABS",   "AND_X_IND",    "NOP",      "NOP", "BIT_ZPG",   "AND_ZPG",    "ROL_ZPG",      "NOP",        "PLP_IMPL", "AND_IMM",      "ROL_A",    "NOP",      "BIT_ABS",      "AND_ABS",      "ROL_ABS",      "NOP",
    /*30*/ "BMI_REL",   "AND_Y_IND",    "NOP",      "NOP", "NOP",       "AND_X_ZPG",  "ROL_X_ZPG",    "NOP",        "SEC_IMPL", "AND_Y_ABS",    "NOP",      "NOP",      "NOP",          "AND_X_ABS",    "ROL_X_ABS",    "NOP",
    /*40*/ "RTI_IMPL",  "EOR_X_IND",    "NOP",      "NOP", "NOP",       "EOR_ZPG",    "LSR_ZPG",      "NOP",        "PHA_IMPL", "EOR_IMM",      "LSR_A",    "NOP",      "JMP_ABS",      "EOR_ABS",      "LSR_ABS",      "NOP",
    /*50*/ "BVC_REL",   "EOR_Y_IND",    "NOP",      "NOP", "NOP",       "EOR_X_ZPG",  "LSR_X_ZPG",    "NOP",        "CLI_IMPL", "EOR_Y_ABS",    "NOP",      "NOP",      "NOP",          "EOR_X_ABS",    "LSR_X_ABS",    "NOP",
    /*60*/ "RTS_IMPL",  "ADC_X_IND",    "NOP",      "NOP", "NOP",       "ADC_ZPG",    "ROR_ZPG",      "NOP",        "PLA_IMPL", "ADC_IMM",      "ROR_A",    "NOP",      "JMP_IND",      "ADC_ABS",      "ROR_ABS",      "NOP",
    /*70*/ "BVS_REL",   "ADC_Y_IND",    "NOP",      "NOP", "NOP",       "ADC_X_ZPG",  "ROR_X_ZPG",    "NOP",        "SEI_IMPL", "ADC_Y_ABS",    "NOP",      "NOP",      "NOP",          "ADC_X_ABS",    "ROR_X_ABS",    "NOP",
    /*80*/ "NOP",       "STA_X_IND",    "NOP",      "NOP", "STY_ZPG",   "STA_ZPG",    "STX_ZPG",      "NOP",        "DEY_IMPL", "NOP",          "TXA_IMPL", "NOP",      "STY_ABS",      "STA_ABS",      "STX_ABS",      "NOP",
    /*90*/ "BCC_REL",   "STA_Y_IND",    "NOP",      "NOP", "STY_X_ZPG", "STA_X_ZPG",  "STX_Y_ZPG",    "NOP",        "TYA_IMPL", "STA_Y_ABS",    "TXS_IMPL", "NOP",      "NOP",          "STA_X_ABS",    "NOP",          "NOP",
    /*A0*/ "LDY_IMM",   "LDA_X_IND",    "LDX_IMM",  "NOP", "LDY_ZPG",   "LDA_ZPG",    "LDX_ZPG",      "NOP",        "TAY_IMPL", "LDA_IMM",      "TAX_IMPL", "NOP",      "LDY_ABS",      "LDA_ABS",      "LDX_ABS",      "NOP",
    /*B0*/ "BCS_REL",   "LDA_Y_IND",    "NOP",      "NOP", "LDY_X_ZPG", "LDA_X_ZPG",  "LDX_Y_ZPG",    "NOP",        "CLV_IMPL", "LDA_Y_ABS",    "TSX_IMPL", "NOP",      "LDY_X_ABS",    "LDA_X_ABS",    "LDX_Y_ABS",    "NOP",
    /*C0*/ "CPY_IMM",   "CMP_X_IND",    "NOP",      "NOP", "CPY_ZPG",   "CMP_ZPG",    "DEC_ZPG",      "NOP",        "INY_IMPL", "CMP_IMM",      "DEX_IMPL", "NOP",      "CPY_ABS",      "CMP_ABS",      "DEC_ABS",      "NOP",
    /*D0*/ "BND_REL",   "CMP_Y_IND",    "NOP",      "NOP", "NOP",       "CMP_X_ZPG",  "DEC_X_ZPG",    "NOP",        "CLD_IMPL", "CMP_Y_ABS",    "NOP",      "NOP",      "NOP",          "CMP_X_ABS",    "DEC_X_ABS",    "NOP",
    /*E0*/ "CPX_IMM",   "SBC_X_IND",    "NOP",      "NOP", "CPX_ZPG",   "SBC_ZPG",    "INC_ZPG",      "NOP",        "INX_IMPL", "SBC_IMM",      "NOP_IMPL", "NOP",      "CPX_ABS",      "SBC_ABS",      "INC_ABS",      "NOP",
    /*F0*/ "BEQ_REL",   "SBC_Y_IND",    "NOP",      "NOP", "NOP",       "SBC_X_ZPG",  "INC_X_ZPG",    "NOP",        "SED_IMPL", "SBC_Y_ABS",    "NOP",      "NOP",      "NOP",          "SBC_X_ABS",    "INC_X_ABS",    "NOP",

];
