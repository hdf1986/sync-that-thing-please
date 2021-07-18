# Sync that thing please!

This project is an attempt to build an useful system backup tool for my computers (not intended for servers).

I hate that in every computer I've a different .zshrc file, a different set of packages, different configurations, and i wanted to make it consistent across computers, with something that manages to solve the current differences I've across computers.

I could've made a bunch of bash scripts to solve this? yeah, but wanted to keep practicing my Rust so it doesn't get... rusty

# Installation

sh -c "$(curl -fsSL https://raw.github.com/hdf1986/sync-that-thing-please/master/build.sh)"

# TODO's
Yeah, having this in a github project is a todo too

- Folder structure creation & sync [Done]
- Update of it's own code [Done]
- AutoUpdate of it's own code [Done]
- One liner installation [Done]
- Automated installation of snap packages when possible [Done]
- Automated installation of apt packages when possible [To do]
- Autodetection of apt packages not included in config [To do]

- Autosync of generic dot files
- Configurable backup of configs
- Git integrated
- Daemons handling
- Secret avoidance
- Backup tools
  - Ensure git repos are up-to-date & pushed
  - Review typical folders looking for custom files
  - Upload entire folders to google drive
- Sync of /bin executables
- workspace directories handling
- Autoclone selected repos
- Autoinclusion of custom scripts
-


