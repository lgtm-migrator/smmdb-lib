#!/bin/bash

touch ~/.npmrc
echo "//registry.npmjs.org/:_authToken=$NPM_TOKEN" >> ~/.npmrc
wasm-pack build
wasm-pack pack
wasm-pack publish
npm dist-tag add cemu-smm@$(npx -c 'echo "$npm_package_version"') next