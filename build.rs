fn main() {
    let src = [
        "ref/randombytes.c",
        "ref/poly.c",
        "ref/polyvec.c",
        "ref/reduce.c",
        "ref/cbd.c",
        "ref/ntt.c",
        "ref/verify.c",
        "ref/indcpa.c",
        "ref/kex.c",
        "ref/fips202.c",
        "ref/kem.c",
    ];
    cc::Build::new()
        .define("KYBER_K", "3")
        .files(src.iter())
        .include("ref/")
        .flag("-g")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-O3")
        .flag("-fomit-frame-pointer")
        .flag("-fPIC")
        .static_flag(true)
        .compile("kyber");
}
