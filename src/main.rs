#[macro_use] extern crate log;
extern crate simplelog;

use std::fs::File;
use simplelog::*;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Result;
use color::{Hsv, ToHsv};
use image::RgbaImage;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use rgb::{RGB, RGB8};
use serde::{Deserialize, Serialize};
use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::font::{Font, TextAlignment, TextLayout, TextOptions};
use speedy2d::image::{ImageDataType, ImageSmoothingMode};
use speedy2d::shape::Rectangle;
use speedy2d::window::{MouseButton, UserEventSender, WindowCreationOptions, WindowHandler, WindowHelper, WindowSize, WindowStartupInfo};


use crate::spotify::QueueObjectCurrentlyPlaying;

mod spotify;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> Result<()> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(format!("{NAME}.log")).unwrap()),
        ]
    ).unwrap();
    let window: Window<DisplayData> = Window::new_with_user_events("Title", WindowCreationOptions::new_windowed(
        WindowSize::PhysicalPixels(UVec2::new(800, 480)),
        None,
    )).unwrap();
    let user_event_sender = window.create_user_event_sender();
    window.run_loop(MyWindow { events: user_event_sender, data: DisplayData::default() });
}

#[derive(Clone, Debug)]
struct DisplayData {
    track: String,
    artist: String,
    album: String,
    accent: RGB<u8>,
    image: RgbaImage,
    track_length: Duration,
    current_progress: Duration,
    playing: bool,
    shuffle: bool,
    repeat: RepeatState,
    actions: spotify::DisallowsObject,
    mouse: (f32, f32),
    track_offset: f32,
    token: RefreshResponse,
    cover_position: ((f32, f32), (f32, f32)),
    progress_position: ((f32, f32), (f32, f32)),
    play_button_position: ((f32, f32), (f32, f32)),
    prev_button_position: ((f32, f32), (f32, f32)),
    next_button_position: ((f32, f32), (f32, f32)),
    repeat_button_position: ((f32, f32), (f32, f32)),
    shuffle_button_position: ((f32, f32), (f32, f32)),
}

impl Default for DisplayData {
    fn default() -> Self {
        let button_offset = 125.0;
        DisplayData {
            track: Default::default(),
            album: Default::default(),
            artist: Default::default(),
            accent: Default::default(),
            image: Default::default(),
            track_length: Default::default(),
            current_progress: Default::default(),
            playing: Default::default(),
            shuffle: Default::default(),
            repeat: Default::default(),
            actions: Default::default(),
            mouse: Default::default(),
            track_offset:Default::default(),
            token: Default::default(),
            cover_position: ((25.0, 25.0), (225.0, 225.0)),
            progress_position: ((25.0, 250.0), (775.0, 300.0)),
            shuffle_button_position: ((25.0 + button_offset, 325.0), (100.0 + button_offset, 400.0)),
            prev_button_position: ((125.0 + button_offset, 325.0), (200.0 + button_offset, 400.0)),
            play_button_position: ((225.0 + button_offset, 325.0), (300.0 + button_offset, 400.0)),
            next_button_position: ((325.0 + button_offset, 325.0), (400.0 + button_offset, 400.0)),
            repeat_button_position: ((425.0 + button_offset, 325.0), (500.0 + button_offset, 400.0)),
        }
    }
}

struct MyWindow {
    events: UserEventSender<DisplayData>,
    data: DisplayData,
}

impl WindowHandler<DisplayData> for MyWindow {
    fn on_start(&mut self, helper: &mut WindowHelper<DisplayData>, _info: WindowStartupInfo) {
        let user_event_sender = self.events.clone();
        let client = Client::builder().build().unwrap();
        let refresh = include_str!("refresh.token");
        let token = Arc::new(Mutex::new(get_token(&client, refresh.into()).unwrap()));
        let poll_token = token.clone();
        let poll_client = client.clone();
        let refresh_token = token.clone();
        let refresh_client = client.clone();
        let image = image::io::Reader::new(std::io::Cursor::new(include_bytes!("../logo.ico")))
            .with_guessed_format().unwrap()
            .decode().unwrap()
            .into_rgba8();
        let icon_size = (image.width(), image.height());
        warn!("icon_size:{icon_size:?}");
        helper.set_icon_from_rgba_pixels(image.into_raw(), icon_size).unwrap();
        let _poll_thread = std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_millis(500));

