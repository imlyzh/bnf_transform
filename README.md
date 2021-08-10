# bnf_transform

Let BNF file transfrom to pest.

## who compilation it?

1. install rust toolchain
2. run `cargo build --release`
3. from `target` dir copy `bnf_transform` to you program dir
4. extend set env variable `path`

## who use?

```sh
bnf_transform <bnf-style> <input.bnf> <output.pest>
```

## example

```sh
bnf_transform --llir llir.bnf llir.pest
```
