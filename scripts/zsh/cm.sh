autoload -Uz add-zsh-hook
cm () {
    export COMEMBERT_OUTPUT=tmp

    comembert "$@"

    if [[ -e "/tmp/comembert" ]]; then
        BUFFER=$(cat /tmp/comembert)

        zle end-of-line
        zle redisplay
        rm /tmp/comembert
    fi
    export COMEMBERT_OUTPUT=
}

add-zsh-hook precmd _initialize_comembert
_initialize_comembert() {
    zle -N cm
    bindkey '\C-f' cm
    add-zsh-hook -d precmd _initialize_comembert
}

