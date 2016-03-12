#! /bin/bash
DIRECTORY_TO_OBSERVE="src"

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

function wait_for_change {
  inotifywait -r -q \
    -e modify,move,create,delete \
    $DIRECTORY_TO_OBSERVE
}

function build {
  echo -e "$GREEN *** Starting build ***$NC"
  cargo build
  if [ $? -eq 0 ]; then
    echo -e "$GREEN *** Build complete ***$NC"
  else
    echo -e "$RED *** Build failed ***$NC"
  fi
}

build

while wait_for_change; do
  sleep 0.25;
  build
  sleep 3;
done
