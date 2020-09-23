#!/bin/bash

if [ "$#" -ne 1 ]
then
  echo "Don't forget args"
  exit 1
fi

docker build -t ananaa/uuid-writer:"$1" .
docker tag ananaa/uuid-writer:"$1" ananaa/uuid-writer:latest
docker push ananaa/uuid-writer:latest
docker push ananaa/uuid-writer:"$1"