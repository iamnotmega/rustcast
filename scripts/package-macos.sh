#!/usr/bin/env -S bash -e

APP_BUNDLE_PATH="${APP_BUNDLE_PATH:?APP_BUNDLE_PATH not set}"
DMG_NAME="${DMG_NAME:?DMG_NAME not set}"
DMG_DIR="${DMG_DIR:?DMG_DIR not set}"

VOLUME_NAME="Rustcast"   # visible volume name
STAGING_DIR="$DMG_DIR/dmg-staging"

rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR"

# Copy app and Applications symlink into staging dir
cp -R "$APP_BUNDLE_PATH" "$STAGING_DIR/"
ln -s /Applications "$STAGING_DIR/Applications"

# Remove old DMG
rm -f "$DMG_DIR/$DMG_NAME"

# Create DMG
hdiutil create -volname "$VOLUME_NAME" \
  -srcfolder "$STAGING_DIR" \
  -ov -format UDZO \
  "$DMG_DIR/$DMG_NAME"

echo "Created DMG at $DMG_DIR/$DMG_NAME"
echo "DMG_PATH=$DMG_DIR/$DMG_NAME" >> "$GITHUB_ENV"
