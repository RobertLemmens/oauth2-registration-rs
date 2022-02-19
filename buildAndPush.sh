#!/bin/bash
if [[ $1 == 'dev' ]]
then
echo "Building dev image"
docker build -t registry.gitlab.com/notes26/notes-api/registration-server:dev .
docker push registry.gitlab.com/notes26/notes-api/registration-server:dev
else
echo "Building image"
docker build -t registry.gitlab.com/notes26/notes-api/registration-server:0.2.0 .
docker push registry.gitlab.com/notes26/notes-api/registration-server:0.2.0
fi