# read ops.txt
with open('ops.txt') as f:
    opcode = 0x00
    ins = f.readline()
    while ins:
        # skip undefined opcodes
        if opcode in [0xd3, 0xe3, 0xe4, 0xf4, 0xdb, 0xeb, 0xec, 0xfc, 0xdd, 0xed, 0xfd]:
            print('0x{:02x} => {{ self.undefined_op(opcode); 1 }},'.format(opcode))
            opcode += 1
            continue

        # decode op
        ins_parts = ins.split()
        op = ins_parts[0].lower()
        cycles = f.readline().split()[1]
        if cycles.isnumeric():
            cycles = int(cycles) // 4
        else:
            cycles = str(int(cycles.split('/')[0]) // 4) + '/' + str(int(cycles.split('/')[1]) // 4)

        output = '0x{:02x} => {{ '.format(opcode)
        additional = ''

        if op == 'ld' or op == 'ldh':
            is_16bit = opcode in [0x01, 0x11, 0x21, 0x31, 0x08, 0xf8, 0xf9]
            if not is_16bit:
                output += 'self.ld_byte('
                
                operands = ins_parts[1].split(',')
                dest = operands[0]
                src = operands[1]
                mem_dest = '(' in dest
                mem_src = '(' in src

                if mem_src:
                    src_inner = src[1:-1]
                    if src_inner.isalpha() and len(src_inner) == 2:
                        output += 'self.mmu.read_byte(self.regs.{}()),'.format(src_inner.lower())
                    elif src_inner.isalnum():
                        if '16' in src_inner:
                            output += 'self.mmu.read_byte(self.fetch_ins_word()),'
                        elif '8' in src_inner:
                            output += 'self.mmu.read_byte(self.fetch_ins_byte() as u16 + 0xff00),'
                        elif 'C' in src_inner:
                            output += 'self.mmu.read_byte(self.regs.c() as u16 + 0xff00),'
                    elif '+' in src_inner or '-' in src_inner:
                        output += 'self.mmu.read_byte(self.regs.hl()),'
                        additional += (' self.regs.set_hl(self.regs.hl().wrapping_add(1));' if '+' in src_inner 
                                        else ' self.regs.set_hl(self.regs.hl().wrapping_sub(1));')
                    else:
                        print('da hell')
                elif src.isupper():
                    output += 'self.regs.{}(),'.format(src.lower())
                elif 'd8' in src:
                    output += 'self.fetch_ins_byte(),'
                else:
                    print('vv handle me! vv')

                if mem_dest:
                    dest_inner = dest[1:-1]
                    if dest_inner.isalpha() and len(dest_inner) == 2:
                        output += ' None, self.regs.{}());'.format(dest_inner.lower())
                    elif dest_inner.isalnum():
                        if '16' in dest_inner:
                            output += ' None, self.fetch_ins_word());'
                        elif '8' in dest_inner:
                            output += ' None, self.fetch_ins_byte() as u16 + 0xff00);'
                        elif 'C' in dest_inner:
                            output += 'None, self.regs.c() as u16 + 0xff00);'
                    elif '+' in dest_inner or '-' in dest_inner:
                        output += ' None, self.regs.hl());'
                        additional += (' self.regs.set_hl(self.regs.hl().wrapping_add(1));' if '+' in dest_inner 
                                        else ' self.regs.set_hl(self.regs.hl().wrapping_sub(1));')
                    else:
                        print('da hell')
                elif dest.isupper():
                    output += ' Some(RegIndex::{}), 0);'.format(dest)
                else:
                    print('vv handle me! vv')
            else:
                output += 'self.ld_word('

                operands = ins_parts[1].split(',')
                dest = operands[0]
                src = operands[1]
                mem_dest = '(' in dest

                if '16' in src:
                    output += 'self.fetch_ins_word(), '
                elif '+' in src:
                    # let old_sp = self.regs.sp(); self.add_word(self.fetch_ins_byte() as i8 as i16 as u16, RegIndex::SP); self.ld_word(self.regs.sp(), RegIndex::HL, 0); self.regs.set_sp(old_sp); self.regs.set_zflag(false);
                    output += 'self.regs.sp().wrapping_add(self.fetch_ins_byte() as i8 as i16 as u16), '  ## NEED FLAG RESETS/CHECKS
                    additional += ' self.regs.set_zflag(false); FIX THIS'
                else:
                    output += 'self.regs.{}(), '.format(src.lower()) 

                if mem_dest:
                    output += 'None, self.fetch_ins_word());'
                else: 
                    output += 'Some(RegIndex::{}), 0);'.format(dest)
            
        elif op == 'jr':
            output += 'self.jr('
            operands = ins_parts[1].split(',')
            if len(operands) > 1:
                output += 'Some(Condition::{})) as u8 +'.format(operands[0])
            else:
                output += 'None);'
        elif op == 'inc':
            output += 'self.inc('
            if not '(' in ins_parts[1]:
                output += 'Some(RegIndex::{}), 0);'.format(ins_parts[1])
            else:
                output += 'None, self.regs.hl());'
        elif op == 'dec':
            output += 'self.dec('
            if not '(' in ins_parts[1]:
                output += 'Some(RegIndex::{}), 0);'.format(ins_parts[1])
            else:
                output += 'None, self.regs.hl());'
        elif op == 'add' or op == 'adc':
            carry = op == 'adc'
            is_16bit = opcode in [0x09, 0x19, 0x29, 0x39, 0xe8]

            if not is_16bit:
                output += 'self.add_byte('
                src = ins_parts[1].split(',')[1]

                if '(' in src:
                    output += 'self.mmu.read_byte(self.regs.hl()),'
                elif 'd8' in src:
                    output += 'self.fetch_ins_byte(),'
                else:
                    output += 'self.regs.{}(),'.format(src.lower())

                output += ' {});'.format(str(carry).lower())
            else:
                output += 'self.add_word('
                if not 'r8' in ins_parts[1]:
                    src = ins_parts[1].split(',')[1].lower()
                    output += 'self.regs.{}(), RegIndex::HL);'.format(src)
                else:
                    output += 'self.fetch_ins_byte() as i8 as i16 as u16, RegIndex::SP);'
        elif op == 'sub' or op == 'sbc':
            carry = op == 'sbc'
            output += 'self.sub_byte('
            src = ins_parts[1] if not ',' in ins_parts[1] else ins_parts[1].split(',')[1]

            if '(' in src:
                output += 'self.mmu.read_byte(self.regs.hl()),'
            elif 'd8' in src:
                output += 'self.fetch_ins_byte(),'
            else:
                output += 'self.regs.{}(),'.format(src.lower())

            output += ' {});'.format(str(carry).lower())
        elif op == 'and':
            output += 'self.and('
            if not '(' in ins_parts[1]:
                if not 'd8' in ins_parts[1]:
                    reg = ins_parts[1].lower()
                    output += 'self.regs.{}());'.format(reg)
                else:
                    output += 'self.fetch_ins_byte());'
            else:
                output += 'self.mmu.read_byte(self.regs.hl()));'
        elif op == 'xor':
            output += 'self.xor('
            if not '(' in ins_parts[1]:
                if not 'd8' in ins_parts[1]:
                    reg = ins_parts[1].lower()
                    output += 'self.regs.{}());'.format(reg)
                else:
                    output += 'self.fetch_ins_byte());'
            else:
                output += 'self.mmu.read_byte(self.regs.hl()));'
        elif op == 'or':
            output += 'self.or('
            if not '(' in ins_parts[1]:
                if not 'd8' in ins_parts[1]:
                    reg = ins_parts[1].lower()
                    output += 'self.regs.{}());'.format(reg)
                else:
                    output += 'self.fetch_ins_byte());'
            else:
                output += 'self.mmu.read_byte(self.regs.hl()));'
        elif op == 'cp':
            output += 'self.cp('
            if not '(' in ins_parts[1]:
                if not 'd8' in ins_parts[1]:
                    reg = ins_parts[1].lower()
                    output += 'self.regs.{}());'.format(reg)
                else:
                    output += 'self.fetch_ins_byte());'
            else:
                output += 'self.mmu.read_byte(self.regs.hl()));'
        elif op == 'ret' or op == 'reti':
            output += 'self.ret('
            if len(ins_parts) == 1:
                output += 'None);'
            else:
                output += 'Some(Condition::{})) as u8 +'.format(ins_parts[1])
        elif op == 'call':
            output += 'self.call('
            if not ',' in ins_parts[1]:
                output += 'None);'
            else:
                output += 'Some(Condition::{})) as u8 +'.format(ins_parts[1].split(',')[0])
        elif op == 'pop':
            output += 'self.pop(RegIndex::{});'.format(ins_parts[1])
        elif op == 'push':
            output += 'self.push(self.regs.{}());'.format(ins_parts[1].lower())
        elif op == 'rst':
            addr = '0x' + ins_parts[1].split('H')[0]
            output += 'self.rst({} as u16);'.format(addr)
        elif op == 'jp':
            output += 'self.jump('
            if '16' in ins_parts[1]:
                output += 'self.fetch_ins_word(),'
            else:
                output += 'self.regs.hl(),'
            
            if ',' in ins_parts[1]:
                cond = ins_parts[1].split(',')[0]
                output += ' Condition::{}) as u8 +'.format(cond)
            else:
                output += ' None);'
        elif opcode == 0xcb:
            output += 'self.decode_cb();'
        else:
            output += 'self.{}();'.format(op)

        print(output + additional + ' {} }},'.format(cycles))

        f.readline()            # flags
        f.readline()            # empty line
        ins = f.readline()      # next ins
        opcode += 1