fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!(
            r#"Give shell command that should be executed by the service as parameter
{} pwd; ls && echo "OK" "#,
            args[0]
        );
        std::process::exit(1);
    }

    let args = args[1..].join(" ");

    println!(
        r#"
[Unit]
Description=Bash command executed by Systemd service

[Service]
Type=simple
ExecStart=/bin/bash -c "{}"

[Install]
WantedBy=multi-user.target
"#,
        args
    );
}
