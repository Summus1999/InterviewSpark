//! Report export functionality for text and HTML formats

use crate::db::SessionReport;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Report exporter for various formats
pub struct ReportExporter;

impl ReportExporter {
    /// Export report to plain text format
    pub fn export_to_text(report: &SessionReport, file_path: &PathBuf) -> Result<()> {
        let timestamp = chrono::DateTime::parse_from_rfc3339(&report.generated_at)
            .ok()
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| report.generated_at.clone());

        let improvements: Vec<String> = serde_json::from_str(&report.improvements)
            .unwrap_or_default();
        let key_takeaways: Vec<String> = serde_json::from_str(&report.key_takeaways)
            .unwrap_or_default();

        let grade = Self::score_to_grade(report.overall_score);

        let content = format!(
            "========================================\n\
             面试复盘报告\n\
             ========================================\n\n\
             生成时间：{}\n\
             综合评分：{:.1}/10  [{}]\n\n\
             一、整体表现\n\
             {}\n\n\
             二、改进建议\n{}\n\n\
             三、关键要点\n{}\n\n\
             ========================================\n",
            timestamp,
            report.overall_score,
            grade,
            report.summary,
            improvements
                .iter()
                .map(|item| format!("• {}", item))
                .collect::<Vec<_>>()
                .join("\n"),
            key_takeaways
                .iter()
                .map(|item| format!("• {}", item))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(file_path, content)?;
        Ok(())
    }

    /// Export report to HTML format
    pub fn export_to_html(report: &SessionReport, file_path: &PathBuf) -> Result<()> {
        let timestamp = chrono::DateTime::parse_from_rfc3339(&report.generated_at)
            .ok()
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| report.generated_at.clone());

        let improvements: Vec<String> = serde_json::from_str(&report.improvements)
            .unwrap_or_default();
        let key_takeaways: Vec<String> = serde_json::from_str(&report.key_takeaways)
            .unwrap_or_default();

        let grade = Self::score_to_grade(report.overall_score);
        let score_color = Self::score_to_color(report.overall_score);

        let improvements_html = improvements
            .iter()
            .map(|item| format!("<li>{}</li>", html_escape(item)))
            .collect::<Vec<_>>()
            .join("\n");

        let takeaways_html = key_takeaways
            .iter()
            .map(|item| format!("<li>{}</li>", html_escape(item)))
            .collect::<Vec<_>>()
            .join("\n");

        let html_content = format!(
            r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>面试复盘报告</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f5f5f5;
            padding: 20px;
        }}
        
        .report-container {{
            max-width: 900px;
            margin: 0 auto;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            overflow: hidden;
        }}
        
        .report-header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px;
            text-align: center;
        }}
        
        .report-header h1 {{
            font-size: 32px;
            margin-bottom: 20px;
        }}
        
        .report-meta {{
            display: flex;
            justify-content: space-around;
            flex-wrap: wrap;
            gap: 20px;
            margin-top: 20px;
        }}
        
        .meta-item {{
            text-align: center;
        }}
        
        .meta-label {{
            font-size: 14px;
            opacity: 0.9;
        }}
        
        .score-badge {{
            font-size: 28px;
            font-weight: bold;
            margin-top: 5px;
        }}
        
        .grade {{
            display: inline-block;
            padding: 4px 12px;
            background: {score_color};
            border-radius: 4px;
            font-size: 14px;
            margin-left: 10px;
        }}
        
        .report-body {{
            padding: 40px;
        }}
        
        .report-section {{
            margin-bottom: 40px;
        }}
        
        .section-title {{
            font-size: 20px;
            font-weight: bold;
            color: #667eea;
            margin-bottom: 15px;
            padding-bottom: 10px;
            border-bottom: 2px solid #667eea;
        }}
        
        .section-content {{
            color: #555;
            line-height: 1.8;
        }}
        
        .section-content p {{
            margin-bottom: 10px;
            text-align: justify;
        }}
        
        .section-content ul {{
            list-style: none;
            padding-left: 0;
        }}
        
        .section-content li {{
            padding-left: 30px;
            margin-bottom: 12px;
            position: relative;
        }}
        
        .section-content li:before {{
            content: "✓";
            position: absolute;
            left: 0;
            color: #667eea;
            font-weight: bold;
        }}
        
        .footer {{
            background: #f9f9f9;
            padding: 20px 40px;
            text-align: center;
            color: #999;
            font-size: 12px;
            border-top: 1px solid #eee;
        }}
        
        @media print {{
            body {{
                background: white;
                padding: 0;
            }}
            
            .report-container {{
                box-shadow: none;
                border-radius: 0;
            }}
        }}
    </style>
</head>
<body>
    <div class="report-container">
        <div class="report-header">
            <h1>面试复盘报告</h1>
            <div class="report-meta">
                <div class="meta-item">
                    <div class="meta-label">生成时间</div>
                    <div class="meta-value">{}</div>
                </div>
                <div class="meta-item">
                    <div class="meta-label">综合评分</div>
                    <div class="score-badge">
                        {:.1}/10
                        <span class="grade">{}</span>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="report-body">
            <div class="report-section">
                <div class="section-title">一、整体表现</div>
                <div class="section-content">
                    <p>{}</p>
                </div>
            </div>
            
            <div class="report-section">
                <div class="section-title">二、改进建议</div>
                <div class="section-content">
                    <ul>
                        {}
                    </ul>
                </div>
            </div>
            
            <div class="report-section">
                <div class="section-title">三、关键要点</div>
                <div class="section-content">
                    <ul>
                        {}
                    </ul>
                </div>
            </div>
        </div>
        
        <div class="footer">
            <p>本报告由 InterviewSpark 自动生成 | <a href="https://github.com/summus/InterviewSpark">项目主页</a></p>
        </div>
    </div>
</body>
</html>"#,
            timestamp,
            report.overall_score,
            grade,
            html_escape(&report.summary),
            improvements_html,
            takeaways_html,
            score_color = score_color
        );

        fs::write(file_path, html_content)?;
        Ok(())
    }

    /// Convert score to letter grade
    fn score_to_grade(score: f32) -> &'static str {
        match score {
            s if s >= 9.0 => "A+",
            s if s >= 8.5 => "A",
            s if s >= 8.0 => "A-",
            s if s >= 7.5 => "B+",
            s if s >= 7.0 => "B",
            s if s >= 6.5 => "B-",
            s if s >= 6.0 => "C+",
            s if s >= 5.0 => "C",
            s if s >= 4.0 => "D",
            _ => "F",
        }
    }

    /// Convert score to color for badge
    fn score_to_color(score: f32) -> &'static str {
        match score {
            s if s >= 8.5 => "#4CAF50",  // Green
            s if s >= 7.5 => "#8BC34A",  // Light Green
            s if s >= 6.5 => "#FFC107",  // Amber
            s if s >= 5.5 => "#FF9800",  // Orange
            _ => "#F44336",              // Red
        }
    }
}

/// Simple HTML escape function
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
