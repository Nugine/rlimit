#!/bin/bash
OUTEXE=/tmp/rlimit_codegen
OUTRS=$1
if [ -z "$OUTRS" ]; then 
    echo "missing OUTRS argument, please specify a file path"
    exit 1
fi
echo "try CODEGEN64"
g++ ./scripts/codegen.cpp -DCODEGEN64 -std=c++11 -o $OUTEXE
if [ $? -ne 0 ]; then
    echo "try fallback"
    g++ ./scripts/codegen.cpp -std=c++11 -o $OUTEXE
fi
if [ $? -ne 0 ]; then
    echo "codegen failed"
    exit 1
fi
$OUTEXE > $OUTRS
echo "done"
