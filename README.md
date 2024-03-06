# Address handling in CosmWasm stack

## Introduction

* Generally, we are agnostic when it comes to addresses. But blockchain users usually don't.
* The most popular format of representing user address and/or contract address
  is [Bech32](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki).
* There is an enhanced version of this format,
  named [Bech32m](https://github.com/bitcoin/bips/blob/master/bip-0350.mediawiki).
* In prehistoric times (before version 2.0) of `cosmwasm-std` we have used custom internal address format, now we are
  aligned with the format mostly used in real-life blockchains.

## Address representations

### Human-readable

Address has a form of text that is quite easy to remember for most human beings, like this:

```
cosmwasm1h34lmpywh4upnjdg90cjf4j70aee6z8qqfspugamjp42e4q28kqs8s7vcp
```

`cosmwasm` is a human-readable prefix (HRP) then the separator `1` and the rest (easy to remember 😊).

Usually every chain has its own, unique HRP like `juno`, `osmo` and s.o.

### Canonical

```

BC6BFD848EBD7819C9A82BF124D65E7F739D08E002601E23BB906AACD40A3D81

[188,107,253,132,142,189,120,25,201,168,43,241,36,214,94,127,115,157,8,224,2,96,30,35,187,144,106,172,212,10,61,129]

```

Shorter form of the same address.
