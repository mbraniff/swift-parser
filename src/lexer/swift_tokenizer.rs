use core::str;
use std::{collections::HashMap, fs::{File, OpenOptions}, io::{self, Read, SeekFrom, Seek, Write}, path::Path};
use sha2::{Digest, Sha256};
use base64::engine::{general_purpose, Engine as _};

use super::{token::Token, tokenizer};

// Tokenizer that can be configured to cache tokens for files based on file hashes
pub struct Tokenizer {
    use_cache: bool,
    cache_file: Option<File>,
    token_map: HashMap<String, String>,
    hash_map: HashMap<String, String>,
}

impl Tokenizer {
    fn new(use_cache: bool, cache_path: Option<&Path>) -> Tokenizer {
        let mut file: Option<File> = None;
        let mut token_map = HashMap::new();
        let mut hash_map = HashMap::new();
        if let Some(path) = cache_path {
            if let Some(mut read) = File::open(path).ok() {
                let mut content = String::new();
                read.read_to_string(&mut content).unwrap();
                (token_map, hash_map) = Tokenizer::read_cache_file(&content);
            }
            file = Some(OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&path)
                .expect("Failed to open or create cache file"));
        }

        Tokenizer {
            use_cache,
            cache_file: file,
            token_map,
            hash_map,
        }
    }

    fn read_cache_file(content: &str) -> (HashMap<String, String>, HashMap<String, String>) {
        let mut token_map =  HashMap::new();
        let mut hash_map = HashMap::new();
        for line in content.lines() {
            let split: Vec<&str> =  line.split(',').collect();
            let file_name = split[0];
            let file_hash = split[1];
            let token_content = split[2];

            token_map.insert(file_name.to_string(), token_content.to_string());
            hash_map.insert(file_name.to_string(), file_hash.to_string());
        }

        (token_map, hash_map)
    }

    pub fn new_cached(cache_path: &Path) -> Tokenizer {
        Tokenizer::new(true, Some(cache_path))
    }

    pub fn new_non_cached() -> Tokenizer {
        Tokenizer::new(false, None)
    }

    pub fn tokenize(&mut self, file_path: &Path) -> io::Result<Vec<Token>> {
        if let Some(cached_tokens) = self.cached_tokens_for(file_path)? {
            return Ok(cached_tokens);
        }
        println!("Have to read file");
        let (file_hash, tokens) = tokenize(file_path)?;
        self.cache_if_needed(file_path, file_hash, &tokens)?;
        Ok(tokens)
    }

    fn cache_if_needed(&mut self, file_path: &Path, file_hash: String, tokens: &Vec<Token>) -> io::Result<()> {
        if !self.use_cache {
            return Ok(())
        }

        let file_name = file_path.to_string_lossy().to_string();
        let json_content = serde_json::to_string(tokens)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let encoded_tokens = general_purpose::STANDARD.encode(json_content);
        self.hash_map.insert(file_name.to_string(), file_hash);
        self.token_map.insert(file_name, encoded_tokens);
        Ok(())
    }

    fn cached_tokens_for(&mut self, file_path: &Path) -> io::Result<Option<Vec<Token>>> {
        if self.use_cache {
            let mut file = File::open(file_path)?;
            let mut hasher = Sha256::new();

            io::copy(&mut file, &mut hasher)?;
            let hash_bytes = hasher.finalize();
            let hash = general_purpose::STANDARD.encode(hash_bytes);
            let file_name = file_path.to_string_lossy().to_string();
            if self.hash_map.contains_key(&file_name) && self.token_map.contains_key(&file_name) && *self.hash_map.get(&file_name).unwrap() == hash {
                let encoded_data = self.token_map.get(&file_name).unwrap();
                let decoded_data = general_purpose::STANDARD.decode(encoded_data)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let content = str::from_utf8(&decoded_data)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let tokens: Vec<Token> = serde_json::from_str(content)?;
                return Ok(Some(tokens));
            }
        }
        Ok(None)
    }
}

impl Drop for Tokenizer {
    fn drop(&mut self) {
        if !self.use_cache { return; }
        let mut file: &File;
        if let Some(cache_file) = &self.cache_file {
            file = cache_file;
        } else {
            return;
        }

        for (file_name, file_hash) in &self.hash_map {
            if let Some(token_content) = &self.token_map.get(file_name) {
                writeln!(file, "{},{},{}", file_name, file_hash, token_content).unwrap();
            }
        }
    }
}

fn tokenize(file: &Path) -> io::Result<(String, Vec<Token>)> {
    let mut file = File::open(file)?;
    let mut hasher = Sha256::new();

    io::copy(&mut file, &mut hasher)?;
    let hash_bytes = hasher.finalize();
    let hash = general_purpose::STANDARD.encode(hash_bytes);

    file.seek(SeekFrom::Start(0))?;

    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?;
    let file_content = String::from_utf8(file_data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let tokens = tokenizer::tokenize(&file_content);
    Ok((hash, tokens))
}