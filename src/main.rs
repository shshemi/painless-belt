use painless_belt::policy::connect_to_internet::ConnectToInternet;
use painless_belt::policy::execute_system_binaries::ExecuteSystemBinaries;
use painless_belt::policy::read_write_dir::ReadWriteDirectory;
use painless_belt::sandbox::Sandbox;
use painless_belt::traits::ToSbpl;

fn main() {
    let cwd = std::env::current_dir().expect("current_dir");

    let sb = Sandbox::deny_by_default()
        .with_policy(ExecuteSystemBinaries)
        .with_policy(ReadWriteDirectory::new(cwd))
        .with_policy(ReadWriteDirectory::new("/tmp"))
        .with_policy(ReadWriteDirectory::new("/private/tmp"))
        .with_policy(ConnectToInternet);

    println!("--- generated SBPL ---");
    println!("{}", sb.to_sbpl());
    println!("----------------------");

    if let Err(e) = sb.init() {
        eprintln!("sandbox_init failed: {e}");
        std::process::exit(1);
    }
    println!("sandbox applied\n");

    println!("# expected to succeed");
    probe(
        "write /tmp/painless-test",
        std::fs::write("/tmp/painless-test", b"hi"),
    );
    probe("read_dir .", std::fs::read_dir(".").map(|_| ()));
    probe(
        "write ./painless-cwd-test",
        std::fs::write("./painless-cwd-test", b"hi"),
    );
    probe(
        "connect 1.1.1.1:80",
        std::net::TcpStream::connect("1.1.1.1:80").map(|_| ()),
    );

    println!("\n# expected to be DENIED");
    probe(
        "write /etc/painless-test",
        std::fs::write("/etc/painless-test", b"hi"),
    );
    probe("read /etc/hosts", std::fs::read("/etc/hosts").map(|_| ()));
    probe(
        "read_dir /var/log",
        std::fs::read_dir("/var/log").map(|_| ()),
    );
    probe(
        "bind 127.0.0.1:0 (listen)",
        std::net::TcpListener::bind("127.0.0.1:0").map(|_| ()),
    );
    probe("read_dir ..", std::fs::read_dir("..").map(|_| ()));
}

fn probe<E: std::fmt::Display>(label: &str, r: Result<(), E>) {
    match r {
        Ok(()) => println!("  {label:<32} -> ok"),
        Err(e) => println!("  {label:<32} -> err: {e}"),
    }
}
