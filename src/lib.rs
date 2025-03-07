use pyo3::prelude::*;
use rand::Rng;

#[pyfunction]
pub fn get_list(text: &str) -> PyResult<Vec<String>> {
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
pub fn get_longest_seq(nums: Vec<i32>) -> PyResult<(i32, i32)> {
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
pub fn optimize_length(s: Vec<String>, n: i32) -> PyResult<Vec<String>> {
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
pub fn merge(texts: Vec<String>, n: i32) -> PyResult<Vec<String>> {
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

fn iou(box1: (f32, f32, f32, f32), box2: (f32, f32, f32, f32)) -> f32 {
    let x1 = box1.0.max(box2.0);
    let y1 = box1.1.max(box2.1);
    let x2 = box1.2.min(box2.2);
    let y2 = box1.3.min(box2.3);

    let inter_area = (x2 - x1).max(0.0) * (y2 - y1).max(0.0);

    let box1_area = (box1.2 - box1.0) * (box1.3 - box1.1);
    let box2_area = (box2.2 - box2.0) * (box2.3 - box2.1);

    let union_area = box1_area + box2_area - inter_area;

    if union_area != 0.0 {
        inter_area / union_area
    } else {
        0.0
    }
}

fn contained(box1: (f32, f32, f32, f32), box2: (f32, f32, f32, f32)) -> bool {
    box1.0 <= box2.0 && box1.1 <= box2.1 && box1.2 >= box2.2 && box1.3 >= box2.3
}

#[pyfunction]
pub fn structure(
    detections: Vec<(String, (f32, f32, f32, f32))>,
    iou_threshold: f32,
) -> PyResult<Vec<(String, (f32, f32, f32, f32))>> {
    // 先转换为 mut
    let mut detections = detections;
    let mut filtered_detections = Vec::new();

    while !detections.is_empty() {
        let detection = detections.remove(0);
        let mut keep = true;

        // 用于存储待移除的检测框
        let mut to_remove = Vec::new();

        for other_detection in detections.clone() {
            if iou(detection.1, other_detection.1) > iou_threshold {
                // 随机选择是否移除
                if rand::thread_rng().gen_bool(0.5) {
                    to_remove.push(other_detection);
                } else {
                    keep = false;
                    break;
                }
            } else if contained(detection.1, other_detection.1) {
                to_remove.push(other_detection);
            } else if contained(other_detection.1, detection.1) {
                keep = false;
                break;
            }
        }

        for item in to_remove {
            detections.retain(|x| x != &item);
        }

        if keep {
            filtered_detections.push(detection);
        }
    }

    Ok(filtered_detections)
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_list, m)?)?;
    m.add_function(wrap_pyfunction!(structure, m)?)?;
    m.add_function(wrap_pyfunction!(get_longest_seq, m)?)?;
    m.add_function(wrap_pyfunction!(optimize_length, m)?)?;
    m.add_function(wrap_pyfunction!(merge, m)?)?;
    Ok(())
}
