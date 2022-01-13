use cursive::utils::markup::StyledString;
use url::Url;

pub struct Tab {
    title: String,
    url: Url,
    content: StyledString,
}
