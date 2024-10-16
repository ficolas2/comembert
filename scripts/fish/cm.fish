function cm 
    export COMEMBERT_OUTPUT=tmp
    comembert $argv
    commandline --replace "$(cat /tmp/comembert)"
    rm /tmp/comembert
end
