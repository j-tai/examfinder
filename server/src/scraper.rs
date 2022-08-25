mod mathsoc;
mod mathsoc_services;

use std::future::Future;
use std::time::Duration;

use once_cell::sync::Lazy;
use redis::AsyncCommands;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::trace;

use crate::db::RedisConn;
use crate::error::AppResult;
use crate::exam::Exam;

pub async fn fetch(redis: &mut RedisConn, course: &str) -> AppResult<Vec<Exam>> {
    let mut result = mathsoc::fetch(redis, course).await?;
    result.append(&mut mathsoc_services::fetch(redis, course).await?);
    result.sort();
    result.reverse();

    let old_len = result.len();
    result = deduplicate(result.into_iter());
    trace!(
        "Deduplicated: {old_len} -> {} exams for course {course:?}",
        result.len(),
    );
    Ok(result)
}

fn deduplicate(mut exams: impl Iterator<Item = Exam>) -> Vec<Exam> {
    let mut result = vec![];
    let mut previous = if let Some(first) = exams.next() {
        first
    } else {
        return result; // iterator was empty
    };

    for exam in exams {
        if exam.year == previous.year
            && exam.term == previous.term
            && exam.kind == previous.kind
            && exam.source != previous.source
        {
            // Same exam
            previous = exam;
        } else {
            // Different exams
            result.push(previous);
            previous = exam;
        }
    }

    result.push(previous);
    result
}

/// Reqwest client to be used for scraping.
static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:91.0) Gecko/20100101 Firefox/91.0")
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .unwrap()
});

/// Caches a value in Redis.
async fn cache<T, Fut>(redis: &mut RedisConn, key: &str, fetch: Fut) -> AppResult<T>
where
    T: Serialize + DeserializeOwned,
    Fut: Future<Output = AppResult<T>>,
{
    let cached_json: Option<String> = redis.get(key).await?;
    if let Some(json) = cached_json {
        let value = serde_json::from_str(&json)?;
        Ok(value)
    } else {
        let value = fetch.await?;
        let json = serde_json::to_string(&value)?;
        redis.set_ex(key, &json, EXPIRES.as_secs() as usize).await?;
        Ok(value)
    }
}

/// The default expiry time for caches.
const EXPIRES: Duration = Duration::from_secs(60 * 60 * 24 * 7); // 7 days
