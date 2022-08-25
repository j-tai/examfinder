use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{debug, trace};

use crate::db::RedisConn;
use crate::error::AppResult;
use crate::exam::{Exam, Source};

use super::{cache, CLIENT};

pub async fn fetch(redis: &mut RedisConn, course: &str) -> AppResult<Vec<Exam>> {
    let mut exams = cache(redis, "mathsoc:exams", async {
        debug!("Fetching exams");
        let response = CLIENT.get(URL).send().await?;
        let text = response.text().await?;

        // Organize all the exams into this map:
        //
        //     {course: {(year, term, kind): (exam_url, solution_url)}}
        //
        // so that the exam and solution URLs can be matched together.
        let mut records = HashMap::new();

        for name in parse_name_list(&text)? {
            let mtch = match EXAM_NAME.captures(&name) {
                Some(m) => {
                    trace!("Parsed exam name {name:?}");
                    m
                }
                None => {
                    trace!("Couldn't parse exam name {name:?}");
                    continue;
                }
            };
            let course = mtch.get(1).unwrap().as_str();
            let year: i32 = mtch[3].parse().unwrap();
            let term = mtch[2].parse()?;
            let kind = mtch[4].parse()?;
            let exam_or_solution = &mtch[5];
            let url = format!("{BASE_URL}{course}{term}-{year}{kind}{exam_or_solution}.pdf");

            let entry = records
                .entry(course)
                .or_insert_with(|| HashMap::new())
                .entry((year, term, kind))
                .or_insert((None, None));
            if exam_or_solution == "solution" {
                entry.1 = Some(url);
            } else {
                entry.0 = Some(url);
            }
        }

        // Convert the records into exams
        let exams: HashMap<String, Vec<Exam>> = records
            .into_iter()
            .map(|(course, exams)| {
                (
                    course.to_string(),
                    exams
                        .into_iter()
                        .map(|((year, term, kind), (exam_url, solution_url))| Exam {
                            year,
                            term,
                            kind,
                            source: SOURCE,
                            exam_url,
                            solution_url,
                        })
                        .collect(),
                )
            })
            .collect();

        Ok(exams)
    })
    .await?;

    let result = exams.remove(course).unwrap_or_else(|| vec![]);
    trace!("{} exams for course {course:?}", result.len());
    Ok(result)
}

fn parse_name_list(text: &str) -> AppResult<impl Iterator<Item = &str>> {
    let start_pos = NAMES_START.find(&text).ok_or("couldn't find start")?.end();
    let text = &text[start_pos..];
    let end_pos = text.find(NAMES_END).ok_or("couldn't find end")?;
    let text = &text[..end_pos];

    Ok(text
        .split(',')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .flat_map(|part| {
            let part = part.trim();
            let is_string_delim = |c| c == '\'' || c == '"';
            if let Some(part) = part.strip_prefix(is_string_delim) {
                if let Some(part) = part.strip_suffix(is_string_delim) {
                    return Some(part);
                }
            }
            debug!("Couldn't parse string literal {part:?}");
            None
        }))
}

const URL: &str = "https://mathsoc.uwaterloo.ca/exam-bank/";
const BASE_URL: &str = "https://mathsoc.uwaterloo.ca/wp-content/uploads/";
const NAMES_START: Lazy<Regex> = Lazy::new(|| Regex::new(r"let names\s*=\s*\[").unwrap());
const NAMES_END: char = ']';
const EXAM_NAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\w+)(Fall|Spring|Winter)(\d{4})(\w+)(exam|solution)$").unwrap());
const SOURCE: Source = Source::MathSoc;

#[test]
fn parse_name_list_works() {
    let names: Vec<_> = parse_name_list(
        r#"
let names = [
"foo" , 'bar',
'baz'
,'asdf.pdf', 
]
"#,
    )
    .unwrap()
    .collect();
    assert_eq!(names, ["foo", "bar", "baz", "asdf.pdf"]);
}
