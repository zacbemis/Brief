# Brief

A modern, high-performance programming language implemented in Rust, designed with a focus on simplicity, performance, and developer experience.

## Overview

Brief is a statically-typed programming language with type inference, featuring:

- **Indentation-based syntax** (like Python) for clean, readable code
- **Type inference** with optional type annotations (like Go)
- **Bytecode VM** with register-based architecture for performance
- **Rich standard library** with built-in data structures (arrays, maps, stacks, queues)
- **Object-oriented features** with classes and methods
- **String interpolation** with `&variable` syntax
- **Comprehensive error messages** with precise source locations

## Current Status

### ✅ Completed

- **Lexer** (`brief-lexer`): Full tokenization with support for:
  - All keywords, operators, and punctuation
  - String literals with interpolation (`&name`, `&obj.field`)
  - Character literals with escape sequences
  - Number literals (integers and doubles, including `.5` syntax)
  - Indentation-based blocks (tabs only)
  - Line and block comments
  - Comprehensive test suite
  - **Parser** (`brief-parser`): Recursive-descent parser (planned)
  - **AST** (`brief-ast`): Abstract syntax tree types (planned)
  - **Bytecode** (`brief-bytecode`): Register-based bytecode format (planned)
  - **VM** (`brief-vm`): Interpreter with GC (planned)

### 📋 Planned

- **HIR** (`brief-hir`): Desugaring and name resolution
- **IR** (`brief-ir`): Optional SSA-based intermediate representation
- **Runtime** (`brief-runtime`): Standard library functions
- **CLI** (`brief-cli`): REPL and file execution
- **Package Manager** (`brief-pm`): Dependency management
- **Scheduler**: Multi-threaded work-stealing scheduler
- **FFI**: Safe foreign function interface
- **LLVM/Wasm Backend**: Optional native code generation

## Features

### Language Features

#### Variables and Types

```brief
int x                    // Explicit type declaration
str y                    // String type
x := 1                   // Type-inferred initialization
y := "Hello"             // Infers str from literal
const z := 10            // Immutable variable
```

#### Type Casting

```brief
dub(x)                   // Cast to double
str(x)                   // Cast to string
```

#### Operators

```brief
x + y                    // Arithmetic
x ** 2                   // Power operator
x == y                   // Comparison
x && y                   // Boolean AND
x >> 2                   // Bitwise shift
x++                      // Increment
x += 1                   // Compound assignment
```

#### Control Flow

```brief
if (x == 1)
    ret "x is 1"
else
    ret "x is not 1"

while (i <= 10)
    print(i)
    i++

for (i := 0; i < 10; i++)
    print(i)

for (num in array)
    print(num)

match(grade)
    case 'A'
        print("Excellent")
    else
        print("Other grade")
```

#### Functions

```brief
def add(x, y)
    ret x + y

def add(int x, int y) -> int
    ret x + y

def greet(name)
    print("Hello, &name!")
```

#### Classes and Objects

```brief
cls dog
    obj dog(name)
        // Constructor implicitly sets obj.name = name

    obj def greet()
        print("&obj.name says hi.")

    def bark()
        print("woof")

myDog := dog("sparky")
myDog.bark()              // "woof"
myDog.greet()             // "sparky says hi."
```

#### Data Structures

```brief
int[] x                  // Fixed array
x := int[1, 2, 3, 4, 5]  // Initialize with values
int{} y                  // Dynamic array
int:str{} map            // Map with int keys, str values
x := int{stk}            // Stack
x := int{que}            // Queue
```

#### String Interpolation

```brief
name := "World"
print("Hello, &name!")           // "Hello, World!"
print("Value: &obj.field")        // Interpolate object fields
```

## Building

### Prerequisites

- Rust 1.70+ (or latest stable)
- Cargo

### Build Instructions

```bash
# Clone the repository
git clone <repository-url>
cd Brief

# Build all crates
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```

### Build Individual Crates

```bash
# Build just the lexer
cargo build -p brief-lexer

# Run lexer tests
cargo test -p brief-lexer
```

## Project Structure

```
Brief/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── brief-diagnostic/   # Error reporting and diagnostics
│   ├── brief-lexer/        # Tokenizer
│   ├── brief-ast/          # Abstract syntax tree types
│   ├── brief-parser/       # Recursive-descent parser
│   ├── brief-bytecode/     # Register-based bytecode format
│   ├── brief-vm/           # Virtual machine interpreter
│   ├── brief-runtime/      # Standard library
│   └── brief-cli/          # Command-line interface
├── docs/                   # Documentation
└── tests/                  # End-to-end tests
```

## Architecture

Brief follows a clean, modular architecture:

1. **Lexer** → Tokenizes source code into tokens
2. **Parser** → Builds AST from tokens
3. **HIR** → Desugars and resolves names
4. **IR** → Optional SSA-based optimization
5. **Bytecode** → Register-based bytecode generation
6. **VM** → Executes bytecode with GC

### Key Design Decisions

- **Register-based VM**: Better performance than stack-based
- **Indentation-based blocks**: Cleaner syntax, no braces needed
- **Type inference**: Reduces boilerplate while maintaining safety
- **Direct-threaded dispatch**: Fast VM execution
- **Inline caches**: Optimize method/property lookups
- **Quicken**: Specialize generic opcodes to typed versions

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p brief-lexer

# Run with output
cargo test -- --nocapture
```

## Roadmap

### Phase 1: Bootstrap (Current)
- Lexer with full token support
- Parser for expressions and statements
- Basic VM for arithmetic and control flow
- REPL

### Phase 2: Core Language
- Functions and closures
- Arrays and maps
- Classes and objects
- Standard library

### Phase 3: Performance
- Inline caches
- Opcode quickening
- Garbage collector
- Optimizations

### Phase 4: Advanced Features
- Multi-threading and scheduler
- Package manager
- FFI support
- Future total web support

---
