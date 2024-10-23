use reqwest::{StatusCode, Url};

#[derive(Debug)]
pub(crate) enum FastPath {
    UpToDate,
    NeedsFetch,
    Indeterminate,
}

/// extract username & repository from a fetch URL, only if it's on Github.
fn user_and_repo_from_url_if_github(fetch_url: &gix::Url) -> Option<(String, String)> {
    let url = Url::parse(&fetch_url.to_string()).ok()?;
    if !(url.host_str() == Some("github.com")) {
        return None;
    }

    // This expects GitHub urls in the form `github.com/user/repo` and nothing
    // else
    let mut pieces = url.path_segments()?;
    let username = pieces.next()?;
    let repository = pieces.next()?;
    let repository = repository.strip_suffix(".git").unwrap_or(repository);
    if pieces.next().is_some() {
        return None;
    }
    Some((username.to_string(), repository.to_string()))
}

/// use github fast-path to check if the repository has any changes
/// since the last seen reference.
///
/// To save server side resources on github side, we can use an API
/// to check if there are any changes in the repository before we
/// actually run `git fetch`.
///
/// On non-github fetch URLs we don't do anything and always run the fetch.
///
/// Code gotten and adapted from
/// https://github.com/rust-lang/cargo/blob/edd36eba5e0d6e0cfcb84bd0cc651ba8bf5e7f83/src/cargo/sources/git/utils.rs#L1396
///
/// GitHub documentation:
/// https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#get-a-commit
/// specifically using `application/vnd.github.sha`
pub(crate) fn has_changes(
    fetch_url: &gix::Url,
    last_seen_reference: &gix::ObjectId,
    branch_name: &str,
) -> Result<FastPath, reqwest::Error> {
    let (username, repository) = match user_and_repo_from_url_if_github(fetch_url) {
        Some(url) => url,
        None => return Ok(FastPath::Indeterminate),
    };

    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        username, repository, branch_name,
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("crates-index-diff")
        .build()?;
    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github.sha")
        .header("If-None-Match", format!("\"{}\"", last_seen_reference))
        .send()?;

    let status = response.status();
    if status == StatusCode::NOT_MODIFIED {
        Ok(FastPath::UpToDate)
    } else if status.is_success() {
        Ok(FastPath::NeedsFetch)
    } else {
        // Usually response_code == 404 if the repository does not exist, and
        // response_code == 422 if exists but GitHub is unable to resolve the
        // requested rev.
        Ok(FastPath::Indeterminate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_github_http_url() {
        let (user, repo) = user_and_repo_from_url_if_github(
            &gix::Url::try_from("https://github.com/some_user/some_repo.git").unwrap(),
        )
        .unwrap();
        assert_eq!(user, "some_user");
        assert_eq!(repo, "some_repo");
    }

    #[test]
    fn test_github_ssh_url() {
        let (user, repo) = user_and_repo_from_url_if_github(
            &gix::Url::try_from("ssh://git@github.com/some_user/some_repo.git").unwrap(),
        )
        .unwrap();
        assert_eq!(user, "some_user");
        assert_eq!(repo, "some_repo");
    }

    #[test]
    fn test_non_github_url() {
        assert!(user_and_repo_from_url_if_github(
            &gix::Url::try_from("https://not_github.com/some_user/some_repo.git").unwrap(),
        )
        .is_none());
    }
}
