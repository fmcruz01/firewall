build:
    cargo build
    cargo install --path . --bin rscan --force

test: build
    cargo test

cov:
    LLVM_COV="$(rustup run stable rustc --print target-libdir)/../bin/llvm-cov" \
    LLVM_PROFDATA="$(rustup run stable rustc --print target-libdir)/../bin/llvm-profdata" \
    cargo +stable llvm-cov --workspace

cov-check min="90":
    ./scripts/cov-check.sh {{min}}
