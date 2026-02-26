#!/bin/bash

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}════════════════════════════════════════════════${NC}"
echo -e "${GREEN}   Building dockless for all platforms${NC}"
echo -e "${BLUE}════════════════════════════════════════════════${NC}\n"

rm -rf release
mkdir -p release

CARGO_TOML="Cargo.toml"

get_version() {
    grep '^version = ' "$CARGO_TOML" | head -n1 | sed 's/version = "\(.*\)"/\1/'
}
CURRENT_VERSION=$(get_version)

echo -e "${YELLOW}Building Docker image with all platforms...${NC}"
docker build -f Dockerfile --build-arg TARGET=x86_64-unknown-linux-musl --build-arg VERSION=${CURRENT_VERSION} -t dockless-builder:latest . || {
    echo -e "${RED} Build failed${NC}"
    exit 1
}



echo -e "${YELLOW}Extracting binaries...${NC}"
CONTAINER_ID=$(docker create dockless-builder:latest /bin/sh)

echo -e "${YELLOW}Container created...${NC}"
docker cp ${CONTAINER_ID}:/ - | tar -xf - -C release/ 2>/dev/null || {
    docker cp ${CONTAINER_ID}:/dockless-linux-amd64 release/
    docker cp ${CONTAINER_ID}:/dockless-linux-arm64 release/
    docker cp ${CONTAINER_ID}:/dockless-linux-armv7 release/
}

docker rm ${CONTAINER_ID} >/dev/null

chmod +x release/dockless-* 2>/dev/null || true

echo -e "\n${BLUE}════════════════════════════════════════════════${NC}"
echo -e "${GREEN}   Build complete!${NC}"
echo -e "${BLUE}════════════════════════════════════════════════${NC}\n"

echo -e "${YELLOW}Built binaries:${NC}"
ls -lh release/

echo -e "\n${YELLOW}Binary details:${NC}"
for binary in release/*; do
    if [ -f "$binary" ]; then
        echo -e "\n${GREEN}$(basename $binary):${NC}"
        file "$binary" 2>/dev/null || echo "  Binary: $(basename $binary)"
        if [ -x "$binary" ] && [[ "$binary" != *.exe ]]; then
            size=$(du -h "$binary" | cut -f1)
            echo "  Size: $size"
        fi
    fi
done

echo -e "\n${GREEN}✓ All builds successful!${NC}"
echo -e "${BLUE}Release binaries are in: ./release/${NC}\n"