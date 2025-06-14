pkgname=2bos
pkgver=1.0.0
pkgrel=1
pkgdesc="Fully Text User Interface Based Operating System"
arch=('any')
url="https://github.com/yourusername/2BOS"
license=('MIT') # or whatever license you use
depends=('zsh' 'tmux' 'btop' 'nvim') # add any runtime dependencies

source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')  # Use `SKIP` while testing, real checksum preferred later

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 install.sh "$pkgdir/usr/bin/2bos"
}
