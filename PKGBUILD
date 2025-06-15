# Maintainer: Co-Prime Bageler <cloakoverseerb@gmail.com>

pkgname=ftuibos
pkgver=1.0.0
pkgrel=1
pkgdesc="The first fully TUI based operating system."
arch=('any')
url="https://github.com/averagebagelenjoyer/2BOS"
license=('MIT')
depends=()

source=("git+$url")
sha256sums=('SKIP') # TEMPORARY CHECKSUM FOR TESTING

makedepends=('rust' 'cargo')

pkgver() {
    cd "$srcdir/2BOS"
    git describe --tags | sed 's/^v//' # Converts Git tag to version
}

build() {
    cd "${srcdir}"
    cargo build --release # --locked
}

package() {
    cd "${srcdir}"
    install -Dm755 "target/release/ftuibos" "${pkgdir}/usr/bin/ftuibos"
}
