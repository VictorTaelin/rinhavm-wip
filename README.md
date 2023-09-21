RINHA on HVM
============

rinha

> rinha

Rinha
-----

usage:

```
rinhavm example.json # limited to λ-terms
```

result:

```
# Rinha Input:
fn (f) => { fn (x) => { f(f(x)) } }(fn (b) => { fn (t) => { fn (f) => { b(f)(t) } } })(fn (t) => { fn (f) => { t } })

# HVM Net:
$ b
& (0 (0 (0 c (0 d e)) (0 d (0 c e))) (0 (0 f (0 * f)) b))
~ (0 (1 (0 g h) (0 i g)) (0 i h))

# Rinha Output:
fn (a) => { fn (_) => { a } }

# HVM Stats:
- rwts: 16
- dref: 0
```

## Rinha

rinha | rinha | rinha
----- | ----- | -----
rinha | rinha | rinha
rinha | rinha | rinha
rinha | rinha | rinha
rinha | rinha | rinha
rinha | rinha | rinha
rinha | rinha | não

### DISCLAIMER

rinha
