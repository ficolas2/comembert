cm() {
    export COMEMBERT_OUTPUT=tmp

    comembert "$@"
    BUFFER=$(cat /tmp/comembert)

    if [[ -n "$BUFFER" ]]; then
        READLINE_LINE="$BUFFER"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

bind -x '"\C-f": cm'
