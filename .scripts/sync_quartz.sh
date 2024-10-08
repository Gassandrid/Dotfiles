#!/bin/zsh

function syncQuartz {
  # cd into the VAULT directory and run the link.sh script to copy files to the quartz
  cd /Users/gassandrid/VAULT/
  # run the link.sh script
  ./link.sh
  # cd into the quartz directory and run npx quartz-sync to sync files from the VAULT to the quartz
  cd /Users/gassandrid/cs/quartz/
  npx quartz sync
}


