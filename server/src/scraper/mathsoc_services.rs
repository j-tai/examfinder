use std::borrow::Cow;
use std::collections::HashMap;

use serde::Deserialize;
use tracing::{debug, trace};

use crate::db::RedisConn;
use crate::error::AppResult;
use crate::exam::Source;

use super::{cache, Exam, CLIENT};

#[derive(Deserialize)]
struct Subject<'a> {
    #[serde(borrow)]
    code: Cow<'a, str>,
    courses: Vec<i32>,
}

#[derive(Deserialize)]
struct RawExam<'a> {
    #[serde(borrow)]
    term: Cow<'a, str>,
    #[serde(borrow, rename = "type")]
    kind: Cow<'a, str>,
    #[serde(borrow)]
    exam_url: Cow<'a, str>,
    #[serde(borrow)]
    solution_url: Cow<'a, str>,
}

pub async fn fetch(redis: &mut RedisConn, course: &str) -> AppResult<Vec<Exam>> {
    // Fetch the list of courses
    // Example: {"CS135": ("CS", 135), ...}
    let mut courses = cache(redis, "mathsoc_services:courses", async {
        debug!("Fetching course list");
        let response = CLIENT.get(SUBJECTS_URL).send().await?;
        let text = response.text().await?;
        let subjects: Vec<Subject> = serde_json::from_str(&text)?;
        let courses: HashMap<_, _> = subjects
            .into_iter()
            .flat_map(|subject| {
                let code = subject.code;
                subject
                    .courses
                    .into_iter()
                    .map(move |num| (format!("{code}{num}"), (code.to_string(), num)))
            })
            .collect();
        Ok(courses)
    })
    .await?;

    // Verify the course is in the courses list
    let (subject, number) = match courses.remove(course) {
        Some(data) => data,
        None => return Ok(vec![]),
    };

    // Fetch this course's exams
    let key = format!("mathsoc_services:course:{course}");
    let result: Vec<Exam> = cache(redis, &key, async move {
        debug!("Fetching course {course:?}");
        let course_url = format!("{EXAMS_URL}?subject={subject}&course={number}");
        let response = CLIENT.get(&course_url).send().await?;
        let text = response.text().await?;
        let raw_exams: Vec<RawExam> = serde_json::from_str(&text)?;
        raw_exams
            .into_iter()
            .map(|raw| {
                let (term, year) = raw.term.split_once(' ').ok_or("couldn't split term")?;
                let term = term.parse()?;
                let year = year.parse().map_err(|_| "couldn't parse year")?;
                let kind = raw.kind.parse()?;
                Ok(Exam {
                    year,
                    term,
                    kind,
                    source: SOURCE,
                    exam_url: parse_path(&raw.exam_url),
                    solution_url: parse_path(&raw.solution_url),
                })
            })
            .collect()
    })
    .await?;

    trace!("{} exams for course {course:?}", result.len());
    Ok(result)
}

fn parse_path(path: &str) -> Option<String> {
    if path.is_empty() {
        return None;
    }
    Some(format!("{BASE_URL}{path}"))
}

const SUBJECTS_URL: &str = "https://services.mathsoc.uwaterloo.ca/university/courses/";
const EXAMS_URL: &str = "https://services.mathsoc.uwaterloo.ca/university/exams/";
const BASE_URL: &str = "https://services.mathsoc.uwaterloo.ca";
const SOURCE: Source = Source::MathSocServices;
