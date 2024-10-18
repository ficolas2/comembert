cm () {
    comembert "$@"
    BUFFER=$(cat /tmp/comembert)
    zle redisplay
}

zle -N cm
bindkey '\C-f' cm
