#!/bin/bash

set -e

BASEDIR=`dirname ${0}`
OUTDIR=${BASEDIR}/target/

mkdir -p ${OUTDIR}

FILEPATH=${1}
SRC="${FILEPATH##*/}"
BASE="${SRC%.rs}"

rustc ${SRC} --out-dir ${OUTDIR}
${OUTDIR}/${BASE}
