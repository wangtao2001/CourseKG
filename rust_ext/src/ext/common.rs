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
