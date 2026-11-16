//! Build script invoking Tauri code generation.

use std::path;
use std::process;

fn main() {
  let dist_index = path::Path::new("ui/dist/index.html");
  if !dist_index.exists() {
    let npm_cmd = if cfg!(windows) { "npm.cmd" } else { "npm" };
    let _ = process::Command::new(npm_cmd)
      .current_dir("ui")
      .args(["run", "build"])
      .status();
  }
  tauri_build::build();
}
