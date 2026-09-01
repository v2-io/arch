# cpd — cd to a census project by name (`cpd arch/utf`).
# A function, not a bin script: a child process can't change the shell's cwd.
# Sourced from ~/.config/zsh/interactive/ (symlink 25-cpd.zsh → this file).
cpd() {
  local dir
  dir=$(pd "$1") || return
  cd "$dir"
}

_cpd_complete() {
  local -a names
  names=(${(f)"$(projects -n 2>/dev/null)"})
  compadd -a names
}
(( $+functions[compdef] )) && compdef _cpd_complete cpd pd
