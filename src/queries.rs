pub(super) fn repos_overview(
    owned_cursor: Option<Box<str>>,
    contrib_cursor: Option<Box<str>>,
) -> Box<str> {
    let owned_cursor = owned_cursor.map_or_else(|| String::from("null"), |c| format!(r#""{}""#, c));
    let contrib_cursor =
        contrib_cursor.map_or_else(|| "null".to_owned(), |c| format!(r#""{}""#, c));

    format!(
        "{{
            viewer {{
                login, name,
                repositories(
                    first: 100,
                    orderBy: {{ field: UPDATED_AT, direction: DESC }},
                    isFork: false, after: {owned_cursor}
                ) {{
                    pageInfo {{ hasNextPage endCursor }}
                    nodes {{
                        nameWithOwner
                        stargazers {{ totalCount }}
                        forkCount
                        languages(first: 10, orderBy: {{ field: SIZE, direction: DESC }}) {{
                            edges {{ size node {{ name color }} }}
                        }}
                    }}
                }}
                repositoriesContributedTo(
                    first: 100,
                    includeUserRepositories: false, orderBy: {{ field: UPDATED_AT, direction: DESC }},
                    contributionTypes: [COMMIT, PULL_REQUEST, REPOSITORY, PULL_REQUEST_REVIEW],
                    after: {contrib_cursor}
                ) {{
                    pageInfo {{ hasNextPage endCursor }}
                    nodes {{
                        nameWithOwner
                        stargazers {{ totalCount }}
                        forkCount
                        languages(first: 10, orderBy: {{ field: SIZE, direction: DESC }}) {{
                            edges {{ size node {{ name color }} }}
                        }}
                    }}
                }}
            }}}}",
    )
    .into_boxed_str()
}
