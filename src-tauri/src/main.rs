// © 2026 Giuseppe Cioffo (Pepix) — PepixGames. Cioffo Rapportini.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Errore durante l'avvio di Cioffo Rapportini");
}
