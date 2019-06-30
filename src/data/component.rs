use futures::compat::*;
use lsp_types::{MarkupContent, MarkupKind, Uri};
use reqwest::r#async::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ComponentDocumentation {
    pub caption: String,
    pub content: MarkupContent,
}

impl ComponentDocumentation {
    pub async fn lookup(name: &str) -> Option<ComponentDocumentation> {
        let uri: Uri = format!("http://ctan.org/json/2.0/pkg/{}", name)
            .parse()
            .ok()?;

        let client = Client::new();
        let mut response = client.get(uri).send().compat().await.ok()?;
        let component: Component = response.json().compat().await.ok()?;
        if component.errors.is_some() {
            return None;
        }

        let description = component
            .descriptions
            .iter()
            .find(|description| description.language.is_none());

        if let Some(description) = description {
            Some(ComponentDocumentation {
                caption: component.caption,
                content: MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: html2md::parse_html(&description.text).into(),
                },
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Component {
    name: String,
    caption: String,
    descriptions: Vec<ComponentDescription>,
    errors: Option<serde_json::Value>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
struct ComponentDescription {
    language: Option<String>,
    text: String,
}