                let mut_data = poll(poll_token.lock().unwrap().deref(),&poll_client);
                match mut_data{
                    Ok(mut_data)=>user_event_sender.send_event(mut_data.clone()).unwrap(),
                    Err(err)=>error!("{err:?}")
                };
            };
        });
        let _refresh_thread: std::thread::JoinHandle<Result<()>> = std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(60 * 30));
                let mut token = refresh_token.lock().unwrap();
                let new_token = get_token(&refresh_client, refresh.into());
                match new_token{
                    Ok(new_token) => {
                        info!("Got New Token");
                        *token = new_token},
                    Err(err)=>error!("{err:?}")
                }
            }
        });
    }
    fn on_user_event(&mut self, helper: &mut WindowHelper<DisplayData>, user_event: DisplayData) {
        let mut new_data = DisplayData { mouse: self.data.mouse,track_offset:self.data.track_offset, ..user_event };
        if new_data.track!=self.data.track{
            new_data.track_offset = -50.0;
        }
        self.data = new_data;
        helper.request_redraw();
    }
    fn on_draw(&mut self, helper: &mut WindowHelper<DisplayData>, graphics: &mut Graphics2D) {
        let bytes = include_bytes!("RobotoMedium.ttf");
        let font = Font::new(bytes).unwrap();

        let bytes = include_bytes!("NotoEmoji-Regular.ttf");
        let unifont = Font::new(bytes).unwrap();
        let accent = Color::from_int_rgb(self.data.accent.r, self.data.accent.g, self.data.accent.b);

        graphics.clear_screen(Color::from_int_rgb(1, 4, 9));
        let cover_rect = Rectangle::from_tuples(self.data.cover_position.0, self.data.cover_position.1);
        let cover_image = graphics.create_image_from_raw_pixels(ImageDataType::RGBA, ImageSmoothingMode::Linear, (self.data.image.width(), self.data.image.height()), self.data.image.as_raw().as_slice()).unwrap();
        graphics.draw_rectangle_image(cover_rect, &cover_image);
        helper.set_title(format!("{} - {} - {} {}", self.data.track.clone(), self.data.artist.clone(), NAME, VERSION));
        let track = font.layout_text(&*self.data.track.clone(), 32.0, TextOptions::new());
        let track_origin = (self.data.cover_position.1.0 + 25.0, 25.0);
        let track_window = Rectangle::from_tuples(track_origin,(track_origin.0+525.0,track_origin.1+32.0));

        if track.width() > 525.0{
            let m_width = font.layout_text("m",32.0,TextOptions::new()).width();
            graphics.draw_text_cropped((track_origin.0-self.data.track_offset.clamp(0.0,track.width()-525.0),track_origin.1),track_window,accent,&track);
            self.data.track_offset+=m_width/60.0;
            if self.data.track_offset > (track.width()-525.0)+50.0{
                self.data.track_offset = -50.0;
            }
            helper.request_redraw();
        }else{
            graphics.draw_text(track_origin, accent, &track);
        }
        let album = font.layout_text(&*self.data.album.clone(), 24.0, TextOptions::new());
        graphics.draw_text((self.data.cover_position.1.0 + 25.0, 75.0), accent, &album);
        let artist = font.layout_text(&*self.data.artist.clone(), 24.0, TextOptions::new());
        graphics.draw_text((self.data.cover_position.1.0 + 25.0, 105.0), accent, &artist);

        let progress_bar_back = Rectangle::from_tuples(self.data.progress_position.0, self.data.progress_position.1);
        let progress_bar_back_color = Color::from_rgba(accent.r(), accent.g(), accent.b(), 0.5);
        graphics.draw_rectangle(progress_bar_back, progress_bar_back_color);
        let progress = (self.data.current_progress.as_millis() as f64 / self.data.track_length.as_millis() as f64) as f32;
        let progress_bar = Rectangle::from_tuples(self.data.progress_position.0, (25.0 + (progress * 750.0), self.data.progress_position.1.1));
        graphics.draw_rectangle(progress_bar, accent);
        let progress_text = format!("{:0>2}:{:0>2}/{:0>2}:{:0>2}", self.data.current_progress.as_secs() / 60, self.data.current_progress.as_secs() % 60, self.data.track_length.as_secs() / 60, self.data.track_length.as_secs() % 60);
        let progress_text = font.layout_text(&*progress_text, 32.0, TextOptions::new().with_wrap_to_width(750.0, TextAlignment::Center));
        let mut progress_text_color = Color::WHITE;
        if accent.subjective_brightness() >= 0.5 {
            progress_text_color = Color::BLACK;
        }
        graphics.draw_text((self.data.progress_position.0.0, self.data.progress_position.0.1 + 8.0), progress_text_color, &progress_text);

        draw_button(&self.data.play_button_position, &unifont, Color::TRANSPARENT, accent, "\u{23EF}", graphics);

        draw_button(&self.data.prev_button_position, &unifont, Color::TRANSPARENT, accent, "\u{23EE}", graphics);

        draw_button(&self.data.next_button_position, &unifont, Color::TRANSPARENT, accent, "\u{23ED}", graphics);

        match &self.data.repeat {
            RepeatState::Track => draw_button(&self.data.repeat_button_position, &unifont, Color::TRANSPARENT, accent, "\u{1F502}", graphics),
            RepeatState::Context => draw_button(&self.data.repeat_button_position, &unifont, Color::TRANSPARENT, accent, "\u{1F501}", graphics),
            RepeatState::Off => draw_button(&self.data.repeat_button_position, &unifont, Color::TRANSPARENT, progress_bar_back_color, "\u{1F501}", graphics)
        };
        match &self.data.shuffle {
            true => draw_button(&self.data.shuffle_button_position, &unifont, Color::TRANSPARENT, accent, "\u{1F500}", graphics),
            false => draw_button(&self.data.shuffle_button_position, &unifont, Color::TRANSPARENT, progress_bar_back_color, "\u{1F500}", graphics)
        }
    }
    fn on_touch(&mut self, helper: &mut WindowHelper<DisplayData>, position: Vec2) {
        self.data.mouse = (position.x, position.y);
        self.on_mouse_button_down(helper, MouseButton::Left);
    }
    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<DisplayData>, position: Vec2) {
        self.data.mouse = (position.x, position.y);
    }
    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<DisplayData>, button: MouseButton) {
        if button == MouseButton::Left {
            let top_left = self.data.play_button_position.0;
            let bottom_right = self.data.play_button_position.1;
            let x_range = top_left.0..bottom_right.0;
            let y_range = top_left.1..bottom_right.1;
            if x_range.contains(&self.data.mouse.0) && y_range.contains(&self.data.mouse.1) {
                let client = Client::builder().build().unwrap();
                if self.data.playing {
                    let _ = client.put("https://api.spotify.com/v1/me/player/pause").bearer_auth(self.data.token.access_token.clone()).send();
                    println!("Pause");
                } else {
                    let _ = client.put("https://api.spotify.com/v1/me/player/play").bearer_auth(self.data.token.access_token.clone()).send();
                    println!("Play");
                }
                return;
            }
            let top_left = self.data.prev_button_position.0;
            let bottom_right = self.data.prev_button_position.1;
            let x_range = top_left.0..bottom_right.0;
            let y_range = top_left.1..bottom_right.1;
            if x_range.contains(&self.data.mouse.0) && y_range.contains(&self.data.mouse.1) {
                let client = Client::builder().build().unwrap();
                let _ = client.post("https://api.spotify.com/v1/me/player/previous").bearer_auth(self.data.token.access_token.clone()).send();
                println!("Previous");
                return;
            }
            let top_left = self.data.next_button_position.0;
            let bottom_right = self.data.next_button_position.1;
            let x_range = top_left.0..bottom_right.0;
            let y_range = top_left.1..bottom_right.1;
            if x_range.contains(&self.data.mouse.0) && y_range.contains(&self.data.mouse.1) {
                let client = Client::builder().build().unwrap();
                let _ = client.post("https://api.spotify.com/v1/me/player/next").bearer_auth(self.data.token.access_token.clone()).send();
                println!("Next");
                return;
            }
            let top_left = self.data.repeat_button_position.0;
            let bottom_right = self.data.repeat_button_position.1;
            let x_range = top_left.0..bottom_right.0;
            let y_range = top_left.1..bottom_right.1;
            if x_range.contains(&self.data.mouse.0) && y_range.contains(&self.data.mouse.1) {
                let new_repeat = match &self.data.repeat {
                    RepeatState::Off => "context",
                    RepeatState::Track => "off",
                    RepeatState::Context => "track"
                };
                let client = Client::builder().build().unwrap();
                let _ = client.put(format!("https://api.spotify.com/v1/me/player/repeat?state={}", new_repeat)).bearer_auth(self.data.token.access_token.clone()).send();
                println!("Repeat");
                return;
            }
            let top_left = self.data.shuffle_button_position.0;
            let bottom_right = self.data.shuffle_button_position.1;
            let x_range = top_left.0..bottom_right.0;
            let y_range = top_left.1..bottom_right.1;
            if x_range.contains(&self.data.mouse.0) && y_range.contains(&self.data.mouse.1) {
                let new_shuffle = match &self.data.shuffle {
                    true => "false",
                    false => "true",
                };
                let client = Client::builder().build().unwrap();
                let _ = client.put(format!("https://api.spotify.com/v1/me/player/shuffle?state={}", new_shuffle)).bearer_auth(self.data.token.access_token.clone()).send();
                println!("Shuffle");
                return;
            }
        }
    }
}

