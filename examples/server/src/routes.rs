use crate::dto::{AuthSign, ContentCheck, EncryptedPayload, PhonePayload, ShortLinkDto};
use crate::{AppState, error::Error};
use actix_web::{Responder, web};
use wechat_minapp::link::{Link, ShortLinkArgs};
use wechat_minapp::minapp_security::{Args, MinappSecurity, Scene};
use wechat_minapp::qr::{Qr, QrCodeArgs};
use wechat_minapp::user::User;

pub async fn login(
    state: web::Data<AppState>,
    query: web::Json<AuthSign>,
) -> Result<impl Responder, Error> {
    let client = state.wechat_minapp.client();
    let user = User::new(client);
    let credential = user.login(&query.code).await?;
    Ok(web::Json(credential))
}

pub async fn user_info(
    state: web::Data<AppState>,
    payload: web::Json<EncryptedPayload>,
) -> Result<impl Responder, Error> {
    let client = state.wechat_minapp.client();
    let user = User::new(client);
    let credential = user.login(&payload.code).await?;
    let user_info = credential.decrypt(&payload.encrypted_data, &payload.iv)?;

    Ok(web::Json(user_info))
}

pub async fn get_phone_num(
    state: web::Data<AppState>,
    payload: web::Json<PhonePayload>,
) -> Result<impl Responder, Error> {
    let client = state.wechat_minapp.client();
    let user = User::new(client);
    let phone = user
        .get_contact(&payload.code, payload.openid.as_deref())
        .await?;

    Ok(web::Json(phone))
}

pub async fn get_user_qr(state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let page: &str = "page/index/index?invite=openid";
    let qr_args = QrCodeArgs::builder().path(page).build()?;
    let client = state.wechat_minapp.client();
    let qr = Qr::new(client);
    let buffer = qr.qr_code(qr_args).await?;

    Ok(buffer.buffer().to_vec())
}

pub async fn msg_sec_check(
    state: web::Data<AppState>,
    data: web::Json<ContentCheck>,
) -> Result<impl Responder, Error> {
    let client = state.wechat_minapp.client();
    let user = User::new(client.clone());
    let credential = user.login(&data.code).await?;
    let args = Args::builder()
        .content(&data.content)
        .scene(Scene::Forum)
        .openid(credential.open_id())
        .build()?;
    let security = MinappSecurity::new(client);
    let result = security.msg_sec_check(&args).await?;

    if result.is_pass() {
        println!("内容安全，可以发布");
    } else if result.needs_review() {
        println!("内容需要人工审核");
    } else {
        println!("内容有风险，建议修改");
    }

    Ok(web::Json(result))
}

pub async fn short_link(
    state: web::Data<AppState>,
    data: web::Json<ShortLinkDto>,
) -> Result<impl Responder, Error> {
    let mut args = ShortLinkArgs::builder().path(&data.page_url);

    if let Some(page_title) = &data.page_title {
        args = args.page_title(page_title);
    }

    if let Some(is_permanent) = data.is_permanent
        && is_permanent
    {
        args = args.with_permanent();
    }

    let args = args.build()?;
    let client = state.wechat_minapp.client();
    let link = Link::new(client);
    let result = link.short_link(args).await?;

    Ok(web::Json(result))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(login))
            .route("/user_info", web::post().to(user_info))
            .route("/get_phone_num", web::post().to(get_phone_num))
            .route("/get_user_qr", web::post().to(get_user_qr))
            .route("/msg_sec_check", web::post().to(msg_sec_check))
            .route("/short_link", web::post().to(short_link)),
    );
}
