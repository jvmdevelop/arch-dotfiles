DOTFILES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="$HOME/.config"

CONFIGS=(
  hypr
  kitty
  nvim
  fish
  waybar
  rofi
  rofi.dieno
  wofi
  gtk-3.0
  gtk-4.0
  mpv
  htop
  vesktop
  obs-studio
  Cursor
  Windsurf
)

for cfg in "${CONFIGS[@]}"; do
  src="$DOTFILES_DIR/$cfg"
  dst="$CONFIG_DIR/$cfg"

  if [ ! -e "$src" ]; then
    echo "skip (not found): $cfg"
    continue
  fi

  if [ -L "$dst" ]; then
    rm "$dst"
  elif [ -e "$dst" ]; then
    mv "$dst" "$dst.bak"
    echo "backed up: $cfg -> $cfg.bak"
  fi

  ln -s "$src" "$dst"
  echo "linked: $dst -> $src"
done

echo "Done."
