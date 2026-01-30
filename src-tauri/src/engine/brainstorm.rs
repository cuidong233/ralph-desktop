use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::fs;
use std::path::Path;

/// Question template for brainstorm
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionTemplate {
    pub id: String,
    pub phase: String,
    pub question: String,
    pub description: Option<String>,
    pub question_type: QuestionType,
    pub options: Vec<QuestionOption>,
    pub allow_other: bool,
    pub required: bool,
    pub condition: Option<QuestionCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionCondition {
    pub question_id: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuestionType {
    Single,
    Multiple,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

/// Get the question flow for brainstorm
pub fn get_question_flow() -> Vec<QuestionTemplate> {
    vec![
        // Phase 1: Quick assessment
        QuestionTemplate {
            id: "task_type".to_string(),
            phase: "assessment".to_string(),
            question: "你想做什么类型的任务？".to_string(),
            description: None,
            question_type: QuestionType::Single,
            options: vec![
                QuestionOption {
                    value: "greenfield".to_string(),
                    label: "从零构建新项目".to_string(),
                    description: Some("项目目录是空的，从头开始".to_string()),
                },
                QuestionOption {
                    value: "feature".to_string(),
                    label: "给现有项目加功能".to_string(),
                    description: Some("已有代码，添加新功能".to_string()),
                },
                QuestionOption {
                    value: "refactor".to_string(),
                    label: "重构/优化代码".to_string(),
                    description: Some("改进现有代码的质量或性能".to_string()),
                },
                QuestionOption {
                    value: "bugfix".to_string(),
                    label: "修复 Bug / 让测试通过".to_string(),
                    description: Some("有失败的测试或已知 Bug".to_string()),
                },
            ],
            allow_other: true,
            required: true,
            condition: None,
        },
        // Phase 2: Requirements
        QuestionTemplate {
            id: "project_description".to_string(),
            phase: "requirements".to_string(),
            question: "用一句话描述你想做的事情".to_string(),
            description: Some("不需要太详细，先说个大概".to_string()),
            question_type: QuestionType::Text,
            options: vec![],
            allow_other: false,
            required: true,
            condition: None,
        },
        QuestionTemplate {
            id: "tech_stack".to_string(),
            phase: "technical".to_string(),
            question: "你想用什么技术栈？".to_string(),
            description: None,
            question_type: QuestionType::Single,
            options: vec![
                QuestionOption {
                    value: "node".to_string(),
                    label: "Node.js + Express/Fastify".to_string(),
                    description: None,
                },
                QuestionOption {
                    value: "python".to_string(),
                    label: "Python + FastAPI/Flask".to_string(),
                    description: None,
                },
                QuestionOption {
                    value: "go".to_string(),
                    label: "Go + Gin/Echo".to_string(),
                    description: None,
                },
                QuestionOption {
                    value: "rust".to_string(),
                    label: "Rust + Axum/Actix".to_string(),
                    description: None,
                },
                QuestionOption {
                    value: "react".to_string(),
                    label: "React + TypeScript".to_string(),
                    description: None,
                },
                QuestionOption {
                    value: "svelte".to_string(),
                    label: "Svelte + TypeScript".to_string(),
                    description: None,
                },
            ],
            allow_other: true,
            required: true,
            condition: Some(QuestionCondition {
                question_id: "task_type".to_string(),
                values: vec!["greenfield".to_string()],
            }),
        },
        QuestionTemplate {
            id: "existing_code_info".to_string(),
            phase: "requirements".to_string(),
            question: "项目里有什么需要我了解的？".to_string(),
            description: Some("比如使用了什么框架、有什么特殊约定".to_string()),
            question_type: QuestionType::Text,
            options: vec![],
            allow_other: false,
            required: false,
            condition: Some(QuestionCondition {
                question_id: "task_type".to_string(),
                values: vec!["feature".to_string(), "refactor".to_string(), "bugfix".to_string()],
            }),
        },
        QuestionTemplate {
            id: "test_requirement".to_string(),
            phase: "criteria".to_string(),
            question: "需要写测试吗？".to_string(),
            description: None,
            question_type: QuestionType::Single,
            options: vec![
                QuestionOption {
                    value: "full".to_string(),
                    label: "完整测试".to_string(),
                    description: Some("单元测试 + 集成测试，覆盖率 > 80%".to_string()),
                },
                QuestionOption {
                    value: "basic".to_string(),
                    label: "基础测试".to_string(),
                    description: Some("核心功能有测试即可".to_string()),
                },
                QuestionOption {
                    value: "none".to_string(),
                    label: "不需要测试".to_string(),
                    description: Some("只要代码能跑就行".to_string()),
                },
            ],
            allow_other: false,
            required: true,
            condition: None,
        },
        QuestionTemplate {
            id: "additional_requirements".to_string(),
            phase: "criteria".to_string(),
            question: "还有其他要求吗？".to_string(),
            description: Some("比如代码风格、文档、特定的库等".to_string()),
            question_type: QuestionType::Text,
            options: vec![],
            allow_other: false,
            required: false,
            condition: None,
        },
    ]
}

/// Generate prompt from brainstorm answers
pub fn generate_prompt(answers: &std::collections::HashMap<String, serde_json::Value>) -> String {
    let task_type = answers
        .get("task_type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let task_type_desc = match task_type {
        "greenfield" => "从零构建一个新项目",
        "feature" => "给现有项目添加新功能",
        "refactor" => "重构和优化现有代码",
        "bugfix" => "修复 Bug 或让测试通过",
        _ => task_type,
    };

    let description = answers
        .get("project_description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let tech_stack = answers
        .get("tech_stack")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let existing_info = answers
        .get("existing_code_info")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let test_req = answers
        .get("test_requirement")
        .and_then(|v| v.as_str())
        .unwrap_or("basic");

    let test_desc = match test_req {
        "full" => "完整测试覆盖（单元测试 + 集成测试，覆盖率 > 80%）",
        "basic" => "基础测试（核心功能有测试即可）",
        "none" => "不需要测试",
        _ => test_req,
    };

    let additional = answers
        .get("additional_requirements")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let mut prompt = format!(
        r#"你正在进行一个 Ralph Loop 任务。请仔细阅读以下要求，然后开始工作。

## 任务描述

{}

## 任务类型

{}
"#,
        description, task_type_desc
    );

    if !tech_stack.is_empty() {
        prompt.push_str(&format!(
            r#"
## 技术栈

{}
"#,
            tech_stack
        ));
    }

    if !existing_info.is_empty() {
        prompt.push_str(&format!(
            r#"
## 现有代码信息

{}
"#,
            existing_info
        ));
    }

    prompt.push_str(&format!(
        r#"
## 测试要求

{}
"#,
        test_desc
    ));

    if !additional.is_empty() {
        prompt.push_str(&format!(
            r#"
## 其他要求

{}
"#,
            additional
        ));
    }

    prompt.push_str(
        r#"
## 完成标准

当你完成所有上述要求后，请输出以下内容表示任务完成：

<done>COMPLETE</done>

## 工作方式

1. 先分析当前代码状态（如果有）
2. 制定实施计划
3. 逐步实现，每完成一步就运行测试（如果有）
4. 遇到错误时，分析原因并修复
5. 全部完成后，输出完成信号

开始工作吧！
"#,
    );

    prompt
}

/// Generate design document and save to project directory
pub fn generate_design_doc(
    project_name: &str,
    project_path: &Path,
    answers: &std::collections::HashMap<String, serde_json::Value>,
    prompt: &str,
) -> Result<String, String> {
    let now = Utc::now();
    let date_str = now.format("%Y-%m-%d").to_string();

    let task_type = answers
        .get("task_type")
        .and_then(|v| v.as_str())
        .unwrap_or("task");

    let description = answers
        .get("project_description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let tech_stack = answers
        .get("tech_stack")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let test_req = answers
        .get("test_requirement")
        .and_then(|v| v.as_str())
        .unwrap_or("basic");

    let additional = answers
        .get("additional_requirements")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Build answers table
    let mut answers_table = String::new();
    for (key, value) in answers {
        let value_str = match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Array(arr) => arr
                .iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            _ => value.to_string(),
        };
        answers_table.push_str(&format!("| {} | {} |\n", key, value_str));
    }

    let doc_content = format!(
        r#"# {} 设计文档

> 生成时间: {}
> 由 Ralph Desktop Brainstorm 生成

## 概述

{}

## 需求分析

### 用户回答摘要

| 问题 | 回答 |
|------|------|
{}

### 功能需求

- 任务类型: {}
- 技术栈: {}
- 测试要求: {}
{}

## 完成标准

任务完成的标准：
- [ ] 实现所有描述的功能
- [ ] 代码能够正常运行
- [ ] 满足测试要求
- [ ] 输出完成信号 `<done>COMPLETE</done>`

## 生成的 Prompt

```
{}
```

---

*此文档由 Ralph Desktop 自动生成，可手动编辑。*
"#,
        project_name,
        now.format("%Y-%m-%d %H:%M:%S UTC"),
        description,
        answers_table,
        task_type,
        if tech_stack.is_empty() { "未指定" } else { tech_stack },
        test_req,
        if additional.is_empty() { String::new() } else { format!("- 其他要求: {}", additional) },
        prompt
    );

    // Create docs/plans directory
    let docs_dir = project_path.join("docs").join("plans");
    fs::create_dir_all(&docs_dir).map_err(|e| e.to_string())?;

    // Generate filename
    let filename = format!("{}-{}-design.md", date_str, task_type);
    let doc_path = docs_dir.join(&filename);

    // Write file
    fs::write(&doc_path, doc_content).map_err(|e| e.to_string())?;

    Ok(doc_path.to_string_lossy().to_string())
}
