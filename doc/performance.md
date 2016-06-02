# Performance

## 1000 Random Games To End

Using: `time cargo run`.

### Set A

Using the Rust Nightly from perhaps 2016-05-25.

#### Trial 1

```
X wins:  428
O wins:  345
  ties:  227
cargo run  79.31s user 0.10s system 99% cpu 1:19.47 total
```

#### Trial 2

```
X wins:  428
O wins:  345
  ties:  227
cargo run  78.16s user 0.21s system 99% cpu 1:18.66 total
```

#### Trial 3

```

X wins:  428
O wins:  345
  ties:  227
cargo run  78.63s user 0.16s system 99% cpu 1:18.83 total
```

### Set B

Using the Rust Nightly from last night, 2016-06-01.

#### Trial 1

```
X wins:  428
O wins:  345
  ties:  227
cargo run  80.85s user 0.19s system 99% cpu 1:21.17 total
```

#### Trial 2

```
X wins:  428
O wins:  345
  ties:  227
cargo run  77.22s user 0.07s system 99% cpu 1:17.35 total
```

#### Trial 3

```
X wins:  428
O wins:  345
  ties:  227
cargo run  81.43s user 0.25s system 99% cpu 1:22.10 total
```

### Set C

This is with reasonable (much better than the 'throwing darts' approach) random
move generation:

`let seed: &[_] = &[219, 9990002, 22004, 23];`


```
X wins:  393
O wins:  392
  ties:  215
cargo run  13.56s user 0.19s system 98% cpu 13.930 total
```

```
X wins:  393
O wins:  392
  ties:  215
cargo run  13.37s user 0.06s system 99% cpu 13.455 total
```

`let seed: &[_] = &[219, 9990003, 22004, 23];`


```
X wins:  417
O wins:  367
  ties:  216
cargo run  13.59s user 0.16s system 98% cpu 13.895 total
```

`let seed: &[_] = &[219, 9990004, 22004, 23];`

```
X wins:  414
O wins:  347
  ties:  239
cargo run  14.19s user 0.21s system 98% cpu 14.603 total
```
