// Vigil
//
// Microservices Status Page
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use url_serde::SerdeUrl;

use APP_CONF;

const LOGO_EXTENSION_SPLIT_SPAN: usize = 4;

lazy_static! {
    pub static ref INDEX_CONFIG: IndexContextConfig = IndexContextConfig {
        page_title: APP_CONF.branding.page_title.to_owned(),
        company_name: APP_CONF.branding.company_name.to_owned(),
        icon_color: APP_CONF.branding.icon_color.to_owned(),
        icon_url: APP_CONF.branding.icon_url.to_owned(),
        icon_mime: ImageMime::guess_from(APP_CONF.branding.icon_url.as_str()),
        logo_color: APP_CONF.branding.logo_color.to_owned(),
        logo_url: APP_CONF.branding.logo_url.to_owned(),
        website_url: APP_CONF.branding.website_url.to_owned(),
        support_url: APP_CONF.branding.support_url.to_owned(),
        custom_html: APP_CONF.branding.custom_html.to_owned(),
    };
}

#[derive(Serialize)]
pub enum Status {
    #[serde(rename = "healthy")]
    Healthy,

    #[serde(rename = "sick")]
    Sick,

    #[serde(rename = "dead")]
    Dead,
}

#[derive(Serialize)]
pub enum ProbeType {
    #[serde(rename = "passive")]
    Passive,

    #[serde(rename = "active")]
    Active,
}

#[derive(Serialize)]
pub enum ImageMime {
    #[serde(rename = "image/png")]
    ImagePNG,

    #[serde(rename = "image/jpeg")]
    ImageJPEG,

    #[serde(rename = "image/gif")]
    ImageGIF,

    #[serde(rename = "image/svg")]
    ImageSVG,
}

impl ImageMime {
    fn guess_from(logo_url: &str) -> ImageMime {
        if logo_url.len() > LOGO_EXTENSION_SPLIT_SPAN {
            let (_, logo_url_extension) = logo_url.split_at(
                logo_url.len() - LOGO_EXTENSION_SPLIT_SPAN
            );

            match logo_url_extension {
                ".svg" => ImageMime::ImageSVG,
                ".jpg" => ImageMime::ImageJPEG,
                ".gif" => ImageMime::ImageGIF,
                _ => ImageMime::ImagePNG,
            }
        } else {
            ImageMime::ImagePNG
        }
    }
}

#[derive(Serialize)]
pub struct IndexContext<'a> {
    pub status: Status,
    pub refreshed_at: String,
    pub groups: Vec<IndexContextGroup>,
    pub config: &'a IndexContextConfig,
}

#[derive(Serialize)]
pub struct IndexContextConfig {
    pub page_title: String,
    pub company_name: String,
    pub icon_color: String,
    pub icon_url: SerdeUrl,
    pub icon_mime: ImageMime,
    pub logo_color: String,
    pub logo_url: SerdeUrl,
    pub website_url: SerdeUrl,
    pub support_url: SerdeUrl,
    pub custom_html: Option<String>,
}

#[derive(Serialize)]
pub struct IndexContextGroup {
    status: Status,
    nodes: Vec<IndexContextGroupNode>,
}

#[derive(Serialize)]
pub struct IndexContextGroupNode {
    name: String,
    status: Status,
    probe_type: ProbeType,
}
