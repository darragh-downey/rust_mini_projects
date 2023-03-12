# Rust Code Challenges: Challenge 11

Your challenge is to implement a code that
can parse and validate an ISBN-13 number.

## Testing your solution

Use `cargo test` to evaluate yourself:

```console
$ cargo test
...
```

## Steps

### Step 1

Remove hyphens and isolate check digit

```console
978-3-16-148410-0

978316148410     0
```

### Step 2

Apply weights

```console
9 7 8 3 
1 6 1 4 
8 4 1 0

9x1 7x3 8x1 3x3 
1x1 6x3 1x1 4x3 
8x1 4x3 1x1 0x3

9 21 8 9
1 18 1 12
8 12 1 0
```

### Step 3

Sum result

```console
9 21 8 9
1 18 1 12
8 12 1 0  = 100
```

### Step 4

Divide by 10 and take the remainder (% 10)

```console
100 % 10 = 0
```

### Step 5

Subtract the remainder from 10

```console
0 - 10 = 10
```

### Step 6

Take the last digit

```console
10
 ^
```
