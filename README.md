# markdown-benchmarks

Testing performance of the popular markdown libraries:

* [blackfriday](https://github.com/russross/blackfriday)
* [comrak](https://github.com/kivikakk/comrak)
* [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)
* [cmark](https://github.com/commonmark/cmark)
* [md4c](https://github.com/mity/md4c)

Running benchmarks:

```bash
git clone git@github.com:commonmark/cmark.git
mkdir -p cmark/build
cd cmark/build
cmake ..
make
cd ../..

git clone git@github.com:hoedown/hoedown.git
cd hoedown
make
cd ..

git clone git@github.com:mity/md4c.git

git clone git@github.com:1Password/markdown-benchmarks.git
go get -u gopkg.in/russross/blackfriday.v2

cd markdown-benchmarks
make build
make run -s
```

2026 September results on GitHub Actions (ubuntu-latest), pulldown-cmark 0.13.4 & comrak 0.54.0

Produced by CI — see [`.github/workflows/benchmark.yml`](.github/workflows/benchmark.yml)
([Actions runs](../../actions/workflows/benchmark.yml)). Rust parsers bumped to current
releases; C parsers, methodology and `sample1.md` unchanged.

```bash
$ make run -s
Blackfriday (Go):
  1000 iterations = 0.044s
 10000 iterations = 0.409s
100000 iterations = 4.151s

Comrak (Rust):
  1000 iterations =   0.055s
 10000 iterations =   0.545s
100000 iterations =   5.463s

Pulldown-cmark (Rust):
  1000 iterations =   0.019s
 10000 iterations =   0.198s
100000 iterations =   2.000s

Cmark (C):
  1000 iterations =   0.040s
 10000 iterations =   0.403s
100000 iterations =   4.011s

Hoedown (C):
  1000 iterations =   0.020s
 10000 iterations =   0.204s
100000 iterations =   2.028s

MD4C (C) with empty callbacks:
  1000 iterations =   0.012s
 10000 iterations =   0.128s
100000 iterations =   1.291s
```

2020 August results on Intel Core i7 6700 @ 4.0GHz

```bash
$ make run -s
Blackfriday (Go):
  1000 iterations = 0.081s
 10000 iterations = 0.758s
100000 iterations = 7.419s

Comrak (Rust):
  1000 iterations =   0.113s
 10000 iterations =   1.115s
100000 iterations =  11.113s

Pulldown-cmark (Rust):
  1000 iterations =   0.021s
 10000 iterations =   0.211s
100000 iterations =   2.179s

Cmark (C):
  1000 iterations =   0.046s
 10000 iterations =   0.458s
100000 iterations =   4.653s

Hoedown (C):
  1000 iterations =   0.022s
 10000 iterations =   0.227s
100000 iterations =   2.238s

MD4C (C) with empty callbacks:
  1000 iterations =   0.012s
 10000 iterations =   0.118s
100000 iterations =   1.174s
```

2019 March results on 2018 Mac Book Pro (2.9 GHz Intel Core i9)

```bash
$ make run -s
Blackfriday (Go):
  1000 iterations = 0.054s
 10000 iterations = 0.514s
100000 iterations = 5.121s

Comrak (Rust):
  1000 iterations =   0.264s
 10000 iterations =   2.542s
100000 iterations =  26.623s

Pulldown-cmark (Rust):
  1000 iterations =   0.044s
 10000 iterations =   0.373s
100000 iterations =   3.785s

Cmark (C):
  1000 iterations =   0.080s
 10000 iterations =   0.798s
100000 iterations =   7.788s

Hoedown (C):
  1000 iterations =   0.023s
 10000 iterations =   0.228s
100000 iterations =   2.303s

MD4C (C) with empty callbacks:
  1000 iterations =   0.010s
 10000 iterations =   0.112s
100000 iterations =   1.088s
```


