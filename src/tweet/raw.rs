use crate::{place, user};
use chrono;
use serde::{Deserialize, Serialize};

use crate::common::serde_datetime;

use super::{
    deserialize_tweet_source, ExtendedTweetEntities, FilterLevel, Tweet, TweetEntities, TweetSource,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RawTweet {
    pub coordinates: Option<RawCoordinates>,
    #[serde(with = "serde_datetime")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub current_user_retweet: Option<CurrentUserRetweet>,
    pub display_text_range: Option<(usize, usize)>,
    pub entities: TweetEntities,
    pub extended_entities: Option<ExtendedTweetEntities>,
    pub extended_tweet: Option<RawExtendedTweet>,
    pub favorite_count: i32,
    pub favorited: Option<bool>,
    pub filter_level: Option<FilterLevel>,
    pub id: u64,
    pub in_reply_to_user_id: Option<u64>,
    pub in_reply_to_screen_name: Option<String>,
    pub in_reply_to_status_id: Option<u64>,
    pub lang: Option<String>,
    pub place: Option<place::Place>,
    pub possibly_sensitive: Option<bool>,
    pub quoted_status_id: Option<u64>,
    pub quoted_status: Option<Box<Tweet>>,
    pub retweet_count: i32,
    pub retweeted: Option<bool>,
    pub retweeted_status: Option<Box<Tweet>>,
    #[serde(deserialize_with = "deserialize_tweet_source")]
    pub source: Option<TweetSource>,
    pub text: Option<String>,
    pub full_text: Option<String>,
    pub truncated: bool,
    pub user: Option<Box<user::TwitterUser>>,
    #[serde(default)]
    pub withheld_copyright: bool,
    pub withheld_in_countries: Option<Vec<String>>,
    pub withheld_scope: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RawExtendedTweet {
    pub full_text: String,
    pub display_text_range: Option<(usize, usize)>,
    pub entities: TweetEntities,
    pub extended_entities: Option<ExtendedTweetEntities>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RawCoordinates {
    #[serde(rename = "type")]
    pub kind: String,
    pub coordinates: (f64, f64),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct CurrentUserRetweet {
    pub id: u64,
}
