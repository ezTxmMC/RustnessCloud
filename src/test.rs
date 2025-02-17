fn main() {
    let keep_running = Arc::new(AtomicBool::new(true));

    task::block_on(async {
        while keep_running.load(Ordering::Relaxed) {
            let uuid = Uuid::new_v4();

            let jar_file_name = "paper-1.20.6.jar".to_string();
            let eula_file_name = "eula.txt".to_string();

            let jar_file = format!("softwares/{}", &jar_file_name);
            let eula_file = format!("misc/{}", &eula_file_name);
            let working_dir = format!("tmp/server-{}", &uuid);

            if !Path::new(&working_dir).exists() {
                fs::create_dir_all(&working_dir).expect("Fehler beim Erstellen des Arbeitsverzeichnisses");
            }

            let jar_file_name_as_path = Path::new(&jar_file)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let eula_file_name_as_path = Path::new(&eula_file)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let jar_destination_file = Path::new(&working_dir).join(&jar_file_name_as_path);
            let eula_destination_file = Path::new(&working_dir).join(&eula_file_name_as_path);

            match fs::copy(&jar_file, &jar_destination_file) {
                Ok(_) => {
                    println!("JAR-Datei erfolgreich in das Arbeitsverzeichnis kopiert.");
                    let destination_file_path = jar_destination_file.to_str().unwrap();
                    println!("Pfad der kopierten Datei: {}", destination_file_path);
                }
                Err(err) => {
                    eprintln!("Fehler beim Kopieren der JAR-Datei: {}", err);
                    exit(1);
                }
            }
            match fs::copy(&eula_file, &eula_destination_file) {
                Ok(_) => {
                    println!("EULA-Datei erfolgreich in das Arbeitsverzeichnis kopiert.");
                    let destination_file_path = eula_destination_file.to_str().unwrap();
                    println!("Pfad der kopierten Datei: {}", destination_file_path);
                }
                Err(err) => {
                    eprintln!("Fehler beim Kopieren der EULA-Datei: {}", err);
                    exit(1);
                }
            }
            thread::sleep(Duration::from_secs(1));
            let jar_file_name_clone = jar_file_name_as_path.clone();
            let working_dir_clone = working_dir.clone();
            let keep_running_clone = Arc::clone(&keep_running);
            let handle = thread::spawn(move || {
                start_terminal(&jar_file_name_clone, &working_dir_clone, keep_running_clone);
            });

            // Warte auf Benutzereingabe für den Stop-Befehl
            println!("Drücken Sie 'q' und dann Enter, um das Programm zu beenden und den Serverordner zu löschen:");
            let mut input = String::new();
            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin.lock());
            reader
                .read_line(&mut input)
                .expect("Fehler beim Lesen der Benutzereingabe");

            if input.trim() == "q" {
                keep_running.store(false, Ordering::Relaxed);
                // Warte auf das Beenden der Java-Anwendung
                handle.join().expect("Thread konnte nicht beendet werden");

                // Lösche den Serverordner
                if let Err(err) = fs::remove_dir_all(&working_dir) {
                    eprintln!("Fehler beim Löschen des Arbeitsverzeichnisses: {}", err);
                    exit(1);
                }
            }
        }
    });
}

fn start_terminal(jar_file_name: &str, working_dir: &str, keep_running: Arc<AtomicBool>) {
    let mut cmd = Command::new("java");
    cmd.arg("-jar")
        .arg(jar_file_name)
        .arg("--nogui")
        .current_dir(working_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let child = cmd.spawn().expect("Fehler beim Starten der Java-Anwendung");

    // Warte auf SIGINT-Signal (Strg+C) zum Beenden der Java-Anwendung
    let mut signal_stream = signal(SignalKind::interrupt()).expect("Fehler beim Einrichten des Signal-Handlers");
    task::block_on(async {
        signal_stream.recv().await;
        if let Err(err) = child.kill() {
            eprintln!("Fehler beim Beenden der Java-Anwendung: {}", err);
            exit(1);
        }
        keep_running.store(false, Ordering::Relaxed);
    });
}
