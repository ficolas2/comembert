function cm 
    export COMEMBERT_OUTPUT=tmp

    comembert $argv

    if test -e "/tmp/comembert"
        commandline --replace "$(cat /tmp/comembert)"
        rm /tmp/comembert
    end

end
