zle -N __nekko::search

__nekko::search() {
    LBUFFER="$(nekko search --query="$LBUFFER")"
}
