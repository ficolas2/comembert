cm () {
    export COMEMBERT_OUTPUT=tmp

    comembert "$@"

    if [[ -e "/tmp/comembert" ]]; then
        BUFFER=$(cat /tmp/comembert)

        zle redisplay
        rm /tmp/comembert
    fi
    export COMEMBERT_OUTPUT=
}

zle -N cm
bindkey '\C-f' cm
