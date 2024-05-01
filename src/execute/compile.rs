use crate::execute::machine::Machine;
use crate::parse::opcodes::{OpCodes, Tokens};
use qbe::*;

/// Return our QBE IR, and also a bool if this should be statically compiled
pub fn compile(
    ast: &Vec<Tokens>,
    machine: &Machine,
    debug: &bool,
    file_location: String,
) -> (String, bool) {
    let mut output_string = String::new();

    // If so, let's just create a silly little return
    if ast.is_empty() {
        let mut module = Module::new();
        let mut func = Function::new(
            Linkage::public(),
            "main".to_owned(),
            Vec::new(),
            Some(Type::Word),
        );
        func.add_block("start".to_owned());
        func.add_instr(Instr::Ret(Some(Value::Const(0))));
        module.add_function(func);
        return (module.to_string(), false);
    }

    let mut module = Module::new();
    let mut counter = 1;
    let mut while_counter = 1;
    module.add_data(DataDef {
        linkage: Linkage::private(),
        name: "tape".into(),
        align: Some(8),
        items: vec![(Type::Zero, DataItem::Const(machine.get_size() as u64))],
    });
    // Create `main`
    if *debug {
        output_string.push_str(&format!(r#"dbgfile "{}""#, file_location));
    }
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

    generate_qbe(ast, &mut counter, &mut while_counter, *debug, &mut func);
    func.add_instr(Instr::Ret(Some(Value::Const(0))));
    module.add_function(func);
    (
        if !output_string.is_empty() {
            output_string + "\n"
        } else {
            "".to_string()
        } + &module.to_string(),
        true,
    )
}

fn format_counter(value: i32) -> String {
    format!(".{value}")
}

fn format_label(value: i32) -> String {
    format!("loop.{value}")
}

fn generate_qbe(
    ast: &Vec<Tokens>,
    counter: &mut i32,
    while_counter: &mut i32,
    debug: bool,
    func: &mut Function<'_>,
) {
    // Main logic
    for part in ast {
        match part.get_type() {
            // Ok this is for people possibly looking into using QBE for their brainfuck compiler
            // but are totally confused as how to implement it, just like I was, so I'll explain it
            // here. You're welcome ;)
            //
            // So the first thing which is up a bit in the code which is important is this:
            //     storel $tape, %.1
            // which is basicaly just storing a number (%.1) which acts like a pointer to $tape.
            OpCodes::Inc(x) => {
                // For this set, we are incrementing the pointer, but we don't really have a
                // pointer in the compiled program. So what do we do?
                //
                // %.2 =l loadl %.1
                // %.3 =l add %.2, x*4
                // storel %.3, %.1
                //
                // Recall that %.1 is assigned to an alloc8 8: this is just our pointer of sorts.
                // All this function does is increment the "pointer", which is really just an int.
                if debug {
                    func.add_instr(Instr::DbgLoc(
                        part.get_location().0 as u64,
                        Some(part.get_location().1 as u64),
                    ));
                }
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
                        Value::Const(u64::from(*x) * 4),
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
                // Same for this one, but use Sub instead of Add
                if debug {
                    func.add_instr(Instr::DbgLoc(
                        part.get_location().0 as u64,
                        Some(part.get_location().1 as u64),
                    ));
                }
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
                        Value::Const(u64::from(*x) * 4),
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
                // %.2 =l loadl %.1
                // %.3 =w loadw %.2
                // %.4 =w add %.3, x
                // storew %.4, %.2
                //
                // So what this does is load our pointer into a register (%.2), then does a
                // conversion to make it a word in %.3, then we add our x into what we got from
                // %.3, then we store that computation into our original %.2 which loaded the
                // pointer.
                if debug {
                    func.add_instr(Instr::DbgLoc(
                        part.get_location().0 as u64,
                        Some(part.get_location().1 as u64),
                    ));
                }
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
                if debug {
                    func.add_instr(Instr::DbgLoc(
                        part.get_location().0 as u64,
                        Some(part.get_location().1 as u64),
                    ));
                }
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
                // Same drill, we load our pointer, then we run the C putchar function on our
                // loaded pointer
                if debug {
                    func.add_instr(Instr::DbgLoc(
                        part.get_location().0 as u64,
                        Some(part.get_location().1 as u64),
                    ));
                }
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
                if debug {
                    func.add_instr(Instr::DbgLoc(
                        part.get_location().0 as u64,
                        Some(part.get_location().1 as u64),
                    ));
                }
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
                if debug {
                    func.add_instr(Instr::DbgLoc(
                        part.get_location().0 as u64,
                        Some(part.get_location().1 as u64),
                    ));
                }
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
                generate_qbe(ast, counter, while_counter, debug, func);
                func.add_instr(Instr::Jmp(format_label(returned_while)));
                func.add_block(format_label(returned_while + 2));
            }
            OpCodes::Clear => {
                // Functions basically the same as `OpCodes::Inc/Dec` but instead of running add or
                // sub on it, we just copy the value 0.
                if debug {
                    func.add_instr(Instr::DbgLoc(
                        part.get_location().0 as u64,
                        Some(part.get_location().1 as u64),
                    ));
                }
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
