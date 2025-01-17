use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn get_title_from_latex(latex: String) -> PyResult<Vec<String>> {
    let mut titles = Vec::new();

    let commands = vec![
        "title",
        "part",
        "chapter",
        "section",
        "subsection",
        "subsubsection",
        "paragraph",
        "subparagraph",
    ];

    for command in commands {
        let pattern = format!(r"\\({})\{{(.*?)\}}", command);
        let re = Regex::new(&pattern).map_err(|e| PyValueError::new_err(e.to_string()))?;

        for caps in re.captures_iter(&latex) {
            if let Some(title) = caps.get(1) {
                titles.push(title.as_str().to_string());
            }
        }
    }

    Ok(titles)
}

#[pyfunction]
pub fn get_list_from_string(text: &str) -> PyResult<Vec<String>> {
    let mut list_string = String::new();
    let mut stack = 0;
    let mut chars = text.chars();
    let mut result = Vec::new();

    while let Some(s) = chars.next() {
        if s == '[' {
            stack += 1;
        }
        if stack > 0 {
            list_string.push(s);
        }
        if s == ']' {
            stack -= 1;
            if stack == 0 {
                result = vec![list_string.to_string()];
                break;
            }
        }
    }

    Ok(result)
}

#[pyfunction]
pub fn find_longest_consecutive_sequence(nums: Vec<i32>) -> PyResult<(i32, i32)> {
    if nums.is_empty() {
        return Ok((-1, -1));
    }

    let mut max_start = nums[0];
    let mut max_end = nums[0];
    let mut current_start = nums[0];
    let mut max_length = 1;
    let mut current_length = 1;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] + 1 {
            current_length += 1;
            if current_length > max_length {
                max_length = current_length;
                max_start = current_start;
                max_end = nums[i];
            }
        } else {
            current_start = nums[i];
            current_length = 1;
        }
    }

    Ok((max_start, max_end))
}

#[pyfunction]
pub fn optimize_strings_length(s: Vec<String>, n: i32) -> PyResult<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let mut buffer = String::new();

    for string in s {
        if string.chars().count() < n as usize {
            buffer.push_str(&string);
            // buffer.push_str("\n");
            if buffer.chars().count() >= n as usize {
                let trimmed = buffer.trim_end().to_string();
                result.push(trimmed);
                buffer.clear();
            }
        } else {
            let sentences: Vec<&str> = string.split('。').collect();
            for (i, sentence) in sentences.iter().enumerate() {
                let mut current = String::from(*sentence);
                if i < sentences.len() - 1 {
                    current.push('。');
                }
                if current.chars().count() < n as usize {
                    buffer.push_str(&current);
                    if buffer.chars().count() >= n as usize {
                        result.push(buffer.clone());
                        buffer.clear();
                    }
                } else {
                    result.push(current);
                }
            }
        }
    }
    if !buffer.is_empty() {
        result.push(buffer);
    }

    Ok(result)
}

#[pyfunction]
pub fn merge_strings(texts: Vec<String>, n: i32) -> PyResult<Vec<String>> {
    let mut result = Vec::new();
    let mut chunks = Vec::new();

    // 分割并收集所有文本片段
    for text in texts {
        chunks.extend(
            text.split('。')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
        );
    }

    // 为不以句号结尾的片段添加句号
    chunks = chunks
        .into_iter()
        .map(|chunk| {
            if !chunk.ends_with('。') {
                chunk + "。"
            } else {
                chunk
            }
        })
        .collect();

    let mut current = String::new();
    let mut is_first_exceed = true;

    for chunk in chunks {
        let current_len = current.chars().count();
        let chunk_len = chunk.chars().count();

        if current_len + chunk_len < n as usize {
            current.push_str(&chunk);
        } else if current_len + chunk_len == n as usize {
            current.push_str(&chunk);
            result.push(current);
            current = String::new();
            is_first_exceed = true;
        } else {
            if is_first_exceed {
                is_first_exceed = false;
                current.push_str(&chunk);
            } else {
                if !current.is_empty() {
                    result.push(current);
                }
                current = chunk;
                is_first_exceed = true;
            }
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    Ok(result)
}
