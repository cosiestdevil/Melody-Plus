/*
 * Spotify Web API
 *
 * You can use Spotify's Web API to discover music and podcasts, manage your Spotify library, control audio playback, and much more. Browse our available Web API endpoints using the sidebar at left, or via the navigation bar on top of this page on smaller screens.  In order to make successful Web API requests your app will need a valid access token. One can be obtained through <a href=\"https://developer.spotify.com/documentation/general/guides/authorization-guide/\">OAuth 2.0</a>.  The base URI for all Web API requests is `https://api.spotify.com/v1`.  Need help? See our <a href=\"https://developer.spotify.com/documentation/web-api/guides/\">Web API guides</a> for more information, or visit the <a href=\"https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer\">Spotify for Developers community forum</a> to ask questions and connect with other developers. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::spotify;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SegmentObject {
    /// The starting point (in seconds) of the segment.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    /// The duration (in seconds) of the segment.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// The confidence, from 0.0 to 1.0, of the reliability of the segmentation. Segments of the song which are difficult to logically segment (e.g: noise) may correspond to low values in this field. 
    #[serde(rename = "confidence", skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    /// The onset loudness of the segment in decibels (dB). Combined with `loudness_max` and `loudness_max_time`, these components can be used to describe the \"attack\" of the segment.
    #[serde(rename = "loudness_start", skip_serializing_if = "Option::is_none")]
    pub loudness_start: Option<f64>,
    /// The peak loudness of the segment in decibels (dB). Combined with `loudness_start` and `loudness_max_time`, these components can be used to describe the \"attack\" of the segment.
    #[serde(rename = "loudness_max", skip_serializing_if = "Option::is_none")]
    pub loudness_max: Option<f64>,
    /// The segment-relative offset of the segment peak loudness in seconds. Combined with `loudness_start` and `loudness_max`, these components can be used to desctibe the \"attack\" of the segment.
    #[serde(rename = "loudness_max_time", skip_serializing_if = "Option::is_none")]
    pub loudness_max_time: Option<f64>,
    /// The offset loudness of the segment in decibels (dB). This value should be equivalent to the loudness_start of the following segment.
    #[serde(rename = "loudness_end", skip_serializing_if = "Option::is_none")]
    pub loudness_end: Option<f64>,
    /// Pitch content is given by a “chroma” vector, corresponding to the 12 pitch classes C, C#, D to B, with values ranging from 0 to 1 that describe the relative dominance of every pitch in the chromatic scale. For example a C Major chord would likely be represented by large values of C, E and G (i.e. classes 0, 4, and 7).  Vectors are normalized to 1 by their strongest dimension, therefore noisy sounds are likely represented by values that are all close to 1, while pure tones are described by one value at 1 (the pitch) and others near 0. As can be seen below, the 12 vector indices are a combination of low-power spectrum values at their respective pitch frequencies. ![pitch vector](https://developer.spotify.com/assets/audio/Pitch_vector.png) 
    #[serde(rename = "pitches", skip_serializing_if = "Option::is_none")]
    pub pitches: Option<Vec<f64>>,
    /// Timbre is the quality of a musical note or sound that distinguishes different types of musical instruments, or voices. It is a complex notion also referred to as sound color, texture, or tone quality, and is derived from the shape of a segment’s spectro-temporal surface, independently of pitch and loudness. The timbre feature is a vector that includes 12 unbounded values roughly centered around 0. Those values are high level abstractions of the spectral surface, ordered by degree of importance.  For completeness however, the first dimension represents the average loudness of the segment; second emphasizes brightness; third is more closely correlated to the flatness of a sound; fourth to sounds with a stronger attack; etc. See an image below representing the 12 basis functions (i.e. template segments). ![timbre basis functions](https://developer.spotify.com/assets/audio/Timbre_basis_functions.png)  The actual timbre of the segment is best described as a linear combination of these 12 basis functions weighted by the coefficient values: timbre = c1 x b1 + c2 x b2 + ... + c12 x b12, where c1 to c12 represent the 12 coefficients and b1 to b12 the 12 basis functions as displayed below. Timbre vectors are best used in comparison with each other. 
    #[serde(rename = "timbre", skip_serializing_if = "Option::is_none")]
    pub timbre: Option<Vec<f64>>,
}

impl SegmentObject {
    pub fn new() -> SegmentObject {
        SegmentObject {
            start: None,
            duration: None,
            confidence: None,
            loudness_start: None,
            loudness_max: None,
            loudness_max_time: None,
            loudness_end: None,
            pitches: None,
            timbre: None,
        }
    }
}

