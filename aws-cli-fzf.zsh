AWS_CLI_FZF_TOOLDIR=${AWS_CLI_FZF_TOOLDIR-${0:A:h}}
AWS_CLI_FZF_BASE_DIR="${XDG_DATA_HOME-$HOME/.local/share}/zsh/aws-cli-fzf"
AWS_CLI_FZF_HELP_DIR="${AWS_CLI_FZF_BASE_DIR}/help_files"

function complete_arguments() {
    read _CURSOR _BUFFER <<< $( \
        "${AWS_CLI_FZF_TOOLDIR}/rust/aws_cli_fzf/target/release/aws_cli_fzf" \
        "${AWS_CLI_FZF_HELP_DIR}" \
        "${AWS_CLI_FZF_TOOLDIR}" \
    )
    if [[ -n "${_CURSOR}" ]] && [[ -n "${_BUFFER}" ]]; then
        BUFFER="${_BUFFER} "
        CURSOR=$((${_CURSOR} + 1))
        zle redisplay
    fi
}
zle -N complete_arguments
bindkey "^o" complete_arguments
