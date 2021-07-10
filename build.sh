#!/usr/bin/env bash

mkdir -p $HOME/.sync-that-thing-please/bin

if [ -d "$HOME/.sync-that-thing-please/source" ] 
then
  git --git-dir $HOME/.sync-that-thing-please/source/.git pull origin master 
else
  git clone git@github.com:hdf1986/sync-that-thing-please.git $HOME/.sync-that-thing-please/source --depth=1
fi

cd $HOME/.sync-that-thing-please/source

cargo build --release

cp $HOME/.sync-that-thing-please/source/target/release/sync-that-thing-please $HOME/.sync-that-thing-please/bin

$HOME/.sync-that-thing-please/bin/sync-that-thing-please