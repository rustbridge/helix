#!/bin/bash

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
source $HOME/.cargo/env

\curl -sSL https://get.rvm.io | bash -s stable --ruby
source $HOME/.rvm/scripts/rvm

gem install bundler
