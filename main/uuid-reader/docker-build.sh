#!/bin/bash

if [ "$#" -ne 1 ]
then
  echo "Don't forget args"
  exit 1
fi

docker build -t ananaa/uuid-reader:"$1" .
docker tag ananaa/uuid-reader:"$1" ananaa/uuid-reader:latest
docker push ananaa/uuid-reader:latest
docker push ananaa/uuid-reader:"$1"