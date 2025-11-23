use std::sync::Mutex;
use tauri::State;
use vrchatapi::{
    apis,
    models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode},
};

use crate::AppData;

fn get_config(state: &State<'_, Mutex<AppData>>) -> apis::configuration::Configuration {
    let mut config = apis::configuration::Configuration::default();
    config.user_agent = Some(String::from(
        "vrc-mu-desktop/0.0.1 (https://github.com/RedlineTriad/vrc-mu-desktop)",
    ));
    {
        let state = state.lock().unwrap();
        config.client = reqwest::Client::builder()
            .cookie_provider(state.cookie_jar.clone())
            .build()
            .unwrap();
    }
    config
}

#[tauri::command]
pub async fn am_logged_in(state: State<'_, Mutex<AppData>>) -> Result<bool, ()> {
    let config = get_config(&state);

    let user = apis::authentication_api::get_current_user(&config).await;

    if let Ok(user) = user {
        match user {
            EitherUserOrTwoFactor::CurrentUser(_me) => Ok(true),
            EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => Ok(false),
        }
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn login(
    state: State<'_, Mutex<AppData>>,
    username: String,
    password: String,
    two_fa_code: Option<String>,
) -> Result<String, String> {
    let mut config = get_config(&state);
    config.basic_auth = Some((username, Some(password)));

    let user = apis::authentication_api::get_current_user(&config)
        .await
        .map_err(|_e| String::from("failed to log in"))?;

    println!("Login attempt for user with two_fa_code: {:?}", two_fa_code);
    match user {
        vrchatapi::models::EitherUserOrTwoFactor::CurrentUser(me) => {
            println!("Username: {}", me.username.unwrap())
        }
        vrchatapi::models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
            let two_fa_code = match two_fa_code {
                Some(code) => code,
                None => return Err("2FA code required".into()),
            };
            if requires_auth
                .requires_two_factor_auth
                .contains(&String::from("emailOtp"))
            {
                if let Err(err) = apis::authentication_api::verify2_fa_email_code(
                    &config,
                    TwoFactorEmailCode::new(two_fa_code),
                )
                .await
                {
                    return Err(format!("Error verifying 2FA code: {}", err));
                }
            } else {
                if let Err(err) = apis::authentication_api::verify2_fa(
                    &config,
                    TwoFactorAuthCode::new(two_fa_code),
                )
                .await
                {
                    return Err(format!("Error verifying 2FA code: {}", err));
                }
            }
        }
    }

    let user = apis::authentication_api::get_current_user(&config)
        .await
        .map_err(|e| format!("failed to get user after 2FA: {:?}", e))?;

    match user {
        EitherUserOrTwoFactor::CurrentUser(user) => return Ok(user.id),
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => {
            return Err("2FA code required, this is really weird".into())
        }
    }
}
