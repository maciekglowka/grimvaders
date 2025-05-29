set -e 

SHEETS=(
    "units"
)

for SHEET in "${SHEETS[@]}"; do
    echo "Generating sprite sheet: $SHEET"
    aseprite -b --sheet $(pwd)/assets/sprites/$SHEET.png --sheet-type rows --sheet-width 1024 --data $(pwd)/assets/sprites/$SHEET.json --list-tags $(pwd)/assets_dev/sprites/$SHEET.aseprite
done
