# FR
[![DeepSource](https://deepsource.io/gh/Laika/FR.svg/?label=active+issues&show_trend=true&token=iQ_HjqrehUx-1r5VuV9J4eVD)](https://deepsource.io/gh/Laika/FR/?ref=repository-badge)

Factorization Tool 

## Installation
```bash
git clone https://github.com/Laika/FR.git && cd FR/
cargo build --release
cargo install --path .
```

## Usage
1. Factorize 64bit integer by trial division.
```console
$ fr 13251537330083289031
n = 31 * 199 * 347 * 6190447136717
```

2. Factorize 256bit integer by Fermat's method.
```console
$ fr --algorithm=fermat 13407807929942597099574024998205846127479365820592393377723561443721764030142790646165789383030198876725227227082741501683806940107542205183165700530855221
n = 115792089237316195423570985008687907853269984665640564039457584007913129640233 * 115792089237316195423570985008687907853269984665640564039457584007913129640237
```
