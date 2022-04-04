#!/usr/bin/env bash

set -e

project_name="vested-dxd-calculator"
download_folder="${HOME}/.${project_name}"

get_arch() {
    a=$(uname -m)
    case ${a} in
        "x86_64" )
            echo "x86_64"
        ;;
        "aarch64" )
            echo "aarch64"
        ;;
        *)
            echo ${NIL}
        ;;
    esac
}

get_os() {
    echo $(uname -s | awk '{print tolower($0)}')
}

get_latest_version() {
    echo $(curl -sL https://api.github.com/repos/luzzif/${project_name}/releases/latest | grep -o '"tag_name": "[^"]*' | grep -o '[^"]*$')
}

version=$(get_latest_version)
binary_name="$(get_arch)-$(get_os)-$version"
binary_location="${download_folder}/${project_name}"

echo "[1/3] Downloading binary from https://github.com/luzzif/$project_name/releases/download/$version/$binary_name to $download_folder..."
rm -rf ${download_folder}
mkdir -p ${download_folder}
curl --fail --location --output "$download_folder/$binary_name" "https://github.com/luzzif/$project_name/releases/download/$version/$binary_name"
chmod +x ${download_folder}/${binary_name}

echo "[2/3] Installing ${project_name}"
sudo cp ${download_folder}/${binary_name} /usr/local/bin/${project_name}

echo "[3/3] Cleaning up"
rm -rf ${download_folder}

echo "${project_name} installed successfully"
echo "Run '$project_name --help' to get started"

exit 0