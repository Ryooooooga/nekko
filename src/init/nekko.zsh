zle -N __nekko::expand

__nekko::expand() {
    LBUFFER="$(nekko expand --query="$LBUFFER")"
}
