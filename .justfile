alias d := docker
alias db := docker-build
alias dp := docker-publish
alias p := pre_commit
alias cp := cargo-publish
alias lp := local-publish

# Build docker file
docker:
    docker build -t dbacademyio/dbcheck .

# Build docker file
docker-build:
    docker build -t dbacademyio/dbcheck .

# Publish docker file
docker-publish:
    docker build -t dbacademyio/dbcheck .
    docker push dbacademyio/dbcheck

# Build and release to crates.io
cargo-publish:
    cargo-version-upgrade patch
    cargo publish

# Build and release to crates.io
local-publish:
    cargo build --release
    cargo install --path .

pre_commit:
    pre-commit run --all-files
