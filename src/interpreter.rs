use std::io::stdio::stdin_raw;

const MEMSIZE: uint = 0xfe;

fn exec(ins: &Vec<char>, mem: &mut [u8, ..MEMSIZE], pos:uint) -> u8 {
    let mut cur = 0u;
    let mut pos = pos;
    let mut inp = stdin_raw();
    loop {
        match ins.get(pos) {
            Some(&'<') => {cur=(cur-1)%MEMSIZE;}
            Some(&'>') => {cur=(cur+1)%MEMSIZE;}
            Some(&'-') => {mem[cur]-=1;}
            Some(&'+') => {mem[cur]+=1;}
            Some(&'.') => {print!("{}",mem[cur] as char);}
            Some(&',') => {mem[cur]=inp.read_byte().unwrap();}
            Some(&'[') => {
                while mem[cur] != 0 {
                    exec(ins, mem, pos+1);
                }
                while ins.get(pos) != Some(&']') {pos+=1}
            }
            Some(&']') | None => {break;}
            _ => {/*comment*/}
        }
        pos += 1;
    }
    mem[cur]
}

fn main() {
    let s = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.".chars().collect();
    exec(&s, &mut [0u8, ..MEMSIZE], 0);
}

