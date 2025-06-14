# Maintainer: Co-Prime Bageler <cloakoverseerb@gmail.com>

pkgname=ftuibos
pkgver=1.0.0
pkgrel=1
pkgdesc="The first fully TUI based operating system."
arch=('any')
url="https://github.com/averagebagelenjoyer/2BOS"
license=('MIT')
depends=()

source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP') # TEMPORARY CHECKSUM FOR TESTING

makedepends=('rust' 'cargo')

build() {
    cd "${srcdir}"
    cargo build --release # --locked
}

package() {
    cd "${srcdir}"
    install -Dm755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
