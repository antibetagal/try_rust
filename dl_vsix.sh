#!/bin/bash

pkg_first=()
pkg_second=()
pkg_version=()
pkg_arch=()

pkg_first+=("ms-vscode")
pkg_second+=("python")
pkg_version+=("2024.17.2024100")
pkg_arch+=("linux-x64")

pkg_first+=("continue")
pkg_second+=("continue")
pkg_version+=("1.0.11")
pkg_arch+=("linux-x64")



# Save location for the VSIX files
save_location="."

for ((i = 0; i < ${#pkg_first[@]}; i++)); do
  url="https://marketplace.visualstudio.com/_apis/public/gallery/publishers/${pkg_first[$i]}/vsextensions/${pkg_second[$i]}/${pkg_version[$i]}/vspackage?targetPlatform=${pkg_arch[$i]}"

  filename=${pkg_first[$i]}.${pkg_second[$i]}-${pkg_version[$i]}-${pkg_arch[$i]}.vsix
  
  echo "Downloading $url to ${save_location}/${filename}"
  curl -o "${save_location}/${filename}" "$url"
done