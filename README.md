# Rive: A Toy Language for Learning Compilers

Welcome to **Rive**! I’m a web developer building this language purely for fun—to dive into how compilers work and to sharpen my Rust skills along the way. Rive is a playground where I experiment with language design, compiling down to JavaScript, and everything in between. This README gives you a peek at its syntax, the JavaScript it generates, and my roadmap for the project.

## Syntax Overview

Rive blends familiar concepts like structs and enums with a protocol-like system (`proto`) for defining behaviors. Here’s a taste:

```rive
# Single-line comment

#*
Multi-line
comment
*#

# Enums with generics
enum Maybe<T> {
    Some(T);
    None;
}

enum Ordering {
    Less;
    Equal;
    Greater;
}

# Protocols with default implementations
proto Equatable<Rhs = Self> {
    fn eq(self, other: Rhs) -> bool;
    fn ne(self, other: Rhs) -> bool {
        not self.eq(other)
    }
}

proto Comparable<Rhs = Self> : Equatable<Rhs> {
    fn cmp(self, other: Rhs) -> Maybe<Ordering>;
    fn gt(self, other: Rhs) -> bool { self.cmp(other) == Some(Greater) }
    fn lt(self, other: Rhs) -> bool { self.cmp(other) == Some(Less) }
    fn gte(self, other: Rhs) -> bool { self.cmp(other) != Some(Less) }
    fn lte(self, other: Rhs) -> bool { self.cmp(other) != Some(Greater) }
}

# Arithmetic protocols
proto Addable<Rhs = Self> { fn add(self, other: Rhs) -> Self; }
proto Subtractable<Rhs = Self> { fn sub(self, other: Rhs) -> Self; }
proto Multipliable<Rhs = Self> { fn mul(self, other: Rhs) -> Self; }
proto Divisible<Rhs = Self> { fn div(self, other: Rhs) -> Self; }

# A struct implementing multiple protocols
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

# Pattern matching with complex arms and ranges
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

- **Comments**: `#` for single-line, `#* ... *#` for multi-line.
- **Enums**: Tagged unions with optional generics (e.g., `Maybe<T>`).
- **Protocols**: Define interfaces (`proto`) with optional defaults and inheritance.
- **Structs**: Data structures that can implement protocols with public fields and methods.

## Example JavaScript Output

Rive compiles to JavaScript, mapping its features to JS constructs. Here’s how the above code might look:

```javascript
// Enums as tagged objects
const maybe_some = (value) => ({ tag: "Some", value });
const maybe_none = () => ({ tag: "None" });

const ordering = {
  less: { tag: "Less" },
  equal: { tag: "Equal" },
  greater: { tag: "Greater" },
};

// Helper for comparing Maybe<Ordering>
const maybe_ordering_eq = (m1, m2) =>
  (m1.tag === "None" && m2.tag === "None") ||
  (m1.tag === "Some" && m2.tag === "Some" && m1.value.tag === m2.value.tag);

// Struct: Point with protocol implementations
const point = {
  new: (x, y) => ({ x, y }),
  eq: (self, other) => self.x === other.x && self.y === other.y,
  ne: (self, other) => !point.eq(self, other),
  cmp: (self, other) => {
    if (point.eq(self, other)) return maybe_some(ordering.equal);
    if (self.x > other.x || (self.x === other.x && self.y > other.y))
      return maybe_some(ordering.greater);
    if (self.x < other.x || (self.x === other.x && self.y < other.y))
      return maybe_some(ordering.less);
    return maybe_none();
  },
  gt: (self, other) =>
    maybe_ordering_eq(point.cmp(self, other), maybe_some(ordering.greater)),
  lt: (self, other) =>
    maybe_ordering_eq(point.cmp(self, other), maybe_some(ordering.less)),
  gte: (self, other) =>
    !maybe_ordering_eq(point.cmp(self, other), maybe_some(ordering.less)),
  lte: (self, other) =>
    !maybe_ordering_eq(point.cmp(self, other), maybe_some(ordering.greater)),
  add: (self, other) => point.new(self.x + other.x, self.y + other.y),
  sub: (self, other) => point.new(self.x - other.x, self.y - other.y),
  mul: (self, other) => point.new(self.x * other.x, self.y * other.y),
  div: (self, other) => point.new(self.x / other.x, self.y / other.y),
};

// Pattern matching with complex arms and ranges
function describe_value(m, n, c) {
  if (
    m.tag === "Some" &&
    m.value.tag === "Less" &&
    n >= 0 &&
    n <= 9 &&
    c >= 97 &&
    c <= 122 // 'a' = 97, 'z' = 122
  )
    return "Less with small number and lowercase";

  if (m.tag === "Some" && m.value.tag === "Equal" && n >= 10 && n < 100)
    return "Equal with medium number";

  if (
    m.tag === "Some" &&
    m.value.tag === "Greater" &&
    n >= -10 &&
    n < 0 &&
    c >= 65 &&
    c <= 90 // 'A' = 65, 'Z' = 90
  )
    return "Greater with negative bound to n";

  if (
    m.tag === "None" &&
    c >= 48 &&
    c <= 57 // '0' = 48, '9' = 57
  )
    return "Unknown with digit";

  return "Something else";
}
```

- **Enums**: Represented as factory functions returning tagged objects.
- **Protocols**: Implicitly enforced; methods are attached to structs.
- **Structs**: Single objects with a `new` constructor and method implementations.

## Project Goals

Rive’s compiler is a work-in-progress, built to explore the full compilation pipeline in Rust. Here’s where I’m at and what’s ahead:

- [x] **Lexing**: Tokenize the source with spans.

  - [x] Defined token types (e.g., `Pub`, `Identifier`).
  - [x] Built a lexer producing `WithSpan<Token>`.
  - [x] Handled whitespace and special characters.

- [ ] **Parsing**: Turn tokens into an Abstract Syntax Tree (AST).

  - [ ] Define AST nodes for Rive’s grammar (structs, enums, protos).
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
