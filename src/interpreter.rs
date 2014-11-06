use std::io::stdio::stdin_raw;

const MEMSIZE: uint = 0xfe;

fn exec(ins: &Vec<char>, mem: &mut [u8], cur:uint, pos:uint) -> uint {
    let mut cur = cur;
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
                    cur = exec(ins, mem, cur, pos+1);
                }
                while ins.get(pos) != Some(&']') {pos+=1}
            }
            Some(&']') | None => {break;}
            _ => {/*comment*/}
        }
        pos += 1;
    }
    cur
}

#[test]
fn generate_increasing() {
    let mut mem = Vec::from_elem(MEMSIZE, 0u8);
    exec(&"+>++>+++>+[<-]".chars().collect(), &mut *mem.as_mut_slice(), 0, 0);
    assert_eq!(mem[0..3], [0,1,2].as_slice());
}

pub fn interpret(s: &str) {
    let s = s.chars().collect();
    exec(&s, &mut [0u8, ..MEMSIZE], 0, 0);
}
