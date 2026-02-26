#!/usr/bin/env node

/**
 * Install script for dprint-plugin-asciidoc
 * Downloads the WASM plugin from the GitHub release
 */

const fs = require('fs');
const path = require('path');
const https = require('https');

const WASM_FILENAME = 'dprint_plugin_asciidoc.wasm';
const PACKAGE_ROOT = path.join(__dirname);
const TARGET_PATH = path.join(PACKAGE_ROOT, WASM_FILENAME);

// Get version from package.json
const packageJson = require('./package.json');
const VERSION = packageJson.version;

const DOWNLOAD_URL = `https://github.com/devcntr-app/dprint-plugin-asciidoc/releases/download/v${VERSION}/dprint-plugin-asciidoc-${VERSION}.wasm`;

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading WASM plugin from ${url}...`);

    const file = fs.createWriteStream(dest);

    https.get(url, (response) => {
      if (response.statusCode === 301 || response.statusCode === 302) {
        // Follow redirects
        file.close();
        fs.unlinkSync(dest);
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }

      if (response.statusCode !== 200) {
        reject(new Error(`Download failed with status code ${response.statusCode}`));
        return;
      }

      response.pipe(file);

      file.on('finish', () => {
        file.close();
        console.log(`WASM plugin downloaded successfully to ${dest}`);
        resolve();
      });
    }).on('error', (err) => {
      fs.unlinkSync(dest);
      reject(err);
    });
  });
}

async function main() {
  // Check if WASM file already exists (e.g., from manual install)
  if (fs.existsSync(TARGET_PATH)) {
    console.log('WASM plugin already exists, skipping download.');
    process.exit(0);
  }

  try {
    await downloadFile(DOWNLOAD_URL, TARGET_PATH);
  } catch (error) {
    console.error('Failed to download WASM plugin:', error.message);
    console.error('');
    console.error('You can manually download the plugin from:');
    console.error(`  https://github.com/devcntr-app/dprint-plugin-asciidoc/releases/tag/v${VERSION}`);
    console.error('');
    console.error('And place it at:');
    console.error(`  ${TARGET_PATH}`);
    process.exit(1);
  }
}

main();
