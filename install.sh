#!/bin/sh
mkdir -p ~/.clrgen/templates
mkdir -p ~/.clrgen/colors
cp -rf templates/* ~/.clrgen/templates/
cp -rf example_data/* ~/.clrgen/colors/
sudo cp clrgen /usr/local/bin/