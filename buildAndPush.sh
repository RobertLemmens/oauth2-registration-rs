#!/bin/bash
docker build -t registry.gitlab.com/notes26/notes-api/registration-server .
docker push registry.gitlab.com/notes26/notes-api/registration-server
