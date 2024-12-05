#!/bin/zsh


# FOR ENVIROMENT VARIABLES:
#
# export VAR_NAME="value" is to set a variable for whole system
# VAR_NAME="value" is to set a variable for the current shell

## DOCUMENT SOURCING

# Scripts doc for environment variables and functions
source ~/.scripts/main.sh


## ENVIRONMENT VARIABLES

# env api keys
source ~/.env

# editor:
export EDITOR="nvim"



## PACKAGES AND PLUGINS

# HOMEBREW
source $HOMEBREW_PREFIX/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh
source $HOMEBREW_PREFIX/share/zsh-autosuggestions/zsh-autosuggestions.zsh

# Oh My Zsh
export ZSH="$HOME/.oh-my-zsh"
plugins=(git)
source $ZSH/oh-my-zsh.sh
source <(fzf --zsh)

# Spiceify
export PATH=$PATH:/Users/gassandrid/.spicetify

# Starship
export STARSHIP_CACHE=~/.starship/cache
eval "$(starship init zsh)"

# Pipx path perms
export PATH="$PATH:/Users/gassandrid/.local/bin"

# Zoxide
eval "$(zoxide init zsh)"
alias cd="z"

# wierd fix for opencv rust bindings
export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/usr/lib/"

# fasder
eval "$(fasder --init auto aliases)"



## ALIASES

# copilot
alias cop="gh copilot suggest"

# switching github accounts
alias switch="gh auth switch"

# because I'm lazy
alias editzsh="nvim ~/.zshrc"

# because python environemnts suck
alias venv_init="python3 -m venv ./ && source ./bin/activate"
alias venv_source="source ./bin/activate"
alias venv_main="source ~/python_main_venv/bin/activate"

# ewantype
alias ewantype="tt"

# lazygit
alias lg="lazygit"



# Sketchybar brew icon updater script
# keeps track of how many outdated brew packages there are
function brew() {
  command brew "$@" 

  if [[ $* =~ "upgrade" ]] || [[ $* =~ "update" ]] || [[ $* =~ "outdated" ]]; then
    sketchybar --trigger brew_update
  fi
}

# clear the screen
clear
export PATH="/opt/homebrew/opt/openjdk/bin:$PATH"
venv_main

export XDG_CONFIG_HOME="/Users/gassandrid/.config"

# nu shell, without setting it as our default shell
nu

