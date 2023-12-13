#!/bin/sh
mkdir -p ~/.config/clrgen/templates
mkdir -p ~/.config/clrgen/colors
mkdir -p ~/.clrgen
cp -rf templates/* ~/.config/clrgen/templates/
cp -rf colors/* ~/.config/clrgen/colors/
sudo cp clrgen /usr/local/bin/