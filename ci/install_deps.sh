export RUSTUP_HOME=/usr/local/rustup
export CARGO_HOME=/usr/local/cargo
export PATH=/usr/local/cargo/bin:$PATH
export RUST_VERSION=1.41.1

set -eux

dpkgArch="$(dpkg --print-architecture)"
case "${dpkgArch##*-}" in
    amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='e68f193542c68ce83c449809d2cad262cc2bbb99640eb47c58fc1dc58cc30add' ;;
    armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='7c1c329a676e50c287d8183b88f30cd6afd0be140826a9fbbc0e3d717fab34d7' ;;
    arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='d861cc86594776414de001b96964be645c4bfa27024052704f0976dc3aed1b18' ;;
    i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='89f1f797dca2e5c1d75790c3c6b7be0ee473a7f4eca9663e623a41272a358da0' ;;
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;;
esac
url="https://static.rust-lang.org/rustup/archive/1.20.2/${rustArch}/rustup-init"
wget --no-verbose "$url"
echo "${rustupSha256} *rustup-init" | sha256sum -c -
chmod +x rustup-init
./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION
rm rustup-init
chmod -R a+w $RUSTUP_HOME $CARGO_HOME
rustup --version
cargo --version
rustc --version

rustup target add x86_64-unknown-linux-musl
apt-get update
apt-get install -y musl-tools
HELM_INSTALL_DIR=/bin
curl https://raw.githubusercontent.com/helm/helm/master/scripts/get | bash
curl -L -o myke https://github.com/fiji-flo/myke/releases/download/0.9.11/myke-0.9.11-x86_64-unknown-linux-musl
chmod +x ./myke
mv ./myke /bin/myke