//! Bootstrap module for initializing knowledge base

use anyhow::Result;
use serde::{Deserialize, Serialize};
use super::{EmbeddingService, VectorStore};
use crate::api::siliconflow::SiliconFlowClient;

/// Bootstrap progress status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct BootstrapProgress {
    pub current: usize,
    pub total: usize,
    pub status: String,
    pub category: String,
}

/// Bootstrap result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct BootstrapResult {
    pub total_questions: usize,
    pub total_answers: usize,
    pub success: bool,
    pub message: String,
    pub failed_items: Vec<String>,
}

/// Knowledge status for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeStatus {
    pub is_empty: bool,
    pub question_count: i64,
    pub answer_count: i64,
}

/// Knowledge stats for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeStats {
    pub total_vectors: i64,
    pub question_count: i64,
    pub answer_count: i64,
    pub jd_count: i64,
}

/// JD template for question generation
#[allow(dead_code)]
struct JdTemplate {
    category: &'static str,
    name: &'static str,
    content: &'static str,
}

/// Get predefined JD templates
fn get_jd_templates() -> Vec<JdTemplate> {
    vec![
        JdTemplate {
            category: "frontend",
            name: "高级前端工程师",
            content: "岗位职责：负责公司核心产品前端开发和维护；参与前端技术方案设计和架构优化。任职要求：3年以上前端开发经验；精通Vue.js和React框架；掌握TypeScript、ES6+；熟悉Webpack、Vite等构建工具。",
        },
        JdTemplate {
            category: "backend",
            name: "高级后端工程师",
            content: "岗位职责：负责后端系统的架构设计和实现；处理高并发、大数据量的技术挑战。任职要求：5年以上后端开发经验；精通Java/Go/Python；深入理解数据库原理；有分布式系统设计经验。",
        },
        JdTemplate {
            category: "pm",
            name: "产品经理",
            content: "岗位职责：负责产品模块的需求分析、设计和上线；进行用户研究和竞品分析。任职要求：2年以上互联网产品经理工作经验；掌握Axure、Figma等原型工具；具有数据分析能力。",
        },
        JdTemplate {
            category: "fullstack",
            name: "全栈工程师",
            content: "岗位职责：同时负责前后端开发；参与技术方案评审和架构设计。任职要求：4年以上开发经验；掌握React/Vue和Node.js；熟悉数据库设计和API开发。",
        },
        JdTemplate {
            category: "qa",
            name: "测试工程师",
            content: "岗位职责：负责产品功能、性能和兼容性测试；设计和开发自动化测试框架。任职要求：2年以上测试经验；掌握Python/Java编程；熟悉Selenium、JMeter等工具。",
        },
        JdTemplate {
            category: "devops",
            name: "DevOps工程师",
            content: "岗位职责：负责基础设施和运维系统的设计与维护；设计和实现CI/CD流程。任职要求：3年以上运维或DevOps经验；精通Docker和Kubernetes；掌握Python/Go/Bash。",
        },
    ]
}

/// Knowledge base bootstrapper
pub struct KnowledgeBootstrap {
    embedding_service: EmbeddingService,
    vector_store: VectorStore,
    api_client: SiliconFlowClient,
}

#[allow(dead_code)]
impl KnowledgeBootstrap {
    /// Create new bootstrap instance
    pub fn new(
        embedding_service: EmbeddingService,
        vector_store: VectorStore,
        api_client: SiliconFlowClient,
    ) -> Self {
        Self {
            embedding_service,
            vector_store,
            api_client,
        }
    }

    /// Check if knowledge base is empty
    pub async fn is_empty(&self) -> Result<bool> {
        let count = self.vector_store.count().await?;
        Ok(count == 0)
    }

    /// Get knowledge base status
    pub async fn get_status(&self) -> Result<KnowledgeStatus> {
        let total = self.vector_store.count().await?;
        let question_count = self.vector_store.count_by_type("question").await?;
        let answer_count = self.vector_store.count_by_type("answer").await?;
        
        Ok(KnowledgeStatus {
            is_empty: total == 0,
            question_count,
            answer_count,
        })
    }

    /// Get knowledge base stats
    pub async fn get_stats(&self) -> Result<KnowledgeStats> {
        let total = self.vector_store.count().await?;
        let question_count = self.vector_store.count_by_type("question").await?;
        let answer_count = self.vector_store.count_by_type("answer").await?;
        let jd_count = self.vector_store.count_by_type("jd").await?;
        
        Ok(KnowledgeStats {
            total_vectors: total,
            question_count,
            answer_count,
            jd_count,
        })
    }

