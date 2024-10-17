cm() {
    export COMEMBERT_OUTPUT=tmp

    comembert "$@"

    if [[ -e "/tmp/comembert" ]]; then
        BUFFER=$(cat /tmp/comembert)

        if [[ -n "$BUFFER" ]]; then
            READLINE_LINE="$BUFFER"
            READLINE_POINT=${#READLINE_LINE}
        fi
        rm /tmp/comembert
    fi
}

bind -x '"\C-f": cm'
