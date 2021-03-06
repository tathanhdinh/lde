/*!
References:

* http://sparksandflames.com/files/x86InstructionChart.html
* http://www.c-jump.com/CIS77/CPU/x86/X77_0060_mod_reg_r_m_byte.htm
* http://ref.x86asm.net/geek32.html
* https://github.com/greenbender/lend

May contain errors...
*/

use contains::Contains;
use InstLen;

static TABLE_PREFIX: [u32; 8] = [
    /* 0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F */
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 0
    0b_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0, // 2
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 4
    0b_0_0_0_0_1_1_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 6
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1_0_0_0_0, // 8
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // A
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // C
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1_0_1_1_0_0_0_0_0_0_0_0_0_0_0_0, // E
];
//---- One-byte opcodes ----
static TABLE_MODRM_A: [u32; 8] = [
    /* 0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F */
    0b_1_1_1_1_0_0_0_0_1_1_1_1_0_0_0_0_1_1_1_1_0_0_0_0_1_1_1_1_0_0_0_0, // 0
    0b_1_1_1_1_0_0_0_0_1_1_1_1_0_0_0_0_1_1_1_1_0_0_0_0_1_1_1_1_0_0_0_0, // 2
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 4
    0b_0_0_1_1_0_0_0_0_0_1_0_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 6
    0b_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 8
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // A
    0b_1_1_0_0_1_1_1_1_0_0_0_0_0_0_0_0_1_1_1_1_0_0_0_0_1_1_1_1_1_1_1_1, // C
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1_1_0_0_0_0_0_0_1_1, // E
];
static TABLE_IMM8_A: [u32; 8] = [
    /* 0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F */
    0b_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0, // 0
    0b_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0, // 2
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 4
    0b_0_0_0_0_0_0_0_0_0_0_1_1_0_0_0_0_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1, // 6
    0b_1_0_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 8
    0b_0_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_1_1_1_1_1_1_1_0_0_0_0_0_0_0_0, // A
    0b_1_1_0_0_0_0_1_0_1_0_0_0_0_1_0_0_0_0_0_0_1_1_0_0_0_0_0_0_0_0_0_0, // C
    0b_1_1_1_1_1_1_1_1_0_0_0_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // E
];
static TABLE_IMM_A: [u32; 8] = [
    /* 0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F */
    0b_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0, // 0
    0b_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_1_0_0, // 2
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 4
    0b_0_0_0_0_0_0_0_0_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 6
    0b_0_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1_0_0_0_0_0, // 8
    0b_0_0_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1_1_1_1_1_1_1_1, // A
    0b_0_0_0_0_0_0_0_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // C
    0b_0_0_0_0_0_0_0_0_1_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // E
];
//---- Two-byte opcodes ----
static TABLE_MODRM_B: [u32; 8] = [
    /* 0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F */
    0b_1_1_1_1_0_0_0_0_0_0_0_0_0_1_0_0_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1, // 0
    0b_0_0_0_0_0_0_0_0_1_1_1_1_1_1_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 2
    0b_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1, // 4
    0b_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_0_0_0_1_1_1_0_1_1_1_1_1_1_1_1, // 6
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1, // 8
    0b_0_0_0_1_1_1_0_0_0_0_0_1_1_1_1_1_1_1_1_1_1_1_1_1_1_0_1_1_1_1_1_1, // A
    0b_1_1_1_1_1_1_1_1_0_0_0_0_0_0_0_0_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1, // C
    0b_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1_1, // E
];
static TABLE_INVALID_B: [u32; 8] = [
    /* 0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F */
    0b_0_0_0_0_1_0_0_0_0_0_1_0_1_0_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 0
    0b_0_0_0_0_0_1_0_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1_0_1_1_1_1_1_1_1_1, // 2
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 4
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1_1_0_0_0_0, // 6
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // 8
    0b_0_0_0_0_0_0_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // A
    0b_0_0_0_0_0_0_0_0_0_1_1_1_1_1_1_1_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0, // C
    0b_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0_1, // E
];
//---- Three-byte opcodes 38 ----
static TABLE_INVALID_C: [u32; 2] = [
    /* 0 1 2 3 4 5 6 7 8 9 A B C D E F 0 1 2 3 4 5 6 7 8 9 A B C D E F */
    0b_0_0_0_0_0_0_0_0_0_0_0_0_1_1_1_1_0_1_1_1_0_0_1_0_1_1_1_1_0_0_0_1, // 0
    0b_0_0_0_0_0_0_1_1_0_0_0_0_1_1_1_1_0_0_0_0_0_0_1_0_0_0_0_0_0_0_0_0, // 2
];
//---- Three-byte opcodes 3A ----

