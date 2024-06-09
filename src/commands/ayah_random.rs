use crate::Data;

use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomVerse {
    pub verse: Verse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verse {
    pub id: i64,
    #[serde(rename = "verse_number")]
    pub verse_number: i64,
    #[serde(rename = "verse_key")]
    pub verse_key: String,
    #[serde(rename = "hizb_number")]
    pub hizb_number: i64,
    #[serde(rename = "rub_el_hizb_number")]
    pub rub_el_hizb_number: i64,
    #[serde(rename = "ruku_number")]
    pub ruku_number: i64,
    #[serde(rename = "manzil_number")]
    pub manzil_number: i64,
    #[serde(rename = "page_number")]
    pub page_number: i64,
    #[serde(rename = "juz_number")]
    pub juz_number: i64,
    pub translations: Vec<Translation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    pub id: i64,
    #[serde(rename = "resource_id")]
    pub resource_id: i64,
    pub text: String,
}

impl RandomVerse {
    async fn get() -> Result<Self, Error> {
        let url = format!(
            "https://api.quran.com/api/v4/verses/random?language=id&translations=131"
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<RandomVerse>().await?;

        Ok(res)
    }
}

fn concatenate_texts(translations: &[Translation]) -> String {
    translations
        .iter()
        .map(|t| t.text.as_str())
        .collect::<Vec<&str>>()
        .join("")
}

#[poise::command(slash_command, description_localized("en-US","siraman rohani"))]
pub async fn ayah_random(ctx: Context<'_>) -> Result<(), Error> {

    let res = RandomVerse::get().await?;

    ctx.reply(format!("Hello **{}** \n\n  Quran {}\n {}", ctx.author().name, res.verse.verse_key ,concatenate_texts(&res.verse.translations))).await?;
    Ok(())
}