#!/bin/bash

if [ "$#" -ne 1 ]
then
  echo "Don't forget args"
  exit 1
fi

docker build -t ananaa/crudster:"$1" .
docker tag ananaa/crudster:"$1" ananaa/crudster:latest
docker push ananaa/crudster:"$1"
docker push ananaa/crudster:latest
