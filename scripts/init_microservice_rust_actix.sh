#!/bin/bash
pushd $(dirname $0) >/dev/null
cmddir=$(pwd)
popd >/dev/null

http_port=80
https_port=443
container=microservice_rust_actix
while getopts "h::" arg
do
    case $arg in
        h)
            http_port=$OPTARG
            ;;
        ?)
            echo "未知选项"
            exit 1
            ;;
    esac
done

docker create \
--name $container \
--restart=always \
--link redis \
-p $http_port:5000 microservice_rust_actix && \

docker start $container && \

echo "started $container"

