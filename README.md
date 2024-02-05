# About
A toy C Compiler written in pure rust.  

## Features

### Descriptive (and Beautiful) Error Messages
```c
int main() {
    int a;
    int a;
    return 0;
}
```
Produces Error  
![image](https://github.com/nouritsu/rs-rcc/assets/113834791/e5e70bd5-8942-42ba-ac32-eaf1c462ca92)

### AST Printer
```c
int main() {
    return 0;
}
```
Prints  
```rs
[
    (
        FnDeclaration(
            "main",
            [
                (
                    Block(
                        [
                            (
                                Return(
                                    (
                                        LiteralInteger(
                                            0,
                                        ),
                                        24..25,
                                    ),
                                ),
                                17..26,
                            ),
                        ],
                    ),
                    11..28,
                ),
            ],
        ),
        0..28,
    ),
]
```

### (Somewhat) Debuggable Assembly
```c
int main() {
    return 1 && 2;
}
```
Produces  
```asm
	.globl main
main:
	push %rbp
	mov %rsp, %rbp
	mov $1, %rax
	cmp $0, %rax
	jne and_0
	jmp and_ss_0
and_0:
	mov $2, %rax
	cmp $0, %rax
	mov $0, %rax
	setne %al
and_ss_0:
	mov %rbp, %rsp
	pop %rbp
	ret

```
## Installation
Using Cargo (Install rust first, from https://www.rust-lang.org)-
```
cargo install --git https://github.com/nouritsu/rs-rcc/
```
Then use using
```
rcc <file> -o <file>
```

## Getting Help
- Try running `rcc --help` or `rcc -h` for a descriptive help message
- Open an issue on this repository describing your issue and the solution you have attempted

# Plans for the Project
- AST Printer that produces a PNG diagram rather than pretty printing an ugly vec to the terminal
- Cargo benchmarks for lexer, parser, codegen
- Option to add comments to generated assembly so this compiler can be used for educational purposes

# Built using
- [chumsky](https://github.com/zesterer/chumsky) for lexing and parsing - an easy to learn, elegant parser combinator crate for Rust
- [ariadne](https://github.com/zesterer/ariadne) for displaying error messages - integrates EXTREMELY well with chumsky
