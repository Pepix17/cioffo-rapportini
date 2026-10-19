# Cioffo Rapportini v1.6 — Agenda e calendario

## Calendario e agenda
- Clic sul riepilogo della settimana apre un modal calendario a schermo quasi intero.
- Selezionando una data si aggiorna la selezione del giorno nel rapportino settimanale.
- Note per giorno salvate nel campo `notes` del giorno, quindi persistono localmente e sono riutilizzate nella vista settimanale.
- Nuova pagina Agenda con calendario mensile, indicatori per programma/svolto/note e scheda del giorno.
- Tutti i dati restano locali nel browser.

## Dettatura vocale: stato
Il prototipo usa il riconoscimento vocale disponibile via browser (Web Speech API / SpeechRecognition quando supportata). Se nel tuo browser funziona, puoi continuare a usarlo: va verificato il permesso microfono e la trascrizione. Questo non prova che sia offline; supporto, accuratezza e necessità di connessione dipendono dal browser/dispositivo. Un motore offline nativo Android non è ancora integrato in questo prototipo. Test da fare: dettatura breve, frase lunga, stop/riavvio, rifiuto permesso, assenza connessione e controllo prima del salvataggio.


## Correzioni successive — esportazione periodo e PDF
- Corretta la disposizione dei testi del profilo nell’intestazione PDF con righe flessibili e spaziatura tipografica dedicata alla stampa.
- Aggiunta selezione del periodo: 1, 2 o 3 settimane, mese completo, intervallo personalizzato o tutti i rapportini.
- Le date iniziale e finale vengono suggerite automaticamente a partire dalla settimana selezionata e possono essere modificate.
- PDF e CSV usano lo stesso intervallo selezionato.


## Correzione esportazione PDF — v1.7.1
- Nel layout di anteprima sono disponibili tre pulsanti distinti: Stampa, Salva PDF e Chiudi.
- Rimossa l'apertura automatica della finestra di stampa: ora si apre soltanto dopo aver premuto Stampa o Salva PDF.
- Il pulsante Salva PDF richiama la finestra di stampa del sistema, dove è possibile scegliere “Salva come PDF” (comportamento standard del browser/WebView).


## v1.8.0 — Cioffo Rapportini
- Nome aggiornato nei metadati e nei documenti esportati.
- Salva PDF genera un PDF locale senza aprire la finestra di stampa; Stampa resta distinto.
- Redesign UI/UX con animazioni e microinterazioni. La navigazione è stata lasciata invariata.


## Versione 1.9.0 — Professional UI refresh
- Aggiornato il design dell area di lavoro: gerarchie tipografiche, superfici, ombre, colori, stati focus, animazioni e microinterazioni.
- Menu laterale e relative regole CSS lasciati intenzionalmente invariati.
- Mantenuta la generazione PDF diretta separata dal comando Stampa.
- Metadati applicazione aggiornati per il futuro packaging desktop/mobile.
- La compilazione EXE/APK e il sistema di aggiornamento non sono inclusi in questo pacchetto sorgente.


## Versione 1.10.0 — Cronologia e aggiornamenti
- Aggiunto il pulsante **Versione** nell’area superiore, senza modificare la navigazione laterale.
- Aggiunto un modal con transizione animata e cronologia delle novità dalla v1.5 alla v1.10.0.
- Aggiunto il pulsante **Cerca aggiornamenti**. La UI comunica chiaramente che la verifica online richiede ancora la configurazione del canale ufficiale: non simula una ricerca riuscita.
- Conservate le funzioni esistenti e i dati locali.


## Versione 1.12.0 — Build native e controllo release
- Il pulsante di ricerca controlla ora la release pubblica GitHub configurata in fase di build, confronta le versioni e mostra un link ufficiale per scaricare gli aggiornamenti.
- Aggiunto scaffolding Tauri 2 per installer Windows e workflow GitHub Actions per Windows e Android.
- L'APK del workflow è esplicitamente un APK di test; per distribuzione stabile serve una chiave Android persistente e una build firmata release.
- La sidebar resta invariata. Il controllo aggiornamenti non legge né invia i dati dei rapportini.
