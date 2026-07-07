#!/bin/bash

docker build -t test_maria .
docker run -d --name test_maria -p 3306:3306 test_maria