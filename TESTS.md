# Tests

These are all the tests I've run to ensure that bfc-rs is up to brainfuck standards (well there's none but whatever):

| Program | Interpreter  | Compiler     | Notes |
|---------|--------------|-----------|------------|
| `e.b`  | ✅ | ❌      | Segfaults on compiler, I assume it's an integer overflow  |
| `Mandelbrot.b`  | ✅ |  ✅ | |
| `bitwidth.b`  | ✅ |  ✅ | I had to do commit 2c54339 to make sure the interpreter outputs the same as the compiler |
| `hanoi.b`  | ✅ |  ✅ | |
