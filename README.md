# Rive: A Toy Language for Learning Compilers

Welcome to **Rive**! I’m a web developer building this language purely for fun—to dive into how compilers work and to sharpen my Rust skills along the way. Rive is a playground where I experiment with language design, compiling down to JavaScript, and everything in between. This README gives you a peek at its syntax, the JavaScript it generates, and my roadmap for the project.

## Syntax Overview

#### Single-Line Comment

```rive
# Single-line comment
```

**Compiled JS:**

```javascript
// Single-line comment
```

- Simple: `#` becomes `//`, and the comment text follows. No runtime impact.

---

#### Multi-Line Comment

```rive
#*
Multi-line
comment
*#
```

**Compiled JS:**

```javascript
/*
Multi-line
comment
*/
```

- `#*` and `*#" map to `/_`and`_/` for JS multi-line comments. Again, no runtime code.

---

#### Enum with Generics: `Maybe<T>`

```rive
enum Maybe<T> {
    Some(T);
    None;
}
```

**Compiled JS:**

```javascript
// Some(T) -> [1, value]
// None -> [0]
```

- As established, variants use integer tags in arrays:
  - `Some(T)` → `[1, value]`
  - `None` → `[0]`
- Generics (`T`) are erased at compile time (like Rust’s monomorphization), so no JS runtime representation is needed. The type-checker ensures `T` is valid.

---

#### Enum without Generics: `Ordering`

```rive
enum Ordering {
    Less;
    Equal;
    Greater;
}
```

**Compiled JS:**

```javascript
// Less -> [0]
// Equal -> [1]
// Greater -> [2]
```

- No-data variants get sequential tags:
  - `Less` → `[0]`
  - `Equal` → `[1]`
  - `Greater` → `[2]`
- Simple and minimal, with no runtime type info.

---

#### Protocol with Default Implementation: `Equatable`

```rive
proto Equatable<Rhs = Self> {
    fn eq(self, other: Rhs) -> bool;
    fn ne(self, other: Rhs) -> bool {
        !self.eq(other)
    }
}
```

**Compiled JS:**

```javascript
// No code generated for the protocol itself.
// Abstract method eq must be implemented by structs.
// Default method ne is generated per implementing struct.
```

For an implementing struct like `Point`:

- `eq` → `point_eq(self, other)` (user-provided).
- `ne` → `point_ne(self, other)` (auto-generated as `!point_eq(self, other)`).

---

#### Protocol with Inheritance: `Comparable`

```rive
proto Comparable<Rhs = Self> : Equatable<Rhs> {
    fn cmp(self, other: Rhs) -> Maybe<Ordering>;
    fn gt(self, other: Rhs) -> bool { self.cmp(other) == Some(Greater) }
    fn lt(self, other: Rhs) -> bool { self.cmp(other) == Some(Less) }
    fn gte(self, other: Rhs) -> bool { self.cmp(other) != Some(Less) }
    fn lte(self, other: Rhs) -> bool { self.cmp(other) != Some(Greater) }
}
```

**Compiled JS:**

```javascript
// cmp must be implemented by conforming types
function comparable_gt(self, other) {
  let result = comparable_cmp(self, other);
  return result[0] === 1 && result[1][0] === 2; // Some(Greater)
}
function comparable_lt(self, other) {
  let result = comparable_cmp(self, other);
  return result[0] === 1 && result[1][0] === 0; // Some(Less)
}
function comparable_gte(self, other) {
  let result = comparable_cmp(self, other);
  return !(result[0] === 1 && result[1][0] === 0); // Not Some(Less)
}
function comparable_lte(self, other) {
  let result = comparable_cmp(self, other);
  return !(result[0] === 1 && result[1][0] === 2); // Not Some(Greater)
}
```

- `cmp` is abstract, implemented by conforming types (e.g., `comparable_cmp`).
- Default methods are standalone functions:
  - `Maybe<Ordering>` uses `[1, [n]]` for `Some` and `[0]` for `None`.
  - `Ordering` uses `[0]`, `[1]`, `[2]` for `Less`, `Equal`, `Greater`.
  - `==` and `!=` translate to deep comparisons of the tagged arrays.
- Minimal logic: no unnecessary allocations, just tag checks.

---

#### Arithmetic Protocol: `Addable`

```rive
proto Addable<Rhs = Self> { fn add(self, other: Rhs) -> Self; }
```

**Compiled JS:**

```javascript
// add must be implemented by conforming types
```

- Purely abstract, so no JS code unless implemented by a struct. The prefix `addable_add` would be used by implementors.

---

#### Struct with Protocol Conformance: `Point`

```rive
struct Point : Comparable, Addable, Subtractable, Multipliable, Divisible {
    pub x: float;
    pub y: float;

    fn eq(self, other: Self) -> bool {
        self.x == other.x and self.y == other.y
    }

