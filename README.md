# fluido-saturation

A mixer searching experiment using equality saturation, more specifically `egg`.

## Usage

Example code comes with a CLI:

```console
fluido-saturate --target-concentration 0.01 --input-space 0.04 --input-space 0 --time-limit 1
```

The command above will try to find output concentration of 0.01 using the provided input concentrations.

Output for the above example:

```console
> fluido-saturate --target-concentration 0.01 --input-space 0.04 --input-space 0 --time-limit 1

Starting to equality saturation, this will take ~60 seconds
Runner report
=============
  Stop reason: TimeLimit(60.058731875)
  Iterations: 655
  Egraph size: 813996 nodes, 1276 classes, 813996 memo
  Rebuilds: 0
  Total time: 60.064849359999975
    Search:  (0.08) 4.624326999
    Apply:   (0.90) 54.213128551000004
    Rebuild: (0.02) 1.2272947920000008

Optimized sequence: (mix 0 (mix 0 0.04))
Cost: 0.0
```

## Details

The saturation starts with a number, the target concentration, for the given example command above:

```console
(0.01)
```

A rewrite rule that expands a number is applied to the target concentation.

```rust
rw!("expand-num";
    "(?a)" => "(mix ?a ?a)"),
```

The expression becomes:

```console
(mix 0.01 0.01)
```

There is also a rewrite rule for searching different mixer input combinations that would give the same output concentration:

```rust
rw!("differentiate-mixer";
    "(mix ?a ?b)" => "(mix (- ?a 0.001) (+ ?b 0.001))"),
rw!("differentiate-mixer2";
    "(mix ?a ?b)" => "(mix (+ ?a 0.001) (- ?b 0.001))"),
```

So equality saturation is able to find:

```console
(mix 0.0, 0.02)
```

and saturation continues like this.


- One thing to note here is the `Concentration` representation used for the saturation. To be able to control the search space, a special representation for concentrations are used rather than a 'normal' `f64`. This allows the control over precision. Currently the `Concentration::EPSILON` = `0.0001`. Which means `0.00002` and `0.00004` are essentially the same concentation for the purpose of saturation. 


### Arithmetic Reasoning

TODO: Explain how arithmetic reasoning works in the current implementation.
