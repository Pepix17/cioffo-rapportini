# Cioffo Rapportini — v1.12.0

App locale per organizzare attività lavorative, agenda, note, archivi ed esportazioni.

## Pacchetto
- `www/`: app HTML/CSS/JavaScript condivisa tra desktop e Android.
- `src-tauri/`: base desktop Tauri 2 per generare installer Windows.
- `capacitor.config.json`: configurazione app Android.
- `.github/workflows/release.yml`: build automatica degli installer e pubblicazione delle release su GitHub.
- `docs/BUILD_INSTALLABILI.md`: istruzioni e limiti della distribuzione.

## Controllo aggiornamenti
Il pulsante **Versione → Cerca aggiornamenti** controlla l'ultima GitHub Release pubblica. Durante la build CI, il workflow configura automaticamente il repository da cui è partita la compilazione. La ricerca non trasmette i dati dei rapportini; mostra il link alla release e lascia all'utente il download/installazione.

## Importante
Il pacchetto è sorgente predisposto per build automatiche, non contiene ancora installer `.exe` o `.apk` compilati. L'APK prodotto dal workflow è una build debug per test; per distribuire aggiornamenti Android stabili occorre configurare una chiave di firma persistente e una build release firmata. Consultare `docs/BUILD_INSTALLABILI.md`.

La navigazione laterale esistente è stata lasciata invariata.
