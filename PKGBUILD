pkgname=2bos
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
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release --bin 2bos
    cargo build --release --bin 2bos-installer
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/2bos" "$pkgdir/usr/bin/2bos"
    install -Dm755 "target/release/2bos-installer" "$pkgdir/usr/bin/2bos-installer"
}
