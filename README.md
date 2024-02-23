# fluido

A mixer searching and scheduling experiment using equality saturation, more specifically `egg`.

## Building 

This repo can be built with [cargo](https://doc.rust-lang.org/cargo/) or using the [nix flake](https://nixos.wiki/wiki/Flakes).

### Using Cargo

Install rust toolchain manager (rustup):

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Build with cargo:

```console
cargo build --release
```

### Using nix

This repo has a nix flake which can be used to either get a dev environment for working on this repoo or building the project.

To install nix and enable experimental flake feature:

```console
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

Assuming nix is installed, building the binary:

```console
nix build
```

Getting a dev environment to work on this repo:

```console
nix develop
```

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

`egg` is a saturation tool, which does not have a 'explicit' notion of arithmetics. To be able to do arithmetic reasoning, we defined a `MixerLang` with some primitive operations such as `Add (+)`, `Sub (-)`. Once a mixer expression become something like `(mix (- ?a 0.001) (+ ?b 0.001))` it is merged with the initial expression `(mix ?a ?b)` as they are equivalent (the former derived from the latter by the differentiate-mixer rewrite rule). Once this merge operation happens, our analysis implementation is called. In which we evaluate the expression and add a `Num` node equivalent to this node representing the arithmetic operation. This way the results of arithmetic operations can be found and added into the egraph.


### MixLang

MixLang is the intermediate language defined for representing mixer graphs and operations. Currently it consists of following constructs:

| Name         | Operand count  | Operand type(s)| Explanation                                                            | Symbolic 
|--------------|----------------|----------------|------------------------------------------------------------------------|---------------------|
| Num          | 1              | Constant       |  A consntant number, representing a specific concentration             | elem1               |
| Add          | 2              | ID, ID         |  Addition of two e-nodes that are num (or equivalent to a num)         | elem1 + elem2       |
| Sub          | 2              | ID, ID         |  Substraction of two e-nodes that are num (or equivalent to a num)     | elem1 - elem2       |
| Mix          | 2              | ID, ID         |  Mixing of two e-nodes that are num (or equivalent to a num)           | (elem1 + elem2) / 2 |
