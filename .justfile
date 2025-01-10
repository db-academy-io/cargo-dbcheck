alias d := docker
alias dp := docker-publish
alias p := pre_commit

# Build docker file
docker:
    docker build -t dbacademyio/dbcheck .

# Publish docker file
docker-publish:
    docker build -t dbacademyio/dbcheck .
    docker push dbacademyio/dbcheck

pre_commit:
    pre-commit run --all-files
