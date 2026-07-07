#import "../utils.typ": parse_d, parse_dt, onlynum
#let age_gr(birth,reg) = {
  let d = (reg - birth).days()
  let y = d/365
  if y == 0 {
    if d <= 6 {"W"}
    else if d <= 30 {"M"}
    else {"Y"}
  } else if y == 1 {"Y"}
  else if y <= 5 {"K"}
  else if y <= 12 {"P"}
  else if y <= 18 {"S"}
  else {"A"}
}
#let score_bt(bt) = {
  if bt < 36.0 or bt > 38.5 {1} else {0}
}
#let score_pr(pr, gr) = {
  let v = if ("W","M").contains(gr) {pr < 100 or pr > 180}
  else if gr == "Y" {pr < 90 or pr > 180}
  else if gr == "K" {pr > 140}
  else if gr == "P" {pr > 130}
  else if gr == "S" {pr > 110}
  else {pr > 90}
  if v {1} else {0}
}
#let score_rr(rr, gr) = {
  let v = ((gr == "W",50),
   (gr == "M",40),
   (gr == "Y",34),
   (gr == "K",22),
   (gr == "P",18),
   (gr == "S",14),
   (gr == "A",20)).find(t => t.at(0)).at(1)
  if rr > v {1} else {0}
}
#let score_wbc(wbc, band, gr) = {
  let v = if band > 10 {true}
  else if gr == "W" {wbc > 34.0}
  else if gr == "M" {wbc < 6.0 or wbc > 19.5}
  else if gr == "Y" {wbc < 6.0 or wbc > 17.5}
  else if gr == "K" {wbc < 6.0 or wbc > 15.5}
  else if gr == "P" {wbc < 4.5 or wbc > 13.5}
  else if gr == "S" {wbc < 4.5 or wbc > 11.0}
  else {wbc < 4.0 or wbc > 12.0}
  if v {1} else {0}
}
#let can_use(birthday, dt) = if birthday == none or dt == none {false} else {
  let gr = age_gr(parse_d(birthday), parse_dt(dt))
  gr != "A"
}
#let from_arr(arr, birthday) = {
  let gr = age_gr(parse_d(birthday), parse_dt(arr.at(0)))
  if gr == "A" {none} else {
    let bt = float(onlynum(arr.at(1)))
    let pr = int(onlynum(arr.at(2)))
    let rr = int(onlynum(arr.at(3)))
    let wbc = float(onlynum(arr.at(16)))
    let band = int(onlynum(arr.at(21)))
    let score = if bt == 0 or pr == 0 or rr == 0 or wbc == 0 {
      none
    } else {
      score_bt(bt) + score_pr(pr, gr) + score_rr(rr, gr) + score_wbc(wbc, band, gr)
    }
    (label: "pSIRS", score: score)
  }
}
#let from_vs(vs, birthday) = {
  let gr = age_gr(parse_d(birthday), parse_dt(vs.vs_datetime))
  if gr == "A" {none} else {
    let bt = float(onlynum(vs.bt))
    let pr = if vs.pr == none {0} else {vs.pr}
    let rr = if vs.rr == none {0} else {vs.rr}
    let wbc = float(onlynum(vs.wbc))
    let band = if vs.band == none {0} else {vs.band}
    let score = if bt == 0 or pr == 0 or rr == 0 or wbc == 0 {
      none
    } else {
      score_bt(bt) + score_pr(pr, gr) + score_rr(rr, gr) + score_wbc(wbc, band, gr)
    }
    (label: "pSIRS", score: score)
  }
}