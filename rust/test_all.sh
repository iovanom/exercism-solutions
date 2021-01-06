#!/bin/bash

echo "==== Start testing all exercises ===="

for d in */ ; do
  cd $d
  
  if cargo test > /dev/null 2>&1;
  then
    status="\e[32mOK\e[0m"
  else
    status="\e[31mFAIL\e[0m"
  fi

  echo -e "$d ===> \e[1m$status\e[0m"
  cd ..
done
