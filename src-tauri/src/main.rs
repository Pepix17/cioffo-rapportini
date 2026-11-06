// © 2026 Giuseppe Cioffo (Pepix) — PepixGames. Cioffo Rapportini.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::webview::{PermissionKind, PermissionResponse};

fn main() {
    tauri::Builder::default()
        // Il frontend richiede getUserMedia solo dopo il clic esplicito dell'utente.
        // WebView2 deve ricevere una risposta esplicita per mostrare/gestire il permesso microfono.
        .on_permission_request(|_webview, permission| match permission {
            PermissionKind::Microphone => PermissionResponse::Allow,
            _ => PermissionResponse::Default,
        })
        .run(tauri::generate_context!())
        .expect("Errore durante l'avvio di Cioffo Rapportini");
}
