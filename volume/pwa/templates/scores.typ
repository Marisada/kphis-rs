#import "scores/mews.typ"
#import "scores/snews.typ"
#import "scores/pops.typ"
#import "scores/pews.typ"
#import "scores/news.typ"
#import "scores/qsofa.typ"
#import "scores/lqsofa.typ"
#import "scores/sirs.typ"
#import "scores/psirs.typ"
#import "/customs/config.typ": score-ews, score-qsofa, score-sirs

#let import_finder(i) = {
  ((i == "MEWS",mews),
   (i == "S-NEWS",snews),
   (i == "POPS",pops),
   (i == "PEWS",pews),
   (i == "NEWS",news),
   (i == "qSOFA",qsofa),
   (i == "LqSOFA",lqsofa),
   (i == "SIRS",sirs),
   (i == "pSIRS",psirs),
   (true, none)).find(t => t.at(0)).at(1)
}
#let label(kind, birthday, dt) = {
  let l2r = if kind == "ews" {score-ews}
    else if kind == "qsofa" {score-qsofa}
    else if kind == "sirs" {score-sirs}
    else {()}
  for score in l2r {
    let b = import_finder(score).can_use(birthday, dt)
    if b {
      return score
    }
  }
  return none
}
#let score(kind, pipe, birthday) = {
  let l2r = if kind == "ews" {score-ews}
    else if kind == "qsofa" {score-qsofa}
    else if kind == "sirs" {score-sirs}
    else {()}
  let arr = pipe.split("|")
  if arr.len() < 22 {none} else {
    for score in l2r {
      let s = import_finder(score).from_arr(arr, birthday)
      if s != none {
        return s
      }
    }
    return none
  }
}
#let score_vs(kind, vs, birthday) = {
  let l2r = if kind == "ews" {score-ews}
    else if kind == "qsofa" {score-qsofa}
    else if kind == "sirs" {score-sirs}
    else {()}
  for score in l2r {
    let s = import_finder(score).from_vs(vs, birthday)
    if s != none {
      return s
    }
  }
}