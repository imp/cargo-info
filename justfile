build:
    cargo build --workspace --all-targets
static:
    cargo build --target x86_64-unknown-linux-musl
clean:
    cargo clean
test:
    cargo test --workspace -- --report-time -Z unstable-options
clippy:
    cargo clippy --workspace --all-targets
c:
    cargo c
pedantic:
    cargo clippy --workspace --all-targets --features pedantic
update:
    cargo update
bloat:
    cargo bloat
cbuild: clean build
ctest: clean test
rustfmt:
    cargo fmt --all -- --check
alias fmt := rustfmt
check: rustfmt update test clippy
fixlock:
    rm Cargo.lock
    cargo update
    git add Cargo.lock

branch := "develop"
merge_request:
    git push -o merge_request.create -o merge_request.target={{branch}} -o merge_request.merge_when_pipeline_succeeds
alias mr := merge_request
release:
    cargo release --no-confirm patch
