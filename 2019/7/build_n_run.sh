#!/bin/bash

PROG=amplification_circuit

if [ -f ${PROG} ]; then
    echo "Cleaning old build..."
    rm ${PROG}
fi

g++ -std=c++17 ${PROG}.cpp -o ${PROG}

if [ -f ${PROG} ]; then
  echo "Launching ${PROG}"
  ./${PROG}
fi
