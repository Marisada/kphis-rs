#import "../utils.typ": parse_dt, onlynum
#let score_bt(bt) = {
  if bt < 36.0 or bt > 38.0 {1} else {0}
}
#let score_pr(pr) = {
  if pr > 90 {1} else {0}
}
#let score_rr(rr) = {
  if rr > 20 {1} else {0}
}
#let score_wbc(wbc, band) = {
  if band > 10 or wbc < 4.0 or wbc > 12.0 {1} else {0}
}
#let can_use(birthday, dt) = true
#let from_arr(arr, birthday) = {
  let bt = float(onlynum(arr.at(1)))
  let pr = int(onlynum(arr.at(2)))
  let rr = int(onlynum(arr.at(3)))
  let wbc = float(onlynum(arr.at(16)))
  let band = int(onlynum(arr.at(21)))
  let score = if bt == 0 or pr == 0 or rr == 0 or wbc == 0 {
    none
  } else {
    score_bt(bt) + score_pr(pr) + score_rr(rr) + score_wbc(wbc, band)
  }
  (label: "SIRS", score: score)
}
#let from_vs(vs, birthday) = {
  let bt = float(onlynum(vs.bt))
  let pr = if vs.pr == none {0} else {vs.pr}
  let rr = if vs.rr == none {0} else {vs.rr}
  let wbc = float(onlynum(vs.wbc))
  let band = if vs.band == none {0} else {vs.band}
  let score = if bt == 0 or pr == 0 or rr == 0 or wbc == 0 {
    none
  } else {
    score_bt(bt) + score_pr(pr) + score_rr(rr) + score_wbc(wbc, band)
  }
  (label: "SIRS", score: score)
}