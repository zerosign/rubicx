from alpine:edge
label maintainer "zerosign <r1nlx0@gmail.com>"

copy target/release/kvs /usr/bin

entrypoint ["/usr/bin/kvs", "--path=/mnt/cephfs/kvs"]