    fn cmp(self, other: Self) -> Maybe<Ordering> {
        if self.eq(other) { Some(Equal) }
        else if self.x > other.x or (self.x == other.x and self.y > other.y) { Some(Greater) }
        else if self.x < other.x or (self.x == other.x and self.y < other.y) { Some(Less) }
        else { None }
    }

    fn add(self, other: Self) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y }
    }

    fn sub(self, other: Self) -> Self {
        Point { x: self.x - other.x, y: self.y - other.y }
    }

    fn mul(self, other: Self) -> Self {
        Point { x: self.x * other.x, y: self.y * other.y }
    }

    fn div(self, other: Self) -> Self {
        Point { x: self.x / other.x, y: self.y / other.y }
    }
}
```

**Compiled JS:**

```javascript
// Point as a plain object: {x: number, y: number}
function point_eq(self, other) {
  return self.x === other.x && self.y === other.y;
}
function point_ne(self, other) {
  return !point_eq(self, other);
}
function comparable_cmp(self, other) {
  if (point_eq(self, other)) return [1, [1]]; // Some(Equal)
  if (self.x > other.x || (self.x === other.x && self.y > other.y))
    return [1, [2]]; // Some(Greater)
  if (self.x < other.x || (self.x === other.x && self.y < other.y))
    return [1, [0]]; // Some(Less)
  return [0]; // None
}
function addable_add(self, other) {
  return { x: self.x + other.x, y: self.y + other.y };
}
function subtractable_sub(self, other) {
  return { x: self.x - other.x, y: self.y - other.y };
}
function multipliable_mul(self, other) {
  return { x: self.x * other.x, y: self.y * other.y };
}
function divisible_div(self, other) {
  return { x: self.x / other.x, y: self.y / other.y };
}
```

- **Struct Representation**: `Point` compiles to a plain JS object `{x, y}`, minimal and native.
- **Methods**: Standalone functions with struct-specific prefixes (e.g., `point_eq`, `addable_add`).
- **Protocol Conformance**: Each method satisfies its protocol, reusing `point_eq` in `cmp` and auto-generating `point_ne`.
- **Operators**: `and` → `&&`, `==` → `===`, etc., for JS compatibility.

---

#### Pattern Matching: `describe_value`

```rive
fn describe_value(m: Maybe<Ordering>, n: int, c: char) -> str {
    match [m, n, c] {
        [Some(Less), 0..=9, 'a'..='z'] => "Less with small number and lowercase",
        [Some(Equal), 10..100, _] => "Equal with medium number",
        [Some(Greater), n @ -10..0, 'A'..='Z'] => "Greater with negative bound to n",
        [None, _, '0'..='9'] => "Unknown with digit",
        _ => "Something else",
    }
}
```

**Compiled JS:**

```javascript
function describe_value(m, n, c) {
  if (m[0] === 1 && m[1][0] === 0 && n >= 0 && n <= 9 && c >= "a" && c <= "z") {
    return "Less with small number and lowercase";
  } else if (m[0] === 1 && m[1][0] === 1 && n >= 10 && n < 100) {
    return "Equal with medium number";
  } else if (
    m[0] === 1 &&
    m[1][0] === 2 &&
    n >= -10 &&
    n < 0 &&
    c >= "A" &&
    c <= "Z"
  ) {
    return "Greater with negative bound to n";
  } else if (m[0] === 0 && c >= "0" && c <= "9") {
    return "Unknown with digit";
  } else {
    return "Something else";
  }
}
```

- **Tuple**: `[m, n, c]` is passed as separate arguments; no runtime tuple object.
- **Patterns**:
  - `Some(Less)` → `m[0] === 1 && m[1][0] === 0` (nested enum check).
  - `0..=9` → `n >= 0 && n <= 9` (inclusive range).
  - `'a'..='z'` → `c >= "a" && c <= "z"` (char range as string comparison).
  - `10..100` → `n >= 10 && n < 100` (exclusive range).
  - `n @ -10..0` → `n >= -10 && n < 0` (binding uses parameter `n` directly).
  - `_` → no condition.
- **Minimal**: No arrays or objects created at runtime; pure conditionals.

---

#### Variable Definition with Enum Literal

```rive
let a = Maybe::Some(10);
```

**Compiled JS:**

```javascript
let a = [1, 10]; // Some(10)
```

- `Maybe::Some(10)`:
  - `Maybe<T>` uses `[1, value]` for `Some` (tag `1`).
  - `10` is the payload, placed at index 1.
- `let a = ...` directly assigns the constructed value.
- Minimal: no runtime type info, just the tagged array.

---

#### Variable Definition with Struct Literal

```rive
let b = Point { x: 1, y: 2 };
```

**Compiled JS:**

```javascript
let b = { x: 1, y: 2 };
```

- `Point { x: 1, y: 2 }`:
  - `Point` is a plain JS object with fields `x` and `y`.
  - Field initializers map directly to object properties.
- `let b = ...` assigns the object literal.
- Lean and native JS, no overhead.

---

#### Loop Expression: `loop`

```rive
let x = loop {
    break 42;
};
```

**Compiled JS:**

```javascript
let x;
while (true) {
  x = 42;
  break;
}
```

- `loop` is an infinite loop, like Rust’s, and an expression that can yield a value via `break`.
- Compiled to a `while (true)` loop with `break` assigning the result to an outer variable.
- Minimal: no extra constructs, just native JS control flow.

---

#### For Loop Expression

```rive
let sum = for x in 0..5 {
    break x + 1;
};
```

**Compiled JS:**

```javascript
let sum;
for (let x = 0; x < 5; x++) {
  sum = x + 1;
  break;
}
```

- `for x in 0..5` iterates over an exclusive range (iterator TBD, here assumed as a simple range).
- Expression returns a value via `break`, assigned to `sum`.
- Compiled to a JS `for` loop; range `[0, 0, 5]` interpreted as `x < 5`.
- Iterator protocol still to be defined.

---

#### While Loop Expression

```rive
let mut n = 0;
let result = while n < 3 {
    n = n + 1;
    break n;
};
```

**Compiled JS:**

```javascript
let n = 0;
let result;
while (n < 3) {
  n = n + 1;
  result = n;
  break;
}
```

- `while` loops until the condition fails, returning a value via `break`.
- `mut` allows `n` to be reassigned; JS uses `let` since block-scoped.
- Simple `while` loop with assignment on `break`.

---

#### String Interpolation

```rive
let name = "world";
let greeting = "Hey, #{name}";
```

**Compiled JS:**

```javascript
let name = "world";
let greeting = `Hey, ${name}`;
```

- Inspired by Ruby, `#{expr}` interpolates expressions in strings.
- Maps to JS template literals with `${expr}`.
- Type-checker ensures `expr` stringifies correctly; no runtime overhead beyond JS strings.

