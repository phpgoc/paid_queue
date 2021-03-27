#!/bin/bash
pushd $(dirname $0) >/dev/null
cmddir=$(pwd)
popd >/dev/null

docker build -t microservice_rust_actix $cmddir/../microservice_rust_actix/
