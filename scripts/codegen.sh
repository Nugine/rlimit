#!/bin/bash -x
OUTEXE=/tmp/rlimit_codegen
OUTRS=$1
if [ -z "$OUTRS" ]; then 
    echo "missing OUTRS argument"
    exit 1
fi
g++ ./scripts/codegen.cpp -DCODEGEN64 -std=c++11 -o $OUTEXE
if [ $? -ne 0 ]; then
    g++ ./scripts/codegen.cpp -std=c++11 -o $OUTEXE
fi
$OUTEXE > $OUTRS
echo "done"
