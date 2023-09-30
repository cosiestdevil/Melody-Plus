use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, Rgba};
use querystring::stringify;
use rand::distributions::{Alphanumeric, DistString};
use sha2::{Digest, Sha256};
use urlencoding::encode;
#[derive(Debug, Clone)]
pub struct SpotifyClient {
    pub client_id: String,
    pub access_token: Option<Auth>,
    client: reqwest::Client,
}
#[derive(Debug, Clone)]
pub struct AuthenticateRequest {
    pub url: String,
    pub code_verifier: String,
    pub state: String,
}

impl<'a> SpotifyClient {
    pub fn new(client_id: String) -> Self {
        Self {
            client_id,
            access_token: Option::<Auth>::None,
            client: reqwest::Client::new(),
        }
    }
    pub fn create_authentication_request(
        &self,
        redirect_url: &str,
        scope: &str,
    ) -> AuthenticateRequest {
        let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let code_verifier = Alphanumeric.sample_string(&mut rand::thread_rng(), 128);
        let code_challenge_bytes = Sha256::digest(&code_verifier);
        let code_challenge = general_purpose::URL_SAFE_NO_PAD.encode(code_challenge_bytes);
        let query = stringify(vec![
            ("response_type", "code"),
            ("scope", &encode(scope)),
            ("redirect_uri", &encode(redirect_url)),
            ("code_challenge_method", "S256"),
            ("code_challenge", &encode(&code_challenge)),
            ("client_id", &encode(&self.client_id)),
            ("state", &encode(&state)),
        ]);
        let url = format!("https://accounts.spotify.com/authorize?{query}");
        let auth = AuthenticateRequest {
            url,
            code_verifier,
            state,
        };
        auth
    }
    pub async fn get_access_token(
        spotify: SpotifyClient,
        code: String,
        code_verifier: String,
        client_id: String,
    ) -> Result<Auth, Error> {
        spotify
            .get_access_token_internal(code, code_verifier, client_id)
            .await
    }
    async fn get_access_token_internal(
        &self,
        code: String,
        code_verifier: String,
        client_id: String,
    ) -> Result<Auth, Error> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", "https://melodyplus.azurewebsites.net"),
            ("client_id", &client_id),
            ("code_verifier", &code_verifier),
        ];
        let resp = self
            .client
            .post("https://accounts.spotify.com/api/token")
            .form(&params)
            .send()
            .await?;
        if resp.status() != 200 {
            Err(Error::APIError)
        } else {
            Ok(Auth::from(resp.json::<AuthResponse>().await?))
        }
    }
    async fn refresh_token(&self, refresh_token: String) -> Result<Auth, Error> {
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", &self.client_id),
            ("refresh_token", &refresh_token),
        ];
        let resp = self
            .client
            .post("https://accounts.spotify.com/api/token")
            .form(&params)
            .send()
            .await?;
        if resp.status() != 200 {
            Err(Error::APIError)
        } else {
            Ok(Auth::from(resp.json::<AuthResponse>().await?))
        }
    }
    pub async fn get_auth(spotify: SpotifyClient, state: String) -> Result<String, Error> {
        spotify.get_auth_internal(state).await
    }
    async fn get_auth_internal(&self, state: String) -> Result<String, Error> {
        let resp = self
            .client
            .get(format!(
                "https://melodyplus.azurewebsites.net/Get?state={state}"
            ))
            .send()
            .await?;
        if resp.status() != 200 {
            Err(Error::APIError)
        } else {
            Ok(resp.json::<String>().await?)
        }
    }
    pub async fn current_playback(mut spotify: SpotifyClient) -> Result<PlaybackResponse, Error> {
        spotify.current_playback_internal().await
    }
    async fn current_playback_internal(& mut self) -> Result<PlaybackResponse, Error> {
        let token = self.access_token.clone().ok_or(Error::NotAuthedError);
        if token.is_err() {
            Err(token.unwrap_err())
        } else {
            let mut auth = token?.clone();
            if &auth.expires_at >= &chrono::Utc::now().timestamp() {
                auth = self.refresh_token(auth.refresh_token).await?;
                self.access_token = Option::Some(auth.clone());
            }
            let resp = self
                .client
                .get("https://api.spotify.com/v1/me/player")
                .bearer_auth(auth.access_token)
                .send()
                .await?;
            if resp.status() != 200 {
                Err(Error::APIError)
            } else {
                Ok(resp.json::<PlaybackResponse>().await?)
            }
        }
    }

    pub async fn download_album(url: String) -> Result<AlbumDisplay, Error> {
        let bytes = &reqwest::get(url).await?.bytes().await?;
        let mut image = image::io::Reader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?
            .into_rgba8();
        let pallette = color_thief::get_palette(
            &image.clone().into_raw(),
            color_thief::ColorFormat::Rgba,
            1,
            2,
        )?;
        image = image::imageops::resize::<ImageBuffer<Rgba<u8>, Vec<u8>>>(
            &image,
            200,
            200,
            image::imageops::FilterType::Nearest,
        );
        let color = pallette[0];
        Ok(AlbumDisplay {
            image: image.clone(),
            color: iced::color!(color.r, color.g, color.b),
        })
    }
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Error {
    APIError,
    NotAuthedError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
impl From<image::ImageError> for Error {
    fn from(error: image::ImageError) -> Error {
        dbg!(error);

        Error::APIError
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
impl From<color_thief::Error> for Error {
    fn from(error: color_thief::Error) -> Error {
        dbg!(error);
        Error::APIError
    }
}
// impl From<dyn std::error::Error> for Error{
//     fn from(error: dyn std::error::Error) -> Error {
//         dbg!(error);

//         Error::APIError
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}
impl From<AuthResponse> for Auth {
    fn from(value: AuthResponse) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            expires_at: chrono::Utc::now().timestamp() + value.expires_in,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaybackResponse {
    pub repeat_state: String,
    pub shuffle_state: bool,
    pub progress_ms: i64,
    pub is_playing: bool,
    pub item: Track,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub duration_ms: i64,
    pub id: String,
    pub name: String,
    pub album: Album,
    pub artists: Vec<Artist>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub images: Vec<SpotifyImage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyImage {
    pub url: String,
    pub height: u32,
    pub width: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artist {
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct AlbumDisplay {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub color: iced::Color,
}
