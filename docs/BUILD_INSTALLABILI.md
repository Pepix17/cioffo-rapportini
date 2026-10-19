# Build installabili — Cioffo Rapportini 1.12.0

## Stato
- Windows: scaffolding Tauri 2 pronto per generare installer `.exe` (NSIS) e `.msi` su Windows.
- Android: configurazione Capacitor pronta; il workflow genera un APK `debug` installabile per test.
- Il controllo aggiornamenti interroga le GitHub Releases pubbliche del repository e confronta il tag con la versione installata. Il workflow sostituisce `__GITHUB_REPOSITORY__` con `owner/repository` durante la build.
- I dati restano locali. Non viene caricato nessun rapportino per cercare aggiornamenti.

## Pubblicazione
1. Crea un repository GitHub pubblico per questo progetto e carica tutti i file del pacchetto.
2. Esegui il workflow `Build Cioffo Rapportini` manualmente per produrre gli artefatti di test.
3. Per creare una release pubblica, assegna e pubblica un tag, ad esempio `v1.12.0`; il workflow caricherà gli installer Windows e l'APK di test nella release.
4. Il pulsante `Cerca aggiornamenti` confronterà il numero versione con l'ultima release pubblica e mostrerà il collegamento alla pagina di download. L'installazione resta una scelta manuale e non avviene in silenzio.

## Android: firma per gli aggiornamenti stabili
L'APK prodotto da `assembleDebug` è per test. Non va usato come canale definitivo: gli APK distribuiti agli utenti devono essere firmati sempre con la stessa chiave privata, conservata fuori dal repository. Se si cambia chiave, Android normalmente non può aggiornare l'app installata sopra quella precedente. Prima di distribuire la versione stabile bisogna configurare una chiave di firma protetta come GitHub Actions Secrets e un task Gradle `assembleRelease`.

## Windows
Il workflow produce installer desktop tramite Tauri. La compilazione reale richiede il runner Windows di GitHub Actions o un PC Windows con Rust e i prerequisiti Tauri; questo ambiente di lavoro non ha Rust/Cargo né Android SDK, quindi qui non posso dichiarare gli installer compilati o testati.

## Prima della pubblicazione
- Verificare PDF, backup/ripristino e persistenza dati nelle build native.
- Fare backup JSON prima di aggiornare.
- Verificare l'apertura del link esterno in Tauri e Android WebView.
- Non eliminare la chiave di firma Android dopo aver distribuito la prima versione stabile.
