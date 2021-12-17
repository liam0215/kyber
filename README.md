# Kyber 

[![Build Status](https://travis-ci.org/pq-crystals/kyber.svg?branch=master)](https://travis-ci.org/pq-crystals/kyber) [![Coverage Status](https://coveralls.io/repos/github/pq-crystals/kyber/badge.svg?branch=master)](https://coveralls.io/github/pq-crystals/kyber?branch=master)

This directory contains our implementation of [Kyber](https://eprint.iacr.org/2017/634). Both the reference code and the AVX2 optimized code are in the directories ref/ and avx2/, respectively.

It also contains an unsafe rust wrapper library, `libkyber-sys`. 
Bindings were generated using `bindgen` and the following command:
```bindgen ref/api.h -o src/bindings.rs --allowlist-var "KYBER_.*" --allowlist-function "crypto_kem.*"```

