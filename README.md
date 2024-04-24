# bfc-rs

A brainfuck compiler & interpreter with a shell to boot!

### Installing
You must have the following installed that is not included in this binary:

- qbe (https://c9x.me/compile/)
- cc (probably already a symlink on your system)

Then run:

```bash
cargo build --release
sudo cp target/release/bfc-rs /usr/bin/
```

### Subcommands
`shell` will launch an interactive REPL where you can put full brainfuck commands in.

`interpret <file>` will run the interpreter on the file.

`compile <file>` will compile the program to a static binary.

#### Optimizations
`bfc-rs` offers a couple useful optimizations out of the box:

##### Contractions
Contractions will turn:
```ir
Add(1)
Add(1)
Add(1)
```

into
```ir
Add(3)
```

thereby reducing the amount of instructions taken.

##### Clear loops
Clear loops will turn:
```ir
Loop: [
    Inc(1)
]
```

into
```ir
Clear
```

thereby turning an at max 254 operation into a single constant operation.

##### Dead code
`bfc-rs` will detect dead code that will not affect the programs running and remove it during compile time/interpreting.
