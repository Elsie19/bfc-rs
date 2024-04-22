use crate::parse::opcodes::OpCodes;
use qbe::*;

use super::machine::Machine;

pub fn compile(ast: &Vec<OpCodes>, machine: &Machine) {
    let mut module = Module::new();
    generate_qbe(&mut module, &machine, &ast);
    println!("{}", module);
}

fn generate_qbe(module: &mut Module, machine: &Machine, ast: &Vec<OpCodes>) {
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
    func.add_instr(Instr::Ret(Some(Value::Const(0))));

    // for part in ast {
    //     match part {
    //         OpCodes::Inc(x) => todo!(),
    //     }
    // }

    module.add_function(func);
}