    /// Bootstrap knowledge base with initial data
    /// Takes a progress callback for UI updates
    pub async fn bootstrap<F>(&self, mut on_progress: F) -> Result<BootstrapResult>
    where
        F: FnMut(BootstrapProgress),
    {
        let templates = get_jd_templates();
        let questions_per_category = 10; // Reduced for faster bootstrap
        let total_items = templates.len() * questions_per_category * 2; // questions + answers
        
        let mut total_questions = 0;
        let mut total_answers = 0;
        let mut failed_items: Vec<String> = Vec::new();
        let mut current = 0;
        
        for template in &templates {
            log::info!("Generating questions for category: {}", template.category);
            
            // Report progress
            on_progress(BootstrapProgress {
                current,
                total: total_items,
                status: format!("正在为{}生成面试题...", template.name),
                category: template.category.to_string(),
            });
            
            // Generate questions using API
            let questions = match self.api_client.generate_questions(
                "", // Empty resume for generic questions
                template.content,
                questions_per_category as u32,
                "friendly",
            ).await {
                Ok(qs) => qs,
                Err(e) => {
                    log::error!("Failed to generate questions for {}: {}", template.category, e);
                    failed_items.push(format!("{}:questions", template.category));
                    continue;
                }
            };
            
            for (idx, question) in questions.iter().enumerate() {
                current += 1;
                
                // Report progress
                on_progress(BootstrapProgress {
                    current,
                    total: total_items,
                    status: format!("正在处理{}第{}题...", template.name, idx + 1),
                    category: template.category.to_string(),
                });
                
                // Embed and store question
                match self.embedding_service.embed_text(question).await {
                    Ok(q_embedding) => {
                        let metadata = serde_json::json!({
                            "category": template.category,
                            "jd_name": template.name,
                        }).to_string();
                        
                        if let Err(e) = self.vector_store.insert(
                            "question",
                            question,
                            &q_embedding,
                            Some(&metadata),
                        ).await {
                            log::error!("Failed to store question: {}", e);
                            failed_items.push(format!("{}:q{}", template.category, idx));
                        } else {
                            total_questions += 1;
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to embed question: {}", e);
                        failed_items.push(format!("{}:q{}_embed", template.category, idx));
                    }
                }
                
                current += 1;
                
                // Generate and store best answer
                on_progress(BootstrapProgress {
                    current,
                    total: total_items,
                    status: format!("正在生成{}第{}题答案...", template.name, idx + 1),
                    category: template.category.to_string(),
                });
                
                match self.api_client.generate_best_answer(
                    question,
                    template.content,
                    &[], // No historical answers
                ).await {
                    Ok(answer) => {
                        match self.embedding_service.embed_text(&answer).await {
                            Ok(a_embedding) => {
                                let metadata = serde_json::json!({
                                    "category": template.category,
                                    "question": question,
                                    "score": 8.5,
                                }).to_string();
                                
                                if let Err(e) = self.vector_store.insert(
                                    "answer",
                                    &answer,
                                    &a_embedding,
                                    Some(&metadata),
                                ).await {
                                    log::error!("Failed to store answer: {}", e);
                                    failed_items.push(format!("{}:a{}", template.category, idx));
                                } else {
                                    total_answers += 1;
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to embed answer: {}", e);
                                failed_items.push(format!("{}:a{}_embed", template.category, idx));
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to generate answer: {}", e);
                        failed_items.push(format!("{}:a{}_gen", template.category, idx));
                    }
                }
            }
        }
        
        // Build HNSW index
        on_progress(BootstrapProgress {
            current: total_items,
            total: total_items,
            status: "正在构建向量索引...".to_string(),
            category: "index".to_string(),
        });
        
        if let Err(e) = self.vector_store.build_index().await {
            log::error!("Failed to build index: {}", e);
            failed_items.push("index_build".to_string());
        }
        
        let success = failed_items.is_empty();
        let message = if success {
            format!("成功初始化知识库：{}道题目，{}条答案", total_questions, total_answers)
        } else {
            format!("知识库初始化完成，{}道题目，{}条答案，{}项失败", 
                total_questions, total_answers, failed_items.len())
        };
        
        Ok(BootstrapResult {
            total_questions,
            total_answers,
            success,
            message,
            failed_items,
        })
    }
}
