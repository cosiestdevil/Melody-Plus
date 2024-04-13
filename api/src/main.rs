mod data;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use dotenvy::dotenv;
use rocket::{get, launch, post, Request, routes, State, uri};
use rocket::http::Status;
use rocket::outcome::Outcome::Success;
use rocket::request::{FromRequest, Outcome};
use rsa::pkcs8::DecodePublicKey;
use rsa::{Oaep, RsaPublicKey};
use rsa::sha2::{Digest, Sha256};
use rand::distributions::{Alphanumeric, DistString};
use rocket::response::Redirect;
use serde::{Deserialize, Serialize};
use crate::data::{get_connection_pool,Connection};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/setup")]
async fn request_spotify(_user:User,_pool: &State<Connection>,current_host: CurrentHost)->Result<Redirect,Status>{
    //let code_verifier = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    //let mut hasher = Sha256::new();
    //hasher.update(code_verifier.as_bytes());
    //let result = hasher.finalize();
    //let code_challenge =URL_SAFE.encode(result.as_slice()).replace("=","");
    //sqlx::query!("insert into spotify_auth_requests (state,code_verifier) values ($1,$2)",state,code_verifier).execute(pool.inner()).await.unwrap();

    let redirect_url = format!("{}{}",current_host.0,uri!(spotify(_,_)));
    let client_id = dotenvy::var("SPOTIFY_CLIENT_ID").or_else(|_|Err(Status::UnavailableForLegalReasons))?;
    let query_params =SpotifyAuthParams {
        response_type: "code".to_owned(),
        client_id: client_id.to_owned(),
        scope: "user-modify-playback-state user-read-playback-state".to_owned(),
        redirect_uri: redirect_url,
        state
    };
    Ok(Redirect::to(format!("https://accounts.spotify.com/authorize?{}",serde_qs::to_string(&query_params).unwrap())))
    //redirect to spotify url
}

#[get("/spotify?<code>&<state>")]
async fn spotify(user:User,pool: &State<Connection>,code:Option<&str>,state:Option<&str>,current_host: CurrentHost)-> Result<(),Status>{
    //let code_verifier = sqlx::query_scalar!("delete from spotify_auth_requests where state = $1 returning code_verifier",state).fetch_one(pool.inner()).await.unwrap();
    let redirect_uri = format!("{}{}",current_host.0,uri!(spotify(_,_)));
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let client_id = dotenvy::var("SPOTIFY_CLIENT_ID").or_else(|_|Err(Status::UnavailableForLegalReasons))?;
    let client_secret = dotenvy::var("SPOTIFY_CLIENT_SECRET").or_else(|_|Err(Status::UnavailableForLegalReasons))?;
    let refresh_token = client
        .post("https://accounts.spotify.com/api/token")
        .form(&TokenRequestBody { grant_type: "authorization_code".to_owned(),code:code.unwrap().to_owned(), client_id:"".to_owned(),redirect_uri })
        .basic_auth(client_id,Some(client_secret))
        .send().await.unwrap();
    let text = refresh_token.text().await.unwrap();
    println!("{}",text);
    let refresh_token = serde_json::from_str::<TokenResponseBody>(text.as_str()).unwrap();
    sqlx::query!("update public.user set spotify_refresh_token = $1 where id=$2",refresh_token.refresh_token,user.id).execute(pool.inner()).await.unwrap();
    Ok(())
}
#[derive(Serialize)]
struct TokenRequestBody{
    grant_type:String,
    code:String,
    redirect_uri:String,
    client_id:String,

}
#[derive(Deserialize)]
struct TokenResponseBody {
    pub refresh_token: String,
}
#[post("/link",data="<key>")]
async fn link_device(user:User,key:&str,pool: &State<Connection>){
    //Insert device against user, remove from any other user
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let result = hasher.finalize();
    let id =URL_SAFE.encode(result.as_slice());
    let device = sqlx::query_as!(Device,"select * from device where id = $1",id)
        .fetch_optional(pool.inner()).await.unwrap();

    match device {
        None => {
            sqlx::query!("insert into device (user_id,public_key,id) values ($1,$2,$3)",user.id,key,id).execute(pool.inner()).await.unwrap();
        }
        Some(_) => {
            sqlx::query!("update device set user_id = $1 where id = $2",user.id,id).execute(pool.inner()).await.unwrap();
        }
    }


}

#[post("/refresh_token",data="<key>")]
async fn refresh_token(key:&str,pool: &State<Connection>)->Result<Vec<u8>,Status>{
    let device = sqlx::query_as!(Device,"select * from device where id = $1",key)
        .fetch_optional(pool.inner()).await.unwrap();
    if let Some(device) = device{
        let public_key = RsaPublicKey::from_public_key_pem(device.public_key.as_str()).unwrap();
        let refresh_token = sqlx::query_scalar!("select spotify_refresh_token from public.user where id = $1",device.user_id).fetch_one(pool.inner()).await.unwrap();
        if let Some(refresh_token) = refresh_token {
            let data = RefreshResponse{
                token:refresh_token,
                secret:dotenvy::var("SPOTIFY_CLIENT_SECRET").or_else(|_|Err(Status::UnavailableForLegalReasons))?
            };
            let data = serde_json::to_string(&data).unwrap();
            let refresh_token = data.as_bytes();
            let mut rng = rand::thread_rng();
            let padding = Oaep::new::<Sha256>();
            let enc_data = public_key.encrypt(&mut rng, padding, &refresh_token[..]).expect("failed to encrypt");
            return Ok(enc_data);
        }
        return Err(Status::BadRequest);
    }
    return Err(Status::Unauthorized);



}
#[derive(Serialize)]
struct RefreshResponse{
    token:String,
    secret:String,
}
#[launch]
async fn rocket() -> _ {
    let _ = dotenvy::dotenv().unwrap();
    let pool = get_connection_pool().await.unwrap();
    rocket::build()
        .mount("/", routes![index,refresh_token,spotify,request_spotify,link_device])
        .manage(pool)
}

struct User{
    id:i32
}
#[rocket::async_trait]
impl <'r> FromRequest<'r> for User{
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = request.headers().get_one("Authorization");
        if let Some(_auth) = auth{

            return Success(User{id:1})
        }
        let cookie_auth = request.cookies().get("Authorization");
        if let Some(_auth) = cookie_auth{
            return Success(User{id:1})
        }
        Outcome::Error((Status::Unauthorized,()))
    }
}

pub struct CurrentHost(String);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentHost {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract the host from the request URI
        let host = dotenvy::var("API_BASE").unwrap();
        Success(CurrentHost(host))
    }
}

struct Device{
    public_key:String,
    user_id:Option<i32>,
    id:String
}

#[derive(Serialize,Deserialize)]
struct SpotifyAuthParams{
    response_type:String,
    client_id:String,
    scope:String,
    redirect_uri:String,
    state:String
}
