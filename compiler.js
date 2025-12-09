const { spawn } = require('child_process');
const fs = require('fs');

const inputFile = process.argv[2];

if (!inputFile) {
  console.error('Usage: node compiler.js <input_file>');
  process.exit(1);
}

// Run the Rust compiler using cargo
// --quiet suppresses the build output
const rustProcess = spawn('cargo', ['run', '--quiet', inputFile], {
  stdio: ['inherit', 'pipe', 'inherit'] // Capture stdout, let stderr pass through
});

rustProcess.stdout.on('data', (data) => {
  // We ignore the Rust stdout because it prints "Compiling..." and "Successfully wrote..."
  // We only want the actual WAT content which is in the file.
});

rustProcess.on('close', (code) => {
  if (code === 0) {
    try {
      // Read the generated output.wat file
      const wat = fs.readFileSync('output.wat', 'utf8');
      // Print it to stdout so it can be piped
      process.stdout.write(wat);
    } catch (err) {
      console.error('Error reading output.wat:', err);
      process.exit(1);
    }
  } else {
    process.exit(code);
  }
});
