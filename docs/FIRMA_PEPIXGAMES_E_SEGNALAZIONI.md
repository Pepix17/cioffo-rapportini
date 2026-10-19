# Firma PepixGames, versionamento e segnalazione bug

© 2026 Giuseppe Cioffo (Pepix) — PepixGames.

- L'attribuzione è inserita nei commenti dei sorgenti HTML, CSS, JavaScript e Rust; non viene visualizzata come contenuto della pagina.
- I file JSON non supportano commenti standard validi: non sono stati inseriti commenti che renderebbero invalido il formato. La titolarità è documentata qui e nei metadati del progetto.
- Il footer e i badge del modal versione sono sincronizzati da `CURRENT_APP_VERSION` in `www/index.html`. Per una nuova release aggiornare anche `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` e la voce changelog.
- Per abilitare il report bug, impostare la variabile GitHub Actions `BUG_REPORT_EMAIL` (Repository → Settings → Secrets and variables → Actions → Variables → New repository variable). In alternativa sostituire `__BUG_REPORT_EMAIL__` in `www/index.html` con l'indirizzo di destinazione.
- Il pulsante apre il client email dell'utente e prepara un messaggio; l'utente deve confermare l'invio. Non vengono allegati rapportini o backup automaticamente.
- Le cartelle sorgenti e i file di sviluppo non vengono installati come cartelle esplorabili con il pacchetto Tauri/APK; tuttavia gli asset HTML/CSS/JS incorporati in un'app possono essere estratti da un utente tecnico. Il packaging non è una protezione crittografica del codice.
