#!/bin/zsh

wsg="hi"                    # this is not an evnironment variable, but a shell variable, somewhat like an alias
export KEY="353mississippi" # this is an environment variable

# function export exmaple

function say_hello() {
  echo "Hello"
}

function say_goodbye() {
  echo "Goodbye"
}

source ~/.dotfiles/.scripts/second.sh

# source quartz syncer
source ~/.dotfiles/.scripts/sync_quartz.sh
