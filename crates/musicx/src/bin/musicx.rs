use musicx::{
    models::{album, artist, user},
    musicx_service::MusicXService,
    song_service::SimpleRecommendation,
};
use uuid::Uuid;
fn main() {
    let mut musicx_service = MusicXService::new();
    let user1_id = generate_timestamp_uuid();

    let user1 = user::User::new(
        user1_id.clone(),
        "user1".to_string(),
        "email1".to_string(),
        "password1".to_string(),
    );
    let user2_id = generate_timestamp_uuid();
    let user2 = user::User::new(
        user2_id.clone(),
        "user2".to_string(),
        "email2".to_string(),
        "password2".to_string(),
    );
    musicx_service.add_user(user1);
    musicx_service.add_user(user2);
    let song1_id = generate_timestamp_uuid();
    let song1 = musicx::models::song::Song::new(
        song1_id.clone(),
        "Song Title 1".to_string(),
        "Artist 1".to_string(),
        "Album 1".to_string(),
        300,
    );
    let song2_id = generate_timestamp_uuid();
    let song2 = musicx::models::song::Song::new(
        song2_id.clone(),
        "Song Title 2".to_string(),
        "Artist 2".to_string(),
        "Album 2".to_string(),
        250,
    );
    musicx_service.add_song(song1);
    musicx_service.add_song(song2);

    let album1_id = generate_timestamp_uuid();
    let album1 = album::Album::new(
        album1_id.clone(),
        "Album Title 1".to_string(),
        "Artist 1".to_string(),
        "some_date".to_string(),
        "rock".to_string(),
        vec![song1_id.clone(), song2_id.clone()],
    );
    musicx_service.add_album(album1);
    let artist1_id = generate_timestamp_uuid();
    let artist1 = artist::Artist::new(
        artist1_id.clone(),
        "Artist Name 1".to_string(),
        "rock".to_string(),
        vec![album1_id.clone()],
    );
    musicx_service.add_artist(artist1);

    musicx_service
        .login_as_user(&user1_id, "password1".to_string())
        .expect("Failed to login user");

    let songs = musicx_service.song_service.get_all_songs();
    for song in songs {
        println!("Song: {} - {}", song.get_id(), song.get_title());
    }
    musicx_service
        .add_song_for_user(&user1_id, song1_id.clone())
        .expect("Failed to add song for user");

    let playlist1_id = generate_timestamp_uuid();
    let playlist1 = musicx::models::user::Playlist::new(
        playlist1_id.clone(),
        "Playlist Title 1".to_string(),
        vec![song1_id.clone(), song2_id.clone()],
    );

    musicx_service
        .add_playlist_to_user(&user1_id, playlist1.clone())
        .expect("Failed to add playlist for user");

    let songs = musicx_service
        .recommend_songs_for_user(&user1_id, &SimpleRecommendation)
        .expect("Failed to recommend songs");

    for song in songs {
        println!("Recommended Song: {} - {}", song.get_id(), song.get_title());
    }
}

fn generate_timestamp_uuid() -> String {
    // UUID v7 uses a timestamp with millisecond precision as the most significant bits,
    // followed by random data. It's designed for time-ordered UUIDs.
    let uuid = Uuid::now_v7();
    uuid.to_string()
}
