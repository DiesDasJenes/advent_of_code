#!/bin/bash

set -eu -o pipefail

_find_name() {
  local name="$1"

  if [ -n "${name}" ] ; then
    grep -i "${name}" <<EOF | head -1
Philipp Ludewig <@diesdasjenes>
Gabriel Vitali <@gabcvit>
EOF
  else
    echo "tbd"
  fi
}

_extract_abbreviation() {
  local name="$1"
  echo "${name}" | awk -F '[<>]' '{print $2}'
} 

_format_pairing_output() {
  local pair="$1"
  local me="$2"
  
  pair=$(_extract_abbreviation "${pair}")
  me=$(_extract_abbreviation "${me}")
  
  [ -z "${pair}" ] && echo "${me}" || [ -z "${me}" ] && echo "${pair}" || echo "${pair}, ${me}"
}

_create_git_message() {
  local pairing_constellation
  pairing_constellation=$(_format_pairing_output "$1" "$3")
  local day="$2"
  local year="$4"
  current_year=$(date +'%Y')
  
  if [ -n "${pairing_constellation}" ]; then
    echo "[${year}|${day}]: [${pairing_constellation}] Subject "
    echo 
    echo "some context/description"
  elif [ -z "${year}" ]; then
    echo "[${current_year}|${day}] Subject "
    echo 
    echo "some context/description"
  else 
    echo "[${year}|${day}] Subject "
    echo 
    echo "some context/description"
  fi
}


_main() {
  local name="$1"
  local day="$2"
  local me="$3"
  local year="$4"

  me=$(_find_name "${me}")
  
  if [ -z "${LONELY}" ] ; then
    pair=$(_find_name "${name}")
  else
    pair=""
  fi

  if [ -z "${DRY_RUN}" ] ; then
    _create_git_message "${pair}" "${day}" "${me}" "${year}"> ./.git/.gitmessage.txt
    git config commit.template "$PWD/.git/.gitmessage.txt"
  else
    _create_git_message "${pair}" "${day}" "${me}" "${year}"
  fi
}

_usage() {
  cat <<EOF
Usage: $0 [-n] [-p <partial pair name>] [-m <partial personal name>] [-l] [-i <day-number>]

Options:
  -y <current year> The current year 
  -d <current day> The day of the advent of code calendar
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
DAY=
YEAR=
DRY_RUN=
LONELY=

while getopts "hd:np:lm:y:" opt; do
  case "${opt}" in
    h) _usage ;;
    m) ME="${OPTARG}" ;;
    p) PAIR="${OPTARG}" ;;
    d) DAY="${OPTARG}" ;;
    y) YEAR="${OPTARG}" ;;
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

_main "${PAIR}" "${DAY}" "${ME}" "${YEAR}" 