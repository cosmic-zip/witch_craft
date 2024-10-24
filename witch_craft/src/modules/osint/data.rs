/// name - url - filter
pub const META_LINKS: [(&str, &str, &str); 18] = [
    ("youtube", "https://www.youtube.com/@@@keyword", ""),
    ("gitlab", "https://gitlab.com/@@keyword", ""),
    ("github", "https://github.com/@@keyword", ""),
    ("bitbucket", "https://bitbucket.org/@@keyword", ""),
    (
        "facebook",
        "https://facebook.com/@@keyword",
        "This content isn't available right now",
    ),
    ("slideshare", "https://slideshare.net/@@keyword", ""),
    (
        "linkedin.corp",
        "https://linkedin.com/company/@@keyword",
        "",
    ),
    ("linkedin.user", "https://linkedin.com/in/@@keyword", ""),
    ("myspace", "https://myspace.com/@@keyword", ""),
    (
        "instagram",
        "https://instagram.com/@@keyword",
        "Sorry, this page isn't available.",
    ),
    (
        "medium",
        "https://medium.com/@@@keyword",
        "Out of nothing, something.",
    ),
    (
        "twitch",
        "https://twitch.tv/@@keyword",
        "Sorry. Unless you've got a time machine, that content is unavailable.",
    ),
    ("mastodon", "https://mastodon.social/@@@keyword", ""),
    (
        "bsky",
        "https://bsky.app/profile/@@keyword",
        "Error: handle must be a valid handle",
    ),
    ("reddit", "https://www.reddit.com/user/@@keyword", ""),
    (
        "twitter",
        "https://www.x.com/@@keyword",
        "This account doesn’t exist",
    ),
    (
        "xvideos",
        "https://www.xvideos.com/profiles/@@keyword",
        "THIS PROFILE DOESN'T EXIST !",
    ),
    (
        "snapchat",
        "https://www.snapchat.com/add/@@keyword",
        "This content was not found",
    ),
];
