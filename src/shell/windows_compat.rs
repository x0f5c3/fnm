/// On Bash for Windows, we need to convert the path from a Windows-style
/// path to a Unix-style path. This is because Bash for Windows doesn't
/// understand Windows-style paths. We use `cygpath` to do this conversion.
/// If `cygpath` fails, we assume we're not on Bash for Windows and just
/// return the original path.
pub fn maybe_fix_windows_path(path: &str) -> Option<String> {
    if !cfg!(windows) {
        return None;
    }

    use std::process::Command;
    let output = Command::new("cygpath").arg(path).output().ok()?;
    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}