fn draw_button(position: &((f32, f32), (f32, f32)), font: &Font, back: Color, front: Color, text: &str, graphics: &mut Graphics2D) {
    let rect = Rectangle::from_tuples(position.0, position.1);
    graphics.draw_rectangle(&rect, back);
    let play_pause_text = font.layout_text(text, rect.height(), TextOptions::new().with_wrap_to_width(rect.width(), TextAlignment::Center));
    graphics.draw_text((position.0.0, position.0.1), front, &play_pause_text);
}

fn get_token(client: &Client, refresh: String) -> Result<RefreshResponse> {
    Ok(client
        .post("https://accounts.spotify.com/api/token")
        .form(&RefreshBody { grant_type: "refresh_token".to_owned(), refresh_token: refresh, client_id: "".to_owned() })
        .basic_auth(include_str!("client.id"), Some(include_str!("client.secret")))
        .send()?.json::<RefreshResponse>()?)

}

fn poll(poll_token: &RefreshResponse, poll_client: &Client) ->Result<DisplayData>{
    let token = poll_token;
    let playback_state = poll_client
        .get("https://api.spotify.com/v1/me/player")
        .bearer_auth(token.access_token.clone())
        .send()?;
    let mut_data = &mut DisplayData::default();
    mut_data.token = token.clone();
    if playback_state.status() != StatusCode::NO_CONTENT {
        let playback_state = playback_state.json::<Option<spotify::CurrentlyPlayingContextObject>>()?;

        if let Some(playback_state) = playback_state {
            if let Some(progress) = playback_state.progress_ms {
                mut_data.current_progress = Duration::from_millis(progress.unsigned_abs().into());
            }
            mut_data.actions = *playback_state.actions.unwrap_or_default();
            mut_data.repeat = playback_state.repeat_state.into();
            mut_data.shuffle = playback_state.shuffle_state.unwrap_or(false);
            mut_data.playing = playback_state.is_playing.unwrap_or(false);
            if let Some(item) = playback_state.item {
                match *item {
                    QueueObjectCurrentlyPlaying::TrackObject(track) => {
                        if let Some(track_name) = track.name {
                            if mut_data.track != track_name {
                                mut_data.track = track_name;
                                if let Some(duration) = track.duration_ms {
                                    mut_data.track_length = Duration::from_millis(duration.unsigned_abs().into());
                                }
                                if let Some(artists) = track.artists {
                                    let artists: Vec<String> = artists.iter().map(|a| a.name.clone().unwrap()).collect();
                                    mut_data.artist = artists.join(", ");
                                }
                                if let Some(album) = track.album {
                                    if mut_data.album != album.name {
                                        mut_data.album = album.name;
                                        if let Some(cover) = album.images.first() {
                                            let bytes = poll_client.get(&cover.url).send()?.bytes()?;
                                            let image = image::io::Reader::new(std::io::Cursor::new(bytes))
                                                .with_guessed_format()?
                                                .decode()?
                                                .into_rgba8();
                                            let palette = color_thief::get_palette(
                                                &image.clone().into_raw(),
                                                color_thief::ColorFormat::Rgba,
                                                1,
                                                36,
                                            )?;
                                            let mut valid_colours: Vec<&RGB8> = palette.iter().filter(|c| {
                                                let hsv: Hsv = color::rgb!(c.r,c.g,c.b).to_hsv();
                                                hsv.s > 0.3 && hsv.v > 0.1
                                            }).collect();
                                            if valid_colours.len() == 0 {
                                                valid_colours = palette.iter().filter(|c| {
                                                    let hsv: Hsv = color::rgb!(c.r,c.g,c.b).to_hsv();
                                                    hsv.v > 0.3
                                                }).collect();
                                            }
                                            mut_data.accent = *valid_colours[0];
                                            mut_data.image = image;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    QueueObjectCurrentlyPlaying::EpisodeObject(_episode) => {}
                }
            }
        }
    }
    Ok(mut_data.clone())
}

#[derive(Serialize)]
struct RefreshBody {
    pub grant_type: String,
    pub refresh_token: String,
    pub client_id: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
struct RefreshResponse {
    pub access_token: String,
}

#[derive(Clone, Debug, Default)]
enum RepeatState {
    #[default]
    Off,
    Track,
    Context,
}

impl From<Option<String>> for RepeatState {
    fn from(value: Option<String>) -> Self {
        if let Some(repeat) = value {
            return match repeat.as_str() {
                "off" => RepeatState::Off,
                "track" => RepeatState::Track,
                "context" => RepeatState::Context,
                _ => RepeatState::Off
            };
        }
        RepeatState::Off
    }
}

/*
Scopes:
user-read-playback-state
*/