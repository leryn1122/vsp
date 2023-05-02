use clap::arg;
use clap::builder::EnumValueParser;
use clap::builder::PossibleValue;
use clap::ArgMatches;
use clap::Command;
use clap::ValueEnum;
use CandidateShell::Bash;
use CandidateShell::Fish;
use CandidateShell::PowerShell;
use CandidateShell::Zsh;

pub(crate) fn cli() -> Command {
  Command::new("completion")
    .about("Generate autocompletion scripts for the specified shell")
    .arg(
      arg!([shell] "Shell which autocompletion generated scripts for")
        .required(true)
        .value_parser(EnumValueParser::<CandidateShell>::new()),
    )
}

/// ## How to use auto completion
/// It supports 4 shells currently, refer to `crate::ops::completion::CandidateShell`. No more
/// shells are about to be available in the future, since following shells cover most terminal
/// users.
/// - `bash`
/// - `fish`
/// - `zsh`
/// - `powershell`
///
/// To enable the auto completion requires to install the third party `bash-completion` manually.
///
/// ```bash
/// # Ubuntu
/// sudo apt-get install bash-completion
/// source /etc/bash_completion
///
/// # Mac OS (using `brew` as package manager)
/// brew install bash-completion
/// source $(brew --prefix)/etc/bash_completion
/// ```
///
/// Output the bash completion contents and source it as below. It is trivial.
///
/// ```bash
/// ## Using bash
/// $ source <(vsp completion bash)
///
/// ## Using zsh
/// $ source <(vsp completion zsh)
///
/// ## Using fish
/// $ vsp completion fish > ~/.config/fish/completions/vsp.fish
/// ```
///
/// Additionally, you may want to add the completion source in your `.bashrc` for every session.
///
/// On Windows platform, it is recommended to use powershell as below:
///
/// ```powershell
/// PS> vsp completion powershell > $HOME\.vsp-completion.ps1
/// PS> Add-Content $PROFILE '. $HOME\.vsp-completion.ps1'
/// PS> Add-Content $PROFILE 'if (Get-Command vsp -ErrorAction SilentlyContinue) {
///         vsp completion powershell | Out-String | Invoke-Expression
///     }'
/// ```
///
/// ## Implementation
/// It is a simple implementation, print the mapped completion script based on inline built-in
/// script located at `completion/completion-<shell>.sh`.
pub(crate) fn execute(args: &ArgMatches) -> anyhow::Result<()> {
  let shell = args
    .get_one::<CandidateShell>("shell")
    .expect("Invalid shell");
  match shell {
    Bash => println!("{}", include_str!("completion/completion-bash.sh")),
    Fish => println!("{}", include_str!("completion/completion-fish.sh")),
    Zsh => println!("{}", include_str!("completion/completion-zsh.sh")),
    PowerShell => println!("{}", include_str!("completion/completion-powershell.ps1")),
  };
  Ok(())
}

/// Enumeration of shell which is enabled for auto completion. Implementation for enumeration value
/// parser, see also `clap::builder::EnumValueParser`.
#[derive(Clone)]
pub enum CandidateShell {
  Bash,
  Fish,
  Zsh,
  PowerShell,
}

impl ValueEnum for CandidateShell {
  fn value_variants<'a>() -> &'a [Self] {
    &[Bash, Fish, Zsh, PowerShell]
  }

  fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
    let possible_value = match self {
      Bash => PossibleValue::new("bash"),
      Fish => PossibleValue::new("fish"),
      Zsh => PossibleValue::new("zsh"),
      PowerShell => PossibleValue::new("powershell"),
    };
    Some(possible_value)
  }
}
