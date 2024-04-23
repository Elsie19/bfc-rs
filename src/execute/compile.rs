use crate::parse::opcodes::OpCodes;
use qbe::*;

use super::machine::Machine;

pub fn compile(ast: &Vec<OpCodes>, machine: &Machine) {
    let mut module = Module::new();
    generate_qbe(&mut module, &machine, &ast);
    println!("{}", module);
}

fn generate_qbe(module: &mut Module, machine: &Machine, ast: &Vec<OpCodes>) {
    let mut counter = 1;
    let mut while_counter = 1;

    module.add_data(DataDef {
        linkage: Linkage::public(),
        name: "tape".into(),
        align: Some(1),
        items: vec![
            (Type::Byte, DataItem::Const(0)),
            (Type::Zero, DataItem::Const(machine.get_size() as u64 - 1)),
        ],
    });
    module.add_data(DataDef {
        linkage: Linkage::public(),
        name: "ptr".into(),
        align: Some(4),
        items: vec![(Type::Word, DataItem::Const(0))],
    });
    // Create `main`
    let mut func = Function::new(
        Linkage::public(),
        "main".to_string(),
        Vec::new(),
        Some(Type::Word),
    );

    func.add_block("start".to_string());

    // Main logic
    for part in ast {
        match part {
            OpCodes::Inc(x) => {
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Global("ptr".to_string())),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Add(
                        Value::Temporary(format!(".{}", counter - 1)),
                        Value::Const(*x as u64),
                    ),
                );
                counter += 1;
                func.add_instr(Instr::Store(
                    Type::Word,
                    Value::Global("ptr".to_string()),
                    Value::Temporary(format!(".{}", counter - 1)),
                ));
                counter += 1;
            }
            OpCodes::Dec(x) => {
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Global("ptr".to_string())),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Sub(
                        Value::Temporary(format!(".{}", counter - 1)),
                        Value::Const(*x as u64),
                    ),
                );
                counter += 1;
                func.add_instr(Instr::Store(
                    Type::Word,
                    Value::Global("ptr".to_string()),
                    Value::Temporary(format!(".{}", counter - 1)),
                ));
                counter += 1;
            }
            OpCodes::Add(x) => {
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Global("ptr".to_string())),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Exts(Type::Word, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter.to_string())),
                    Type::Long,
                    Instr::Mul(
                        Value::Temporary(format!(".{}", (counter - 1).to_string())),
                        Value::Const(1),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Add(
                        Value::Global("tape".to_string()),
                        Value::Temporary(format!(".{}", counter - 1)),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Loads(Type::Byte, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Exts(Type::Byte, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Add(
                        Value::Temporary(format!(".{}", counter - 1)),
                        Value::Const(*x as u64),
                    ),
                );
                func.add_instr(Instr::Store(
                    Type::Byte,
                    Value::Temporary(format!(".{}", counter - 3)),
                    Value::Temporary(format!(".{}", counter)),
                ));
                counter += 1;
            }
            OpCodes::Sub(x) => {
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Global("ptr".to_string())),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Exts(Type::Word, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter.to_string())),
                    Type::Long,
                    Instr::Mul(
                        Value::Temporary(format!(".{}", (counter - 1).to_string())),
                        Value::Const(1),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Add(
                        Value::Global("tape".to_string()),
                        Value::Temporary(format!(".{}", counter - 1)),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Loads(Type::Byte, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Exts(Type::Byte, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Sub(
                        Value::Temporary(format!(".{}", counter - 1)),
                        Value::Const(*x as u64),
                    ),
                );
                func.add_instr(Instr::Store(
                    Type::Byte,
                    Value::Temporary(format!(".{}", counter - 3)),
                    Value::Temporary(format!(".{}", counter)),
                ));
                counter += 1;
            }
            OpCodes::Output => {
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Global("ptr".to_string())),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Exts(Type::Word, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Mul(
                        Value::Temporary(format!(".{}", counter - 1)),
                        Value::Const(1),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Add(
                        Value::Global("tape".to_string()),
                        Value::Temporary(format!(".{}", counter - 1)),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Loads(Type::Byte, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Exts(Type::Byte, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Call(
                        "putchar".to_string(),
                        vec![(Type::Word, Value::Temporary(format!(".{}", counter - 1)))],
                    ),
                );
                counter += 1;
            }
            OpCodes::Input => {
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Call("getchar".to_string(), vec![]),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Global("ptr".to_string())),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Exts(Type::Word, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter.to_string())),
                    Type::Long,
                    Instr::Mul(
                        Value::Temporary(format!(".{}", (counter - 1).to_string())),
                        Value::Const(1),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Add(
                        Value::Global("tape".to_string()),
                        Value::Temporary(format!(".{}", counter - 1)),
                    ),
                );
                counter += 1;
                func.add_instr(Instr::Store(
                    Type::Byte,
                    Value::Temporary(format!(".{}", counter - 3)),
                    Value::Temporary(format!(".{}", counter)),
                ));
                counter += 1;
            }
            OpCodes::Loop(x) => {
                func.add_block(format!("while_cond.{}", while_counter).to_string());
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Load(Type::Word, Value::Global("ptr".to_string())),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Exts(Type::Word, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter.to_string())),
                    Type::Long,
                    Instr::Mul(
                        Value::Temporary(format!(".{}", (counter - 1).to_string())),
                        Value::Const(1),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Add(
                        Value::Global("tape".to_string()),
                        Value::Temporary(format!(".{}", counter - 1)),
                    ),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Word,
                    Instr::Loads(Type::Byte, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.assign_instr(
                    Value::Temporary(format!(".{}", counter)),
                    Type::Long,
                    Instr::Exts(Type::Byte, Value::Temporary(format!(".{}", counter - 1))),
                );
                counter += 1;
                func.add_instr(Instr::Jnz(
                    Value::Temporary(format!(".{}", counter - 1)),
                    format!("while_body.{}", while_counter + 1),
                    format!("while_join.{}", while_counter + 2),
                ));
                counter += 1;
                func.add_block(format!("while_body.{}", while_counter + 1).to_string());
                func.add_block(format!("while_join.{}", while_counter + 2).to_string());
            }
            _ => (),
        }
    }

    func.add_instr(Instr::Ret(Some(Value::Const(0))));

    module.add_function(func);
}
