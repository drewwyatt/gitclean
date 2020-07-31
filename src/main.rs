mod git;

use structopt::StructOpt;

use git::models::GitError;
use git::Git;

#[derive(StructOpt)]
#[structopt(
  about = "Tidy up stale git branches.",
  author = "Drew Wyatt <drew.j.wyatt@gmail.com>",
  name = "git-tidy"
)]
struct Cli {
  #[structopt(
    short,
    long,
    help = "Allow deleting branches irrespective of their apparent merged status"
  )]
  force: bool,

  #[structopt(
    short,
    long,
    help = r#"Present all ": gone" branches in list form, allowing opt-in to deletions"#
  )]
  interactive: bool,

  #[structopt(
    parse(from_os_str),
    default_value = ".",
    help = r#"Path to git repository (defaults to ".")"#
  )]
  path: std::path::PathBuf,
}

fn main() -> Result<(), GitError> {
  let args = Cli::from_args();

  let mut git = Git::from(args.path, args.force, args.interactive);
  let out = git.fetch().prune().list_branches().output()?;

  println!("gone branches: {:?}", git.to_branch_names(out));

  Ok(())
}
