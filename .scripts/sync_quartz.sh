#!/bin/zsh

function syncQuartz {
  # save current directory to go back to it after the script is done
  dir=$(pwd)
  

  # cd into the VAULT directory and run the link.sh script to copy files to the quartz
  cd /Users/gassandrid/VAULT/
  # run the link.sh script
  ./link.sh
  # cd into the quartz directory and run npx quartz-sync to sync files from the VAULT to the quartz
  cd /Users/gassandrid/cs/quartz/
  npx quartz sync

  cd /Users/gassandrid/cs/markdown/stats-notes/

  # remove all files except .git and .gitignore
  # save the .git and .gitignore files in a variable buffer and remove all files
  buffer=$(ls -A | grep -v .git | grep -v .gitignore)
  # remove all files except .git and .gitignore
  rm -rf $buffer

  cp -R /users/gassandrid/vault/notes/statistics/ /users/gassandrid/cs/markdown/stats-notes

  # cd back to the original directory
  cd $dir
}

