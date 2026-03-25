Name:           rexisce
Version:        0.1.0
Release:        1%{?dist}
Summary:        Native XMPP desktop messenger

License:        MIT
URL:            https://github.com/owner/rexisce

BuildRequires:  rust, cargo, pkgconfig(openssl), pkgconfig(dbus-1)
Requires:       openssl-libs, dbus-libs

%description
A native XMPP desktop messenger built with Rust and iced.

%build
cargo build --release --bin rexisce

%install
install -Dm755 target/release/rexisce %{buildroot}%{_bindir}/rexisce

%files
%{_bindir}/rexisce
%license LICENSE
%doc README.md
