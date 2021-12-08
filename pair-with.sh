#!/bin/bash

set -eu -o pipefail

_find_name() {
  local name="$1"

  if [ -n "${name}" ] ; then
    cat <<EOF |grep -i "${name}" | head -1
Philipp Ludewig <@diesdasjenes>
Gabriel Vitali <@gabcvit>
EOF
  else
    echo "tbd"
  fi
}

_format_issue() {
  local issue="$1"
  local jira_board="AoC"

  if [ -z "${issue}" ] ; then
    echo "${jira_board}-???"
  elif [[ "${issue}" = *${jira_board}* ]] ; then
    echo "${issue}"
  else
    echo "${jira_board}-${issue}"
  fi
}

_extract_abbreviation() {
  local name="$1"
  echo ${name} | awk -F '[<>]' '{print $2}'
} 

_format_pairing_output() {
  local pair="$1"
  local me="$2"
  
  pair=$(_extract_abbreviation "${pair}")
  me=$(_extract_abbreviation "${me}")
  
  if [ -z "${pair}" ] ; then
    echo ${me} 
  elif  [ -z "${me}" ]  ; then
    echo ${pair}
  else
    echo "${pair}, ${me}"
  fi

}

_create_git_message() {
  local pairing_constellation=$(_format_pairing_output "$1" "$3")
  local issue="$2"
 
  echo "${issue}: [${pairing_constellation}] Subject "
  echo 
  echo "some context/description"
}

_main() {
  local name="$1"
  local issue="$2"
  local me="$3"

  me=$(_find_name "${me}")
  issue=$(_format_issue "${issue}")
  
  if [ -z "${LONELY}" ] ; then
    pair=$(_find_name "${name}")
  else
    pair=""
  fi

  if [ -z "${DRY_RUN}" ] ; then
    _create_git_message "${pair}" "${issue}" "${me}" > ./.git/.gitmessage.txt
    git config commit.template "$PWD/.git/.gitmessage.txt"
  else
    _create_git_message "${pair}" "${issue}" "${me}"
  fi
}

_usage() {
  cat <<EOF
Usage: $0 [-n] [-p <partial pair name>] [-m <partial personal name>] [-l] [-i <issue-number>]

Options:
  -i <issue number> The JIRA issue number
  -p <partial pair name> Part of the pair name
  -m <partial personal name> Part of your name
  -l lonely - you don't have a pair :'-/
  -n dry run - only print template


This will create a file .git/.gitmessage.txt and set commit.template option to it.

EOF
  exit 1
}

ME=
PAIR=
ISSUE=
DRY_RUN=
LONELY=

while getopts "hi:np:lm:" opt; do
  case "${opt}" in
    h) _usage ;;
    m) ME="${OPTARG}" ;;
    p) PAIR="${OPTARG}" ;;
    i) ISSUE="${OPTARG}" ;;
    n) DRY_RUN=1 ;;
    l) LONELY=1 ;;
    *) _usage ;;
  esac
done
shift $((OPTIND-1))

if [ ! -d ./.git ] ; then
  echo "This is not a git directory!" 1>&1
  _usage
fi

_main "${PAIR}" "${ISSUE}" "${ME}"
