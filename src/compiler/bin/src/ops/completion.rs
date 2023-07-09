use clap::arg;
use clap::builder::PossibleValue;
use clap::Args;
use clap::ValueEnum;

use vsp_error::VspResult;
use vsp_support::resources_str;

use crate::ops::Entrypoint;

/// .after_help(resources_str!("completion/help.txt")
#[derive(Args)]
pub struct CandidateArgument {
  /// Shell which autocompletion generated scripts for
  #[arg()]
  shell: CandidateShell,
}

/// # Autocompletion for shell
///
/// ## How to use auto completion
///
/// It supports 4 shells currently. No more shells are about to be available in the
/// future, since following shells cover most terminal users.
///
/// - `bash`
/// - `fish`
/// - `zsh`
/// - `powershell`
///
/// To enable the auto completion requires to install the third party
/// `bash-completion` manually.
///
/// ```bash
/// # Ubuntu
/// sudo apt-get install bash-completion
/// source /etc/bash_completion
///
/// # Mac OS (if using `brew` as package manager)
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
/// Additionally, you may want to add the completion source in your `.bashrc` for
/// every session.
///
/// On Windows platform, it is recommended to use powershell as below:
///
/// ```powershell
/// PS> vsp completion powershell > $HOME\.vsp-completion.ps1
/// PS> Add-Content $PROFILE '. $HOME\.vsp-completion.ps1'
/// PS> Add-Content $PROFILE 'if (Get-Command vsp -ErrorAction SilentlyContinue) {
/// vsp completion powershell | Out-String | Invoke-Expression
/// }'
/// ```
///
/// ## Implementation
///
/// It is a simple implementation, print the mapped completion script based on inline built-in
/// script located at `resources/completion/completion-<shell>.sh`.
/// Available shell candidates refer to `crate::ops::completion::CandidateShell`.
impl Entrypoint for CandidateArgument {
  fn entrypoint(&mut self) -> VspResult<()> {
    match self.shell {
      CandidateShell::Bash => println!("{}", resources_str!("completion/completion-bash.sh")),
      CandidateShell::Fish => println!("{}", resources_str!("completion/completion-fish.sh")),
      CandidateShell::Zsh => println!("{}", resources_str!("completion/completion-zsh.sh")),
      CandidateShell::PowerShell => {
        println!("{}", resources_str!("completion/completion-powershell.ps1"))
      }
    };
    Ok(())
  }
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
    &[
      CandidateShell::Bash,
      CandidateShell::Fish,
      CandidateShell::Zsh,
      CandidateShell::PowerShell,
    ]
  }

  fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
    let possible_value = match self {
      CandidateShell::Bash => PossibleValue::new("bash"),
      CandidateShell::Fish => PossibleValue::new("fish"),
      CandidateShell::Zsh => PossibleValue::new("zsh"),
      CandidateShell::PowerShell => PossibleValue::new("powershell"),
    };
    Some(possible_value)
  }
}
