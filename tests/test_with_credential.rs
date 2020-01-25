extern crate rspotify;
#[macro_use]
extern crate lazy_static;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;
use rspotify::spotify::senum::{AlbumType, Country};

use std::sync::Mutex;

lazy_static! {
   // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
static ref CLIENT_CREDENTIAL: Mutex<SpotifyClientCredentials> = Mutex::new(SpotifyClientCredentials::default().build());
}
#[test]
fn test_album() {
    // Or set client_id and client_secret explictly
    // let client_credential = SpotifyClientCredentials::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build();
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    let albums = spotify.album(birdy_uri);
    assert!(albums.is_ok());
}

#[test]
fn test_albums() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri1 = String::from("spotify:album:41MnTivkwTO3UUJ8DrqEJJ");
    let birdy_uri2 = String::from("spotify:album:6JWc4iAiJ9FjyK0B59ABb4");
    let birdy_uri3 = String::from("spotify:album:6UXCm6bOO4gFlDQZV5yL37");
    let track_uris = vec![birdy_uri1, birdy_uri2, birdy_uri3];
    let albums = spotify.albums(track_uris);
    assert!(albums.is_ok())
}

#[test]
fn test_album_tracks() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri = "spotify:album:6akEvsycLGftJxYudPjmqK";
    let tracks = spotify.album_track(birdy_uri, Some(2), None);
    assert!(tracks.is_ok());
}

#[test]
fn test_artist_related_artists() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri = "spotify:artist:43ZHCT0cAZBISjO8DG9PnE";
    let artist = spotify.artist_related_artists(birdy_uri);
    assert!(artist.is_ok())
}

#[test]
fn test_artist() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let artist = spotify.artist(birdy_uri);
    assert!(artist.is_ok());
}

#[test]
fn test_artists_albums() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let albums = spotify.artist_albums(
        birdy_uri,
        Some(AlbumType::Album),
        Some(Country::UnitedStates),
        Some(10),
        None,
    );
    dbg!(&albums);
    assert!(albums.is_ok());
}

#[test]
fn test_artists() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri1 = String::from("spotify:artist:0oSGxfWSnnOXhD2fKuz2Gy");
    let birdy_uri2 = String::from("spotify:artist:3dBVyJ7JuOMt4GE9607Qin");
    let artist_uris = vec![birdy_uri1, birdy_uri2];
    let artists = spotify.artists(artist_uris);
    assert!(artists.is_ok());
}

#[test]
fn test_artist_top_tracks() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let tracks = spotify.artist_top_tracks(birdy_uri, Country::UnitedStates);
    dbg!(&tracks);
    assert!(tracks.is_ok());
}

#[test]
fn test_audio_analysis() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let track = "06AKEBrKUckW0KREUWRnvT";
    let analysis = spotify.audio_analysis(track);
    assert!(analysis.is_ok());
}

#[test]
fn test_audio_features() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let track = "spotify:track:06AKEBrKUckW0KREUWRnvT";
    let features = spotify.audio_features(track);
    assert!(features.is_ok());
}

#[test]
fn test_audios_features() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let mut tracks_ids = vec![];
    let track_id1 = String::from("spotify:track:4JpKVNYnVcJ8tuMKjAj50A");
    tracks_ids.push(track_id1);
    let track_id2 = String::from("spotify:track:24JygzOLM0EmRQeGtFcIcG");
    tracks_ids.push(track_id2);
    let features = spotify.audios_features(&tracks_ids);
    assert!(features.is_ok())
}

#[test]
fn test_user() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    dbg!(&spotify);
    let birdy_uri = String::from("tuggareutangranser");
    let user = spotify.user(&birdy_uri);
    assert!(user.is_ok());
}

#[test]
fn test_track() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG6";
    let track = spotify.track(birdy_uri);
    assert!(track.is_ok());
}

#[test]
fn test_tracks() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();
    let birdy_uri1 = "spotify:track:3n3Ppam7vgaVa1iaRUc9Lp";
    let birdy_uri2 = "spotify:track:3twNvmDtFQtAd5gMKedhLD";
    let track_uris = vec![birdy_uri1, birdy_uri2];
    let tracks = spotify.tracks(track_uris, None);
    assert!(tracks.is_ok());
}

#[test]
fn test_existing_playlist() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();

    let playlist = spotify.playlist("37i9dQZF1DZ06evO45P0Eo", None, None);
    assert!(playlist.is_ok());
}

#[test]
fn test_fake_playlist() {
    let spotify = Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build();

    let playlist = spotify.playlist("fake_id", None, None);
    assert!(!playlist.is_ok());
}
