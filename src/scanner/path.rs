// Dependencies

use std::path::PathBuf;
use simple_error::{SimpleError, bail};

// Constant values

const ERR_PATH_NOT_FOUND: &str = r"The provided path does not exist.";
const ERR_PATH_INVALID: &str = r"The provided path is invalid.";

const GAME_DIR: &str = r"game";
const GAME_EXE: &str = r"ffxiv_dx11.exe";

// GameDirectory

#[derive(Default)]
pub struct GameDirectory {
	path_buf: PathBuf
}

impl GameDirectory {
	pub fn at(path_str: &str) -> Result<Self, SimpleError> {
		let mut path_buf = PathBuf::from(path_str);
		if !path_buf.as_path().exists() {
			bail!(ERR_PATH_NOT_FOUND);
		}

		// Truncates path down to the game's base directory.
		// Returns an error if the base directory is not found.
		while match path_buf.file_name().ok_or(ERR_PATH_INVALID)?.to_str() {
			Some("ffxiv_dx11.exe" | "game" | "boot") => true,
			Some(_) => !path_buf.as_path().join(GAME_DIR).join(GAME_EXE).exists(),
			None => bail!(ERR_PATH_INVALID)
		} { path_buf.pop(); }

		Ok(GameDirectory { path_buf })
	}

	pub fn get_exe_path(&self) -> PathBuf {
		self.path_buf.join(GAME_DIR).join(GAME_EXE)
	}
}