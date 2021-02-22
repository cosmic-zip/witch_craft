echo 'Update and install...'
#######################################
pkg update -y
pkg install php ruby vim nano sh zsh git -y
sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
echo 'done'
echo 'Creating alias in .zshrc'
#######################################
echo "alias ll='ls -lha' ">> .zshrc
echo "alias ls='ls -lh' ">> .zshrc
echo "alias rm='srm drf' ">> .zshrc
echo "alias mv='mv -r' ">> .zshrc
echo "alias cls='clear' ">> .zshrc
echo "alias hitc='history -c' ">> .zshrc
echo "alias sht='shutdown -h now' ">> .zshrc
echo "alias np='nmap' ">> .zshrc
ecoh "alias server='php -S 127.0.0.1:8000'"
echo 'done'
#######################################