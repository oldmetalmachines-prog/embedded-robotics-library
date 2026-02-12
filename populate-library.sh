#!/bin/bash

set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEMP_DIR="$SCRIPT_DIR/temp-clones"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}🚀 Populating Rust Embedded Library${NC}\n"
mkdir -p "$TEMP_DIR"

extract_project() {
    local repo_url=$1
    local target_dir=$2
    local source_path=$3
    local description=$4
    
    repo_name=$(basename "$repo_url" | sed 's/\.git$//')
    
    echo -e "${YELLOW}📦 $repo_name${NC}"
    
    if [ ! -d "$TEMP_DIR/$repo_name" ]; then
        echo "  Cloning..."
        git clone --depth 1 "$repo_url" "$TEMP_DIR/$repo_name" > /dev/null 2>&1
    fi
    
    mkdir -p "$SCRIPT_DIR/$target_dir"
    
    if [ -d "$TEMP_DIR/$repo_name/$source_path" ]; then
        cp -r "$TEMP_DIR/$repo_name/$source_path"/* "$SCRIPT_DIR/$target_dir/" 2>/dev/null || true
    fi
    
    cat > "$SCRIPT_DIR/$target_dir/SOURCE.md" << EOF
# Source: $repo_url
**Description:** $description
**Extracted:** $(date +%Y-%m-%d)
EOF

    echo -e "${GREEN}  ✓ Done${NC}"
}

echo -e "\n${BLUE}ESP32 Examples${NC}"

extract_project \
    "git@github.com:jamesmcm/esp32_wifi_tank.git" \
    "esp32/examples/wifi-tank" \
    "wifi_tank/src" \
    "WiFi controlled tank - motor control + networking"

extract_project \
    "git@github.com:jamesmcm/snake_rust_esp32.git" \
    "esp32/examples/snake-complete" \
    "src" \
    "Complete snake game with OLED + joystick"

extract_project \
    "git@github.com:bjoernQ/esp32-rust-nostd-temperature-logger.git" \
    "esp32/sensors/temperature-logger" \
    "src" \
    "MQTT temperature logger"

extract_project \
    "git@github.com:ivmarkov/rust-esp32-std-demo.git" \
    "esp32/examples/std-demo" \
    "src" \
    "WiFi, HTTP server, LED screen"

echo -e "\n${BLUE}Raspberry Pi${NC}"

extract_project \
    "git@github.com:golemparts/rppal.git" \
    "raspberry-pi/gpio/rppal-examples" \
    "examples" \
    "Official RPPAL GPIO examples"

echo -e "\n${BLUE}Reference Lists${NC}"

if [ ! -d "$TEMP_DIR/awesome-esp-rust" ]; then
    git clone --depth 1 git@github.com:esp-rs/awesome-esp-rust.git "$TEMP_DIR/awesome-esp-rust" > /dev/null 2>&1
    mkdir -p "$SCRIPT_DIR/docs/references"
    cp "$TEMP_DIR/awesome-esp-rust/README.md" "$SCRIPT_DIR/docs/references/awesome-esp-rust.md"
    echo -e "${GREEN}  ✓ Reference docs saved${NC}"
fi

cd "$SCRIPT_DIR"
git add .
git commit -m "Add curated ESP32 and RPi Rust examples

- ESP32: WiFi tank, snake game, temperature logger, std demo
- Raspberry Pi: RPPAL examples
- Reference documentation from awesome-esp-rust"

echo -e "\n${GREEN}✅ Complete! Run: git push${NC}"
