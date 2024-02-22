#! /bin/bash

protoc --prost_out=../ -I ./ CMD.proto  Games.proto  Common.proto  Message.proto
# protoc --prost_out=../ -I ./ Games.proto
# protoc --prost_out=../ -I ./ Common.proto 
# protoc --prost_out=../ -I ./ Message.proto
