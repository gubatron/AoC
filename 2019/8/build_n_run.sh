#!/bin/bash

PROG=space_image_format

if [ -f ${PROG} ]; then
    echo "Cleaning old build..."
    rm ${PROG}
fi

g++ -std=c++17 -g ${PROG}.cpp -o ${PROG}

if [ -f ${PROG} ]; then
  echo "Launching ${PROG}"
  ./${PROG}
fi
