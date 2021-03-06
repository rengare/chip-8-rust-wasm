pub fn disassemble(opcode: u16) -> String {
    match opcode {
        0x0000..=0x0FFF => {
            let subcode = opcode & 0x00FF;
            match subcode {
                0x00E0 => {
                    // 00E0 - CLS
                    //Clear the display.
                    String::from("CLS")
                }
                0x00EE => {
                    // 00EE - RET
                    // Return from a subroutine.
                    String::from("RET")
                }
                _ => {
                    // Unused memory
                    String::from("")
                    // 0nnn - SYS addr
                    // Jump to a machine code routine at nnn.
                    // This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
                }
            }
        }
        0x1000..=0x1FFF => {
            // 1nnn - JP addr
            // Jump to location nnn.
            let address = opcode & 0x0FFF;
            format!("JP 0x{:X}", address)
        }
        0x2000..=0x2FFF => {
            // 2nnn - CALL addr
            // Call subroutine at nnn.
            let address = opcode & 0x0FFF;
            format!("CALL 0x{:X}", address)
        }

        0x3000..=0x3FFF => {
            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx = kk.
            let x = (opcode & 0x0F00) >> 8;
            let kk = opcode & 0x00FF;
            format!("SE V{} {}", x, kk)
        }
        0x4000..=0x4FFF => {
            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk.

            //The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
            let x = (opcode & 0x0F00) >> 8;
            let kk = opcode & 0x00FF;
            format!("SNE V{} {}", x, kk)
        }
        0x5000..=0x5FFF => {
            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy.
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            format!("SE V{} V{}", x, y)
        }
        0x6000..=0x6FFF => {
            // 6xkk - LD Vx, byte
            // The interpreter puts the value kk into register Vx.
            let x = (opcode & 0x0F00) >> 8;
            let kk = opcode & 0x00FF;
            format!("LD V{} {}", x, kk)
        }
        0x7000..=0x7FFF => {
            // 7xkk - ADD Vx, byte
            // Set Vx = Vx + kk.
            let x = (opcode & 0x0F00) >> 8;
            format!("ADD V{} {}", x, opcode & 0x00FF)
        }
        0x8000..=0x8FFF => {
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let subcode = opcode & 0x000F;
            match subcode {
                0 => {
                    // 8xy0 - LD Vx, Vy
                    // Set Vx = Vy.
                    format!("LD V{} V{}", x, y)
                }
                1 => {
                    // 8xy1 - OR Vx, Vy
                    // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
                    format!("OR V{} V{}", x, y)
                }
                2 => {
                    // 8xy2 - AND Vx, Vy
                    // Set Vx = Vx AND Vy.
                    // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
                    format!("AND V{} V{}", x, y)

                }
                3 => {
                    // 8xy3 - XOR Vx, Vy
                    // Set Vx = Vx XOR Vy.
                    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
                    format!("XOR V{} V{}", x, y)
                }
                4 => {
                    // 8xy4 - ADD Vx, Vy
                    // Set Vx = Vx + Vy, set VF = carry.
                    // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
                    format!("ADD V{} V{}", x, y)
                }
                5 => {
                    // 8xy5 - SUB Vx, Vy
                    // Set Vx = Vx - Vy, set VF = NOT borrow.
                    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                    format!("SUB V{} V{}", x, y)
                }
                6 => {
                    // 8xy6 - SHR Vx {, Vy}
                    // Set Vx = Vx SHR 1.
                    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                    format!("SHR V{} {{, V{}}}", x, y)
                }
                7 => {
                    // 8xy7 - SUBN Vx, Vy
                    // Set Vx = Vy - Vx, set VF = NOT borrow.
                    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                    format!("SUBN V{} V{} ", x, y)
                }
                0xE => {
                    // 8xyE - SHL Vx {, Vy}
                    // Set Vx = Vx SHL 1.
                    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                    format!("HL V{} V{} ", x, y)
                }
                _ => {
                    format!("??? {:X}", opcode)
                }
            }
        }
        0x9000..=0x9FFF => {
            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy.
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            format!("SNE V{} V{} ", x, y)
        },
        0xA000..=0xAFFF => {
            // Annn - LD I, addr
            // The value of register I is set to nnn.
            format!("LD I {} ", opcode & 0x0FFF)
        }
        0xB000..=0xBFFF => {
            // Bnnn - JP V0, addr
            // Jump to location nnn + V0.
            // The program counter is set to nnn plus the value of V0.
            let address = opcode & 0x0FFF;
            format!("JP V0, {} ", address)
        }
        0xC000..=0xCFFF => {
            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk.
            // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
            let x = (opcode & 0x0F00) >> 8;
            let kk = opcode & 0x00FF;
            format!("RND V{}, {} ", x, kk)
        }

        0xD000..=0xDFFF => {
            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let nibble = opcode & 0x000F;
            format!("DRW V{} V{} {} ", x, y, nibble)
        }
        0xE000 ..= 0xEFFF => {
            let x = (opcode & 0x0F00) >> 8;
            let code = opcode & 0x00FF;
            match code {
                0x9E => {
                    // Ex9E - SKP Vx
                    // Skip next instruction if key with the value of Vx is pressed.
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                    let x = (opcode & 0x0F00) >> 8;
                    format!("SKP V{} ", x)
                }
                0xA1 => {
                    // ExA1 - SKNP Vx
                    // Skip next instruction if key with the value of Vx is not pressed.
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position;
                    format!("SKNP V{}", x)
                }
                _ => {
                    format!("??? {:X}", opcode)
                }
            }
        }
        0xF000 ..= 0xFFFF => {
            let x = (opcode & 0x0F00) >> 8;
            let code = opcode & 0x00FF;
            match code {
                0x07 => {
                    // Fx07 - LD Vx, DT
                    // Set Vx = delay timer value.
                    format!("LD V{}, DT", x)
                }
                0x0A => {
                    // Fx0A - LD Vx, K
                    // Wait for a key press, store the value of the key in Vx.
                    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                    format!("LD V{} K", x)
                }
                0x15 => {
                    // Fx15 - LD DT, Vx
                    // Set delay timer = Vx.
                    format!("LD DT V{}", x)
                }
                0x18 => {
                    // Fx18 - LD ST, Vx
                    // Set sound timer = Vx.
                    format!("LD ST V{}", x)
                }
                0x1E => {
                    // Fx1E - ADD I, Vx
                    // Set I = I + Vx.
                    format!("ADD I V{}", x)
                }
                0x29 => {
                    // Fx29 - LD F, Vx
                    // Set I = location of sprite for digit Vx.
                    format!("LD F V{}", x)
                }
                0x33 => {
                    // Fx33 - LD B, Vx
                    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    format!("LD B V{}", x)
                }
                0x55 => {
                    // Fx55 - LD [I], Vx
                    // Store registers V0 through Vx in memory starting at location I.
                    format!("LD [I] V{}", x)
                }
                0x65 => {
                    // Fx65 - LD Vx, [I]
                    // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                    format!("LD V{}, [I]", x)
                }
                _ => {
                    format!("??? {:X}", opcode)
                }
            }
        }
    }
}
