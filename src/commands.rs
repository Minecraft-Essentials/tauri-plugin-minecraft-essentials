use tauri::{Url, WebviewWindowBuilder};
use tauri::{AppHandle, command, Runtime};

use minecraft_essentials::{AuthType, AuthenticationBuilder, CustomAuthData};
use std::str::FromStr;

#[command]
pub async fn oauth<R: Runtime>(app: AppHandle<R>, client_id: &str, client_secret: &str, bedrock_rel: bool, port: Option<u16>) -> Result<CustomAuthData, String> {
    let mut auth = AuthenticationBuilder::builder(); 
    auth.of_type(AuthType::Oauth).client_id(client_id).client_secret(client_secret).bedrockrel(bedrock_rel).port(port);
    
    let get_info = auth.get_info().await.ouath_url.unwrap();
    let url = Url::from_str(&get_info).map_err(|e| e.to_string())?;
    let window = WebviewWindowBuilder::new(&app, "Minecraft Authentification", tauri::WebviewUrl::External(url)).closable(true).build().map_err(|e| e.to_string())?;

    let auth_info: CustomAuthData = auth.launch().await.map_err(|e| e.to_string())?; 

    if auth_info.access_token.is_some() {
        let _ = window.close();
    };

    Ok(auth_info)
} 
