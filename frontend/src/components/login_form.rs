use dioxus::prelude::*;
use kairos_common::{LoginRequest, LoginResponse};
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use dioxus_router::prelude::use_navigator;
use crate::Route;

#[component]
pub fn LoginForm(
    #[props(optional)] on_success: Option<EventHandler<()>>,
    #[props(optional)] on_error: Option<EventHandler<String>>,
) -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(String::new);
    let navigate = use_navigator();

    rsx! {
        div {
            class: "login-form-container",
            h1 { "Iniciar Sesión" }
            p { "Ingresa tus credenciales para acceder al sistema." }
            form {
                class: "login-form",
                onsubmit: move |_event| { // El evento no se usa, lo marcamos con _
                    loading.set(true);
                    error_msg.set(String::new());
                    let email_val = email();
                    let password_val = password();
                    let mut loading = loading.clone();
                    let mut error_msg = error_msg.clone();
                    let navigate = navigate.clone();
                    let on_success = on_success.clone();
                    let on_error = on_error.clone();
                    spawn_local(async move {
                        let req = LoginRequest {
                            email: email_val.clone(),
                            password: password_val.clone(),
                        };
                        let req_builder = Request::post("/api/v1/auth/login")
                            .header("Content-Type", "application/json")
                            .body(serde_json::to_string(&req).unwrap());
                        let resp = match req_builder {
                            Ok(request) => request.send().await,
                            Err(e) => {
                                loading.set(false);
                                error_msg.set(format!("Error construyendo la petición: {}", e));
                                if let Some(cb) = &on_error { cb.call(format!("Error construyendo la petición: {}", e)); }
                                return;
                            }
                        };
                        match resp {
                            Ok(response) => {
                                if response.status() == 200 {
                                    let login_resp: Result<LoginResponse, _> = response.json().await;
                                    match login_resp {
                                        Ok(_data) => {
                                            loading.set(false);
                                            if let Some(cb) = &on_success { cb.call(()); }
                                            navigate.replace(Route::DashboardPage {});
                                        },
                                        Err(e) => {
                                            loading.set(false);
                                            error_msg.set(format!("Error de parseo: {}", e));
                                            if let Some(cb) = &on_error { cb.call(format!("Error de parseo: {}", e)); }
                                        }
                                    }
                                } else {
                                    loading.set(false);
                                    let err_text = response.text().await.unwrap_or_else(|_| "Error desconocido".to_string());
                                    error_msg.set(format!("Error: {}", err_text));
                                    if let Some(cb) = &on_error { cb.call(format!("Error: {}", err_text)); }
                                }
                            },
                            Err(e) => {
                                loading.set(false);
                                error_msg.set(format!("Error de red: {}", e));
                                if let Some(cb) = &on_error { cb.call(format!("Error de red: {}", e)); }
                            }
                        }
                    });
                },
                div {
                    class: "form-group",
                    label { r#for: "email", "Correo Electrónico" }
                    input {
                        r#type: "email",
                        id: "email",
                        name: "email",
                        placeholder: "tu@email.com",
                        required: true,
                        oninput: move |event| email.set(event.value())
                    }
                }
                div {
                    class: "form-group",
                    label { r#for: "password", "Contraseña" }
                    input {
                        r#type: "password",
                        id: "password",
                        name: "password",
                        placeholder: "••••••••",
                        required: true,
                        oninput: move |event| password.set(event.value())
                    }
                }

                button {
                    r#type: "submit",
                    class: "btn",
                    { if loading() { rsx!(span { "Entrando..." }) } else { rsx!("Entrar") } }
                }
            }
            { if loading() { rsx!(div { "Cargando..." }) } else { rsx!("") } }
            { if !error_msg().is_empty() { rsx!(div { class: "error-msg", "{error_msg()}" }) } else { rsx!("") } }
            div {
                class: "form-footer",
                "¿No tienes una cuenta? ",
                a {
                    href: "#", // Futura ruta de registro
                    "Regístrate aquí"
                }
            }
        }
    }
} 