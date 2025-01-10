alias d := docker
alias dp := docker-publish


# Build docker file
docker:
    docker build -t dbacademyio/dbcheck .

# Publish docker file
docker-publish:
    docker build -t dbacademyio/dbcheck .
    docker push dbacademyio/dbcheck
