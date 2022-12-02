fn is_reg(inp: &'static str) -> bool {
    inp.contains("a") || inp.contains("b")
}

fn parse_reg(inp: &'static str) -> i32 {
    if inp.contains("a") {
        return 0
    }
    else {
        return 1
    }
}

fn parse_num(inp: &'static str) -> i32 {
    inp.parse::<i32>().unwrap()
}

fn execute_inst(inst: &'static str, regs: &mut Vec<u64>, ip: &mut i32) {
    let parts = inst.split(" ").collect::<Vec<_>>();
    let mut op1 = None;
    let mut op2 = None;

    if is_reg(parts[1]) {
        op1 = Some(parse_reg(parts[1]));
    }
    else {
        op1 = Some(parse_num(parts[1]));
    }

    if parts.len() == 3 {
        if is_reg(parts[2]) {
            op2 = Some(parse_reg(parts[2]));
        }
        else {
            op2 = Some(parse_num(parts[2]));
        }
    }

    match parts[0] {
        "hlf" => {
            regs[op1.unwrap() as usize] /= 2;
            *ip += 1;
        },
        "tpl" => {
            regs[op1.unwrap() as usize] *= 3;
            *ip += 1;
        },
        "inc" => {
            regs[op1.unwrap() as usize] += 1;
            *ip += 1;
        },
        "jmp" => {
            *ip += op1.unwrap();
        },
        "jie" => {
            if regs[op1.unwrap() as usize] % 2 == 0 {
                *ip += op2.unwrap();
            }
            else {
                *ip += 1;
            }
        },
        "jio" => {
            if regs[op1.unwrap() as usize] == 1 {
                *ip += op2.unwrap();
            }
            else {
                *ip += 1;
            }
        },
        _ => {
            println!("Unknown inst");
        }
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let mut regs = vec![1, 0];
    
    let mut ip : i32 = 0x0;
    let insts = content.lines().collect::<Vec<_>>();
    while (ip as usize) < insts.len() {
        execute_inst(insts[ip as usize], &mut regs, &mut ip);
    }

    println!("Val of b: {}", regs[1]);
}
