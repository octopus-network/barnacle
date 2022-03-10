#!/bin/bash

echo "Current repo substrate version"
current_dependencies="$(awk '/build-dependencies/{print;getline;print}' runtime/Cargo.toml)"
echo $current_dependencies

echo "Barnacle repo substrate version"
barnacle_dependencies="$(curl -s https://raw.githubusercontent.com/octopus-network/barnacle/master/runtime/Cargo.toml | \
  awk '/build-dependencies/{print;getline;print}')"
echo $barnacle_dependencies

if [ "$current_dependencies" == "$barnacle_dependencies" ]; then
    echo "Success, they are same."
else
    echo "Failed, please check, the current substrate dependencies is different from barnacle."
    exit 125
fi
