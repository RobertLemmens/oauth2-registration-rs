#!/bin/bash
docker build -t registry.gitlab.com/notes26/notes-api/registration-server:0.2.0 .
docker push registry.gitlab.com/notes26/notes-api/registration-server:0.2.0