pub fn inst_len(opcode: &[u8]) -> InstLen {
    let modrm;
    let mut op: u8;
    let (mut ddef, mut mdef) = (4u32, 4u32);
    let (mut dsize, mut msize) = (0u32, 0u32);
    let mut it = opcode.iter();

    // Prefixes
    let mut prefix_len = 0;
    loop {
        op = match it.next() {
            Some(&op) => op,
            None => return InstLen::EMPTY,
        };
        if TABLE_PREFIX.has(op) {
            prefix_len += 1;
            // Operand-size override prefix
            if op == 0x66 {
                ddef = 2u32;
            }
            // Address-size override prefix
            else if op == 0x67 {
                mdef = 2u32;
            }
        } else {
            break;
        }
    }

    let mut op_len = 1;
    if op == 0x0F {
        op = match it.next() {
            Some(&op) => op,
            None => return InstLen::EMPTY,
        };
        op_len += 1;
        // Three-byte opcodes (C)
        if op == 0x38 {
            op = match it.next() {
                Some(&op) => op,
                None => return InstLen::EMPTY,
            };
            op_len += 1;
            // Invalid opcodes
            if if op < 0x40 {
                TABLE_INVALID_C.has(op)
            } else {
                !((0x40..0x42).has(op) || (0x80..0x82).has(op) || (0xF0..0xF2).has(op))
            } {
                return InstLen::EMPTY;
            };
            modrm = true;
        }
        // Three-byte opcodes (D)
        else if op == 0x3A {
            op = match it.next() {
                Some(&op) => op,
                None => return InstLen::EMPTY,
            };
            op_len += 1;
            // Invalid opcodes
            if !((0x08..0x10).has(op)
                || (0x14..0x18).has(op)
                || (0x20..0x23).has(op)
                || (0x40..0x43).has(op)
                || (0x60..0x64).has(op))
            {
                return InstLen::EMPTY;
            };
            modrm = true;
            dsize += 1;
        }
        // Two-byte opcodes (B)
        else {
            // Invalid opcodes
            if TABLE_INVALID_B.has(op) {
                return InstLen::EMPTY;
            }
            modrm = TABLE_MODRM_B.has(op);
            // Check for imm8
            if (0x70..0x74).has(op)
                || op == 0xA4
                || op == 0xAC
                || op == 0xBA
                || op == 0xC2
                || (0xC4..0xC7).has(op)
            {
                dsize += 1;
            }
            // Check for imm16
            if (op & 0xF0) == 0x80 {
                dsize += ddef;
            }
        }
    }
    // One-byte opcodes (A)
    else {
        modrm = TABLE_MODRM_A.has(op);
        // Check `test` opcode with immediate
        if (op == 0xF6 || op == 0xF7)
            && (if let Some(&op) = it.clone().next() {
                op
            } else {
                return InstLen::EMPTY;
            } & 0x38)
                == 0
        {
            dsize += if (op & 1) != 0 { ddef } else { 1 }
        }
        // Check for imm8
        if TABLE_IMM8_A.has(op) {
            dsize += 1;
        }
        // Check for imm16: CALLF Ap, RETN Iw, ENTER eBP Iw Ib, RETF Iw, JMPF Ap
        if op == 0x9A || op == 0xC2 || op == 0xC8 || op == 0xCA || op == 0xEA {
            dsize += 2;
        }
        // Check for immediate
        if TABLE_IMM_A.has(op) {
            dsize += ddef;
        }
        // Special snowflake `movabs`
        if (op & 0xFC) == 0xA0 {
            msize += mdef;
        }
    }

    // Mod R/M
    if modrm {
        op = match it.next() {
            Some(&op) => op,
            None => return InstLen::EMPTY,
        };
        let mode = op & 0xC0;
        let rm = op & 0b111;
        if mode != 0xC0 {
            if rm == 0b100 {
                // Scaled Index Byte
                op = match it.next() {
                    Some(&op) => op,
                    None => return InstLen::EMPTY,
                };
                if mode == 0x00 {
                    if (op & 0b111) == 0b101 {
                        msize += 4;
                    }
                }
            }
            if mode == 0x00 {
                if rm == 0b101 {
                    msize += 4;
                }
            } else if mode == 0x40 {
                msize += 1;
            } else if mode == 0x80 {
                msize += mdef;
            }
        }
    }

    // Get total length and bounds check
    let total_len =
        ((it.as_slice().as_ptr() as usize).wrapping_sub(opcode.as_ptr() as usize)) as u32;
    let total_len = total_len.wrapping_add(dsize + msize) as u8;

    let arg_len = total_len - prefix_len - op_len;
    if total_len as usize <= opcode.len() {
        InstLen {
            total_len,
            op_len,
            arg_len,
            prefix_len,
        }
    } else {
        InstLen::EMPTY
    }
}

//----------------------------------------------------------------

#[cfg(test)]
fn lde_int(bytes: &[u8]) -> u32 {
    inst_len(bytes).total_len as u32
}

#[test]
fn units() {
    // add al, *
    assert_eq!(lde_int(b"\x04*"), 2);
    // mov DWORD PTR [ebp+*]
    assert_eq!(lde_int(b"\x89\x5D*"), 3);
    // test al, al
    assert_eq!(lde_int(b"\x84\xC0"), 2);
    // fld QWORD PTR [eax+eax*4+****]
    assert_eq!(lde_int(b"\xDD\x84\x00****"), 7);
    // mov esi, ****
    assert_eq!(lde_int(b"\xBE****"), 5);
    // mov eax, fs:****
    assert_eq!(lde_int(b"\x64\xA1****"), 6);
    // add DWORD PTR ds:****, eax
    assert_eq!(lde_int(b"\x01\x05****"), 6);
    // addr16 mov eax, dx:**
    assert_eq!(lde_int(b"\x67\xA1**"), 4);
    // add BYTE PTR [bx+si+**], al
    assert_eq!(lde_int(b"\x67\x00\x80**"), 5);
    // inc eax
    assert_eq!(lde_int(b"\x40"), 1);
    // retn
    assert_eq!(lde_int(b"\xC3"), 1);
    // nop dword ptr [rax+*]
    assert_eq!(lde_int(b"\x0F\x1F\x40\x00"), 4);
    // nop dword ptr [rax+****]
    assert_eq!(lde_int(b"\x66\x0F\x0D\x80****"), 8);
    // clflush byte ptr [rax]
    assert_eq!(lde_int(b"\x0F\xAE\x38"), 3);
}
