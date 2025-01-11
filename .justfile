alias d := docker
alias dp := docker-publish
alias p := pre_commit
alias cp := cargo-publish

# Build docker file
docker:
    docker build -t dbacademyio/dbcheck .

# Publish docker file
docker-publish:
    docker build -t dbacademyio/dbcheck .
    docker push dbacademyio/dbcheck

# Build and release to crates.io
cargo-publish:
    cargo version-upgrade
    cargo publish

pre_commit:
    pre-commit run --all-files
