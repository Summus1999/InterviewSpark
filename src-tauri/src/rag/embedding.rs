//! Embedding service using fastembed for text vectorization

use anyhow::{Context, Result};
#[allow(unused_imports)]
use fastembed::{
    TextEmbedding, InitOptionsUserDefined, UserDefinedEmbeddingModel,
    TokenizerFiles, Pooling, QuantizationMode,
};
use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;

/// Embedding service for converting text to vectors
pub struct EmbeddingService {
    model: Arc<Mutex<TextEmbedding>>,
}

impl EmbeddingService {
    /// Initialize embedding service from local model files
    /// 
    /// # Arguments
    /// * `model_dir` - Directory containing model files (onnx/model.onnx, tokenizer.json, etc.)
    pub fn new_from_local(model_dir: PathBuf) -> Result<Self> {
        log::info!("Initializing embedding model from local files: {:?}", model_dir);
        
        // Define file paths
        let onnx_path = model_dir.join("onnx").join("model.onnx");
        let tokenizer_path = model_dir.join("tokenizer.json");
        let config_path = model_dir.join("config.json");
        let special_tokens_path = model_dir.join("special_tokens_map.json");
        let tokenizer_config_path = model_dir.join("tokenizer_config.json");
        
        // Check if required files exist
        if !onnx_path.exists() {
            log::error!("ONNX model file not found: {:?}", onnx_path);
            return Err(anyhow::anyhow!("ONNX model file not found: {:?}", onnx_path));
        }
        
        if !tokenizer_path.exists() {
            log::error!("Tokenizer file not found: {:?}", tokenizer_path);
            return Err(anyhow::anyhow!("Tokenizer file not found: {:?}", tokenizer_path));
        }
        
        log::info!("Model files found, loading into memory...");
        
        // Read all files into memory
        let onnx_file = std::fs::read(&onnx_path)
            .context(format!("Failed to read ONNX file: {:?}", onnx_path))?;
        log::info!("ONNX file loaded: {} bytes", onnx_file.len());
        
        let tokenizer_file = std::fs::read(&tokenizer_path)
            .context(format!("Failed to read tokenizer file: {:?}", tokenizer_path))?;
        
        let config_file = std::fs::read(&config_path)
            .unwrap_or_else(|_| b"{}".to_vec());
        
        let special_tokens_file = std::fs::read(&special_tokens_path)
            .unwrap_or_else(|_| b"{}".to_vec());
        
        let tokenizer_config_file = std::fs::read(&tokenizer_config_path)
            .unwrap_or_else(|_| b"{}".to_vec());
        
        // Create tokenizer files struct
        let tokenizer_files = TokenizerFiles {
            tokenizer_file,
            config_file,
            special_tokens_map_file: special_tokens_file,
            tokenizer_config_file,
        };
        
        // Create user-defined model (BGE uses CLS pooling)
        let user_model = UserDefinedEmbeddingModel::new(onnx_file, tokenizer_files)
            .with_pooling(Pooling::Cls);
        
        let options = InitOptionsUserDefined::new();
        
        log::info!("Initializing TextEmbedding model...");
        
        let model = TextEmbedding::try_new_from_user_defined(user_model, options)
            .map_err(|e| {
                log::error!("Embedding model init failed: {:?}", e);
                log::error!("Model dir: {:?}", model_dir);
                e
            })
            .context("Failed to initialize embedding model from local files")?;
        
        log::info!("Embedding model initialized successfully from local files");
        
        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }

    /// Embed single text into vector
    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let mut model = self.model.lock().await;
        let embeddings = model
            .embed(vec![text.to_string()], None)
            .context("Failed to generate embedding")?;
        
        embeddings
            .into_iter()
            .next()
            .context("No embedding generated")
    }

    /// Embed multiple texts in batch
    #[allow(dead_code)]
    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut model = self.model.lock().await;
        let embeddings = model
            .embed(texts.to_vec(), None)
            .context("Failed to generate batch embeddings")?;
        
        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embed_text() {
        // Skip if model not available locally
        let model_dir = PathBuf::from("resources/models/models--Xenova--bge-small-zh-v1.5");
        if !model_dir.exists() {
            println!("Skipping test: model not available");
            return;
        }
        
        let service = EmbeddingService::new_from_local(model_dir).expect("Failed to init service");
        let result = service.embed_text("测试文本").await;
        assert!(result.is_ok());
        let vec = result.unwrap();
        assert!(!vec.is_empty());
    }
}
