FROM public.ecr.aws/p6e8q1z1/aleph-node:latest

COPY ./tests/docker/docker_entrypoint.sh /node/docker_initial_entrypoint.sh

ENTRYPOINT ["./docker_initial_entrypoint.sh"]
