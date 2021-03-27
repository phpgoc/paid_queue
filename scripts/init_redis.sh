#!/bin/bash

pushd `dirname $0` >/dev/null
cmddir=`pwd`
popd >/dev/null

outport=
container=redis

while getopts "o:" arg
do
    case $arg in
        o)
            outport=$OPTARG
            outport="-p "${outport}":6379"
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
$outport \
-v /usr/share/zoneinfo/Asia/Shanghai:/etc/localtime:ro \
-v $cmddir/../runtime/redis  \
redis:6.2.1 redis-server /usr/local/etc/redis.conf && \


docker cp $cmddir/../config/redis.conf $container:/usr/local/etc/


docker start $container && \

echo "started $container"
