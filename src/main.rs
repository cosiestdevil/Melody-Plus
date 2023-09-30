mod spotify;
use iced::theme::Theme;
use iced::widget::{button, column, container, image as iimage, progress_bar, qr_code, row, text};
use iced::{window, Alignment, Application, Command, Element, Length, Settings};
use spotify::{AlbumDisplay, Auth, AuthenticateRequest, Error, PlaybackResponse, SpotifyClient};
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use tokio::time::{sleep, Duration};

struct Counter {
    spotify: SpotifyClient,
    auth_request: AuthenticateRequest,
    playback: Option<PlaybackResponse>,
    album: String,
    album_display: Option<AlbumDisplay>,
    qr_state: Result<qr_code::State, iced::widget::qr_code::Error>,
}
#[derive(Debug, Clone)]
pub enum Message {
    Close,
    PollForAuth(()),
    PollPlayback(()),
    AuthRecieved(Result<String, Error>),
    TokenRecieved(Result<Auth, Error>),
    PlaybackRecieved(Result<PlaybackResponse, Error>),
    AlbumRecieved(Result<AlbumDisplay, Error>),
}

impl Application for Counter {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Counter, iced::Command<Message>) {
        let client_id: &'static str = env!("MELODY_PLUS_CLIENT_ID");
        let mut spotify = SpotifyClient::new(String::from(client_id));
        let auth_request = spotify.create_authentication_request(
            "https://melodyplus.azurewebsites.net",
            "user-read-playback-state user-modify-playback-state",
        );
        let url = auth_request.url.clone();
        match std::fs::read_to_string("./token.json") {
            Ok(json)=>{
                let auth = serde_json::from_str::<Auth>(json.as_str()).expect("");
                spotify.access_token = Option::Some(auth);
                (Self {
                    spotify,
                    auth_request,
                    playback: Option::None,
                    album: String::new(),
                    album_display: Option::None,
                    qr_state: qr_code::State::new(url),
                },Command::perform(sleep(Duration::from_millis(0)), Message::PollPlayback))
            },
            Err(_)=>{                
                (
                    Self {
                        spotify,
                        auth_request,
                        playback: Option::None,
                        album: String::new(),
                        album_display: Option::None,
                        qr_state: qr_code::State::new(url),
                    },
                    Command::perform(sleep(Duration::from_millis(5000)), Message::PollForAuth),
                )
            }
        }
        
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }
    fn title(&self) -> String {
        String::from("MelodyPlus - Iced")
    }
    fn view(&self) -> Element<Message> {
        let color = match &self.album_display {
            Some(album) => album.color,
            None => iced::Color::from_rgb8(30, 215, 96),
        };
        let element: Element<Message> = match &self.spotify.access_token {
            Some(auth) => match &self.playback {
                None => column![text("I have an access token"), text(&auth.access_token)]
                    .padding(20)
                    .align_items(Alignment::Center)
                    .into(),
                Some(playback) => {
                    let progress = chrono::DateTime::UNIX_EPOCH
                        + chrono::Duration::milliseconds(playback.progress_ms);
                    let total_duration = chrono::DateTime::UNIX_EPOCH
                        + chrono::Duration::milliseconds(playback.item.duration_ms);
                    let image = match &self.album_display {
                        Some(album) => iimage(iimage::Handle::from_pixels(
                            album.image.width(),
                            album.image.height(),
                            album.image.clone().into_raw(),
                        )),
                        None => iimage(iimage::Handle::from_path("./logo.png")),
                    };

                    column![
                        row![
                            column![image],
                            column![
                                text(&playback.item.name).size(48),
                                text(
                                    &playback
                                        .item
                                        .artists
                                        .iter()
                                        .map(|x| x.name.clone())
                                        .collect::<Vec<String>>()
                                        .join(", ")
                                ),
                                text(&playback.item.album.name),
                                column![text(format!(
                                    "{}/{}",
                                    progress.format("%M:%S"),
                                    total_duration.format("%M:%S")
                                ))]
                                .align_items(Alignment::Center),
                                progress_bar(
                                    0.0..=1.0,
                                    ((playback.progress_ms as f64)
                                        / (playback.item.duration_ms as f64))
                                        as f32
                                )
                                .style(move |_: &_| {
                                    progress_bar::Appearance {
                                        bar: iced::Background::Color(color),
                                        background: iced::Background::Color(
                                            iced::Color::from_rgba(color.r, color.g, color.b, 0.3),
                                        ),
                                        border_radius: iced::BorderRadius::from(0.0),
                                    }
                                })
                            ]
                        ] //,
                          // row![button("Play/Pause")]
                    ]
                    .padding(20)
                    .align_items(Alignment::Center)
                    .into()
                }
            },
            None => {
                let state = self.qr_state.as_ref().expect("");
                column![qr_code::QRCode::new(state)].into()
            }
        };
        column![
            container(row![
                container(text("MelodyPlus")).width(Length::Fill),
                button("x").on_press(Message::Close)
            ])
            .style(move |_: &_| {
                container::Appearance {
                    background: Option::Some(iced::Background::Color(color)),
                    ..Default::default()
                }
            })
            .width(Length::Fill),
            element
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PollForAuth(()) => {
                let auth = self.auth_request.clone();
                let spotify = self.spotify.clone();
                let action = SpotifyClient::get_auth(spotify, auth.state);
                Command::perform(action, Message::AuthRecieved)
            }
            Message::AuthRecieved(Ok(response)) => {
                let auth = self.auth_request.clone();
                let spotify = self.spotify.clone();
                let action = SpotifyClient::get_access_token(
                    spotify,
                    response,
                    auth.code_verifier,
                    self.spotify.client_id.clone(),
                );
                Command::perform(action, Message::TokenRecieved)
            }
            Message::AuthRecieved(Err(_)) => {
                Command::perform(sleep(Duration::from_millis(5000)), Message::PollForAuth)
            }
            Message::TokenRecieved(Ok(response)) => {
                let mut file = File::create("./token.json").expect("");
                file.write_all(serde_json::json!(response).to_string().as_bytes())
                    .expect("");
                self.spotify.access_token = Option::Some(response);
                Command::perform(sleep(Duration::from_millis(500)), Message::PollPlayback)
            }
            Message::TokenRecieved(Err(_)) => {
                Command::perform(sleep(Duration::from_millis(5000)), Message::PollForAuth)
            }
            Message::PollPlayback(()) => {
                let action = SpotifyClient::current_playback(self.spotify.clone());
                Command::perform(action, Message::PlaybackRecieved)
            }
            Message::PlaybackRecieved(Ok(playback)) => {
                let album = playback.clone().item.album.clone();
                self.playback = Option::Some(playback);
                if self.album != album.id {
                    self.album = album.id;
                    let image = match album.images.first() {
                        Some(image) => image.clone(),
                        None => todo!(),
                    };
                    let action = SpotifyClient::download_album(image.url);
                    Command::perform(action, Message::AlbumRecieved)
                } else {
                    Command::perform(sleep(Duration::from_millis(1000)), Message::PollPlayback)
                }
            }
            Message::PlaybackRecieved(Err(_)) => {
                self.playback = Option::None;
                self.album = String::new();
                self.album_display = Option::None;
                Command::perform(sleep(Duration::from_millis(5000)), Message::PollPlayback)
            }
            Message::AlbumRecieved(Ok(album)) => {
                self.album_display = Option::Some(album);
                Command::perform(sleep(Duration::from_millis(500)), Message::PollPlayback)
            }
            Message::AlbumRecieved(Err(_)) => todo!(),
            Message::Close => std::process::exit(0),
        }
    }
}

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (800, 480),
            resizable: false,
            decorations: false,
            ..Default::default()
        },
        ..Default::default()
    };
    Counter::run(settings)
}