---

### General Compilation Notes

- **Enums**: `EnumName::Variant(params)` → `[tag, ...params]` (e.g., `Maybe::Some(10)` → `[1, 10]`).
- **Ranges**: `[0, start, end]` (exclusive) or `[1, start, end]` (inclusive).
- **Structs**: `StructName { field: value, ... }` → `{ field: value, ... }`.
- **Methods**: Standalone functions with `self` as the first argument, prefixed by protocol/struct name.
- **Protocols**: Abstract methods are placeholders; defaults are compiled functions scoped to the implementing struct.
- **Pattern Matching**: Translated to `if-else` chains with tag and range checks.
- **Variables**: `let name = expr` → `let name = compiled_expr`.
- **Loops**: `loop`, `for`, `while` are expressions; compiled to JS loops with `break` assigning results.
- **Strings**: `#{expr}` → `${expr}` in template literals.

## Project Goals

Rive’s compiler is a work-in-progress, built to explore the full compilation pipeline in Rust. Here’s where I’m at and what’s ahead:

- [x] **Lexing**: Tokenize the source with spans.

  - [x] Defined token types (e.g., `Pub`, `Identifier`).
  - [x] Built a lexer producing `WithSpan<Token>`.
  - [x] Handled whitespace and special characters.

- [ ] **Parsing**: Turn tokens into an Abstract Syntax Tree (AST).

  - [x] Define AST nodes for Rive’s grammar (structs, enums, protos).
  - [ ] Write a parser to construct the AST.
  - [ ] Add error reporting with spans.

- [ ] **Type-Checking**: Ensure semantic correctness.

  - [ ] Design a type system (e.g., `float`, `Self`, generics).
  - [ ] Implement type checking for expressions and protocols.
  - [ ] Report type errors with spans.

- [ ] **Linting**: Catch style and potential issues.

  - [ ] Set lint rules (e.g., unused variables).
  - [ ] Build a linter to analyze the AST.
  - [ ] Offer suggestions with span info.

- [ ] **Formatting**: Standardize code style.

  - [ ] Define rules (e.g., indentation, spacing).
  - [ ] Create a formatter using the AST or spans.
  - [ ] Ensure consistent output.

- [ ] **Code Generation**: Emit JavaScript.

  - [ ] Map Rive constructs to JS (e.g., `struct` to objects).
  - [ ] Build a generator to produce JS from the AST.
  - [ ] Add runtime support if needed.

- [ ] **Integration**: Glue it all together.

  - [ ] Sequence the pipeline: lex → parse → type-check → lint → format → codegen.
  - [ ] Unify error handling with spans.
  - [ ] Add a CLI for easy use.

This is my learning journey—feel free to follow along or suggest ideas as I go!
