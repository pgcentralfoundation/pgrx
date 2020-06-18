#! /bin/bash
# Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
# governed by the MIT license that can be found in the LICENSE file.

wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
sudo apt-add-repository "deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main"
sudo add-apt-repository ppa:git-core/ppa -y

sudo apt-get update
sudo apt-get install -y clang-6.0
sudo apt-get install -y git


mkdir actions-runner && cd actions-runner
curl -O https://githubassets.azureedge.net/runners/2.163.1/actions-runner-linux-x64-2.163.1.tar.gz
tar xzf ./actions-runner-linux-x64-2.163.1.tar.gz
./config.sh --url https://github.com/zombodb/pgx --token ADF7VSADUDFWKXEN76M4RXC6DLO6S 
./run.sh
