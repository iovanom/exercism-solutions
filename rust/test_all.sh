#!/bin/bash

echo "==== Start testing all exercises ===="

for d in */ ; do
  cd $d
  
  if cargo test > /dev/null 2>&1;
  then
    status='OK'
  else
    status='FAIL'
  fi

  echo "$d ===> $status"
  cd ..
done
