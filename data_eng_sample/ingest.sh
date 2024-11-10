#!/bin/sh

ROOT=$(pwd)
SOURCES="${ROOT}/sources"
DOCS="${ROOT}/docs"

# 1. Create FHS
if [[ ! -d "$SOURCES" ]]; then
  mkdir $SOURCES
fi

if [[ ! -d "$DOCS" ]]; then
  mkdir $DOCS
fi

# 2. Get sources
cd $SOURCES
for i in {19..23}; do curl -fvL "https://www2.census.gov/programs-surveys/cps/datasets/20${i}/supp/dec${i}pub.csv" | xz -z -v - > "dec${i}pub.xz"; done

# 3. Get latest technical supplement
cd $DOCS
wget --no-clobber "https://www2.census.gov/programs-surveys/cps/techdocs/cpsdec23.pdf"

# 4. File integrity and non-repudiation go here (if possible)
