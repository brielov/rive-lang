# Rive Lang

## Syntax overview

```
# This is a single-line comment

#*
This is a
multi-line
comment
*#

enum Maybe<T> {
    Some(T);
    None;
}

enum Ordering {
    Less;
    Equal;
    Greater;
}

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

proto Addable<Rhs = Self> {
    fn add(self, other: Rhs) -> Self;
}

proto Subtractable<Rhs = Self> {
    fn sub(self, other: Rhs) -> Self;
}

proto Multipliable<Rhs = Self> {
    fn mul(self, other: Rhs) -> Self;
}

proto Divisible<Rhs = Self> {
    fn div(self, other: Rhs) -> Self;
}

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

## Example JavaScript Output

```js
// Enum: Maybe<T>
function maybe_some(value) {
  return { tag: "Some", value };
}

function maybe_none() {
  return { tag: "None" };
}

// Enum: Ordering
const ordering_less = { tag: "Less" };
const ordering_equal = { tag: "Equal" };
const ordering_greater = { tag: "Greater" };

// Helper: Compare Maybe<Ordering> for equality
function maybe_ordering_eq(m1, m2) {
  if (m1.tag === "None" && m2.tag === "None") return true;
  if (m1.tag === "Some" && m2.tag === "Some") {
    return m1.value.tag === m2.value.tag;
  }
  return false;
}

// Proto: Equatable (no direct JS output, just a contract)
// Methods are implemented by types like Point below

// Proto: Comparable (no direct JS output, just a contract)
// Methods are implemented or derived below

// Proto: Addable (no direct JS output, just a contract)
// Proto: Subtractable (no direct JS output, just a contract)
// Proto: Multipliable (no direct JS output, just a contract)
// Proto: Divisible (no direct JS output, just a contract)

// Struct: Point
function point_new(x, y) {
  return { x, y };
}

function point_eq(self, other) {
  return self.x === other.x && self.y === other.y;
}

function point_ne(self, other) {
  return !point_eq(self, other);
}

function point_cmp(self, other) {
  if (point_eq(self, other)) {
    return maybe_some(ordering_equal);
  } else if (self.x > other.x || (self.x === other.x && self.y > other.y)) {
    return maybe_some(ordering_greater);
  } else if (self.x < other.x || (self.x === other.x && self.y < other.y)) {
    return maybe_some(ordering_less);
  } else {
    return maybe_none();
  }
}

function point_gt(self, other) {
  return maybe_ordering_eq(
    point_cmp(self, other),
    maybe_some(ordering_greater)
  );
}

function point_lt(self, other) {
  return maybe_ordering_eq(point_cmp(self, other), maybe_some(ordering_less));
}

function point_gte(self, other) {
  return !maybe_ordering_eq(point_cmp(self, other), maybe_some(ordering_less));
}

function point_lte(self, other) {
  return !maybe_ordering_eq(
    point_cmp(self, other),
    maybe_some(ordering_greater)
  );
}

function point_add(self, other) {
  return point_new(self.x + other.x, self.y + other.y);
}

function point_sub(self, other) {
  return point_new(self.x - other.x, self.y - other.y);
}

function point_mul(self, other) {
  return point_new(self.x * other.x, self.y * other.y);
}

function point_div(self, other) {
  return point_new(self.x / other.x, self.y / other.y);
}
```
