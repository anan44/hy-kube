#!/bin/bash

if [ "$#" -ne 1 ]
then
  echo "Don't forget args"
  exit 1
fi

docker build -t ananaa/pingpong:"$1" .
docker tag ananaa/pingpong:"$1" ananaa/pingpong:latest
docker push ananaa/pingpong:"$1"
docker push ananaa/pingpong:latest
