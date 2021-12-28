use serde::{Deserialize, Serialize};
use crate::Error;
use handlebars::Handlebars;
use crate::document::Spec;

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecIndexContext {
    pub specs: Vec<SpecItem>,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecRedirectContext {
    pub spec: SpecItem,
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecItem {
    pub id: String,
    pub url: String,
    pub description: String,
    pub discuss: String,
}

pub fn index_layout(
    specs: &[Spec],
    handlebars: &Handlebars,
) -> Result<String, Error> {
    let context = SpecIndexContext {
        specs: specs.iter().map(|spec| SpecItem {
            id: spec.id.clone(),
            url: spec.url.clone(),
            description: spec.description.clone(),
            discuss: spec.discuss.clone(),
        }).collect(),
    };

    let layouted = handlebars.render("spec/index", &context)?;

    Ok(layouted)
}

pub fn redirect_layout(
    spec: &Spec,
    handlebars: &Handlebars,
) -> Result<String, Error> {
    let context = SpecRedirectContext {
        spec: SpecItem {
            id: spec.id.clone(),
            url: spec.url.clone(),
            description: spec.description.clone(),
            discuss: spec.discuss.clone(),
        },
    };

    let layouted = handlebars.render("spec/redirect", &context)?;

    Ok(layouted)
}
