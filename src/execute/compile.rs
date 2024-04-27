use crate::parse::opcodes::OpCodes;
use qbe::*;

use super::machine::Machine;

pub fn compile(ast: &Vec<OpCodes>, machine: &Machine) -> String {
    let mut module = Module::new();
    let mut counter = 1;
    let mut while_counter = 1;
    module.add_data(DataDef {
        linkage: Linkage::private(),
        name: "tape".into(),
        align: Some(8),
        items: vec![
            // (Type::Byte, DataItem::Const(0)),
            (Type::Zero, DataItem::Const(machine.get_size() as u64)),
        ],
    });
    // Create `main`
    let mut func = Function::new(
        Linkage::public(),
        "main".to_owned(),
        Vec::new(),
        Some(Type::Word),
    );
    func.add_block("start".to_owned());

    // %.1 =l alloc8 8
    func.assign_instr(
        Value::Temporary(format_counter(counter)),
        Type::Long,
        Instr::Alloc8(8),
    );

    // storel $tape, %.1
    func.add_instr(Instr::Store(
        Type::Long,
        Value::Temporary(format_counter(counter)),
        Value::Global("tape".to_owned()),
    ));

    // %.2 =l loadl $stdout
    func.assign_instr(
        Value::Temporary(format_counter(counter + 1)),
        Type::Long,
        Instr::Load(Type::Long, Value::Global("stdout".to_owned())),
    );

    // %.3 =l extsw 0
    func.assign_instr(
        Value::Temporary(format_counter(counter + 2)),
        Type::Long,
        Instr::Exts(Type::Word, Value::Const(0)),
    );

    // call $setbuf(l %.2, l %.3)
    // The goal of this is to disable buffering, because it will give some programs that print a
    // lot in a single line but takes a while a visible speed boost so that the user can see
    // something is going on.
    func.add_instr(Instr::Call(
        "setbuf".to_owned(),
        vec![
            (Type::Long, Value::Temporary(format_counter(counter + 1))),
            (Type::Long, Value::Temporary(format_counter(counter + 2))),
        ],
    ));

    counter += 2;

    generate_qbe(ast, &mut counter, &mut while_counter, &mut func);
    func.add_instr(Instr::Ret(Some(Value::Const(0))));
    module.add_function(func);
    module.to_string()
}

fn format_counter(value: i32) -> String {
    format!(".{}", value)
}

fn format_label(value: i32) -> String {
    format!("loop.{}", value)
}

fn generate_qbe(
    ast: &Vec<OpCodes>,
    counter: &mut i32,
    while_counter: &mut i32,
    func: &mut Function<'_>,
) {
    // Main logic
    for part in ast {
        match part {
            OpCodes::Inc(x) => {
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 1)),
                    Type::Long,
                    Instr::Load(Type::Long, Value::Temporary(format_counter(1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 2)),
                    Type::Long,
                    Instr::Add(
                        Value::Temporary(format_counter(*counter + 1)),
                        Value::Const(*x as u64 * 4),
                    ),
                );
                func.add_instr(Instr::Store(
                    Type::Long,
                    Value::Temporary(format_counter(1)),
                    Value::Temporary(format_counter(*counter + 2)),
                ));
                *counter += 2;
            }
            OpCodes::Dec(x) => {
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 1)),
                    Type::Long,
                    Instr::Load(Type::Long, Value::Temporary(format_counter(1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 2)),
                    Type::Long,
                    Instr::Sub(
                        Value::Temporary(format_counter(*counter + 1)),
                        Value::Const(*x as u64 * 4),
                    ),
                );
                func.add_instr(Instr::Store(
                    Type::Long,
                    Value::Temporary(format_counter(1)),
                    Value::Temporary(format_counter(*counter + 2)),
                ));
                *counter += 2;
            }
            OpCodes::Add(x) => {
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 1)),
                    Type::Long,
                    Instr::Load(Type::Long, Value::Temporary(format_counter(1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 2)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Temporary(format_counter(*counter + 1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 3)),
                    Type::Word,
                    Instr::Add(
                        Value::Temporary(format_counter(*counter + 2)),
                        Value::Const(*x as u64),
                    ),
                );
                func.add_instr(Instr::Store(
                    Type::Word,
                    Value::Temporary(format_counter(*counter + 1)),
                    Value::Temporary(format_counter(*counter + 3)),
                ));
                *counter += 3;
            }
            OpCodes::Sub(x) => {
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 1)),
                    Type::Long,
                    Instr::Load(Type::Long, Value::Temporary(format_counter(1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 2)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Temporary(format_counter(*counter + 1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 3)),
                    Type::Word,
                    Instr::Sub(
                        Value::Temporary(format_counter(*counter + 2)),
                        Value::Const(*x as u64),
                    ),
                );
                func.add_instr(Instr::Store(
                    Type::Word,
                    Value::Temporary(format_counter(*counter + 1)),
                    Value::Temporary(format_counter(*counter + 3)),
                ));
                *counter += 3;
            }
            OpCodes::Output => {
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 1)),
                    Type::Long,
                    Instr::Load(Type::Long, Value::Temporary(format_counter(1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 2)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Temporary(format_counter(*counter + 1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 3)),
                    Type::Word,
                    Instr::Call(
                        "putchar".to_owned(),
                        vec![(Type::Word, Value::Temporary(format_counter(*counter + 2)))],
                    ),
                );
                *counter += 3;
            }
            OpCodes::Input => {
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 1)),
                    Type::Word,
                    Instr::Call("getchar".to_owned(), vec![]),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 2)),
                    Type::Long,
                    Instr::Load(Type::Long, Value::Temporary(format_counter(1))),
                );
                func.add_instr(Instr::Store(
                    Type::Word,
                    Value::Temporary(format_counter(*counter + 2)),
                    Value::Temporary(format_counter(*counter + 1)),
                ));
                *counter += 2;
            }
            OpCodes::Loop(ast) => {
                func.add_block(format_label(*while_counter));
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 1)),
                    Type::Long,
                    Instr::Load(Type::Long, Value::Temporary(format_counter(1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 2)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Temporary(format_counter(*counter + 1))),
                );
                func.add_instr(Instr::Jnz(
                    Value::Temporary(format_counter(*counter + 2)),
                    format_label(*while_counter + 1),
                    format_label(*while_counter + 2),
                ));
                func.add_block(format_label(*while_counter + 1));
                *counter += 3;
                let returned_while = *while_counter;
                *while_counter += 3;
                generate_qbe(ast, counter, while_counter, func);
                func.add_instr(Instr::Jmp(format_label(returned_while)));
                func.add_block(format_label(returned_while + 2));
            }
            OpCodes::Clear => {
                // Functions basically the same as `OpCodes::Inc/Dec` but instead of running add or
                // sub on it, we just copy the value 0.
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 1)),
                    Type::Long,
                    Instr::Load(Type::Long, Value::Temporary(format_counter(1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 2)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Temporary(format_counter(*counter + 1))),
                );
                func.assign_instr(
                    Value::Temporary(format_counter(*counter + 3)),
                    Type::Word,
                    Instr::Copy(Value::Const(0)),
                );
                func.add_instr(Instr::Store(
                    Type::Word,
                    Value::Temporary(format_counter(*counter + 1)),
                    Value::Temporary(format_counter(*counter + 3)),
                ));
                *counter += 3;
            }
        }
    }
}
