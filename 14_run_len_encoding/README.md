# Rust Code Challenges: Challenge 14

Your challenge is to implement [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding).
Design functions `encode()` and `decode()` that takes can encode and decode `&str` (string slices).

**Note:** When a run has more than 9 consecutive elements of the same character, you should break the run into two to avoid parsing ambiguities.

**Note:** To make the challenge more difficult, modify your `encode()` function to accept arbitrary data streams (`[u8]`), while returning valid `String`. Your `decode()` function should do the inverse. This will require you to introduce an escaping mechanism to enable unprintable bytes to be printed.

## Testing your solution

Use `cargo test` to evaluate yourself:

```console
$ cargo test
...
running 2 tests
test abc ... ok
test round_trip ... ok
...
```

## How it works

Prefix the character with the number of contiguous appearances of that character.

```console
LinkedIn    1L1i1n1k1e1d1I1n

AAAAAaAA    5A1a2A
```

## Extra challenges

### Long run

Implement a system/algorithm for handling runs that require more than one digit to represent, e.g. more than nine identical values in a row.

### Arbitrary byte streams

We are currently restricted to valid UTF-8, so design an algorithm that allows any byte to be encoded.
