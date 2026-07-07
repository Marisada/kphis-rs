#import "../utils.typ": parse_dt, onlynum
#let score_bt(bt) = {
  if bt <= 35.0 {2}
  else if bt <= 36.0 {1}
  else if bt <= 38.0 {0}
  else if bt < 38.5 {1}
  else {2}
}
#let score_pr(pr) = {
  if pr <= 40 {3}
  else if pr <= 50 {1}
  else if pr <= 100 {0}
  else if pr <= 120 {1}
  else if pr <= 139 {2}
  else {3}
}
#let score_rr(rr,res) = {
  if rr <= 8 {3}
  else if rr <= 20 {if res {2} else {0}}
  else if rr <= 25 {if res {2} else {1}}
  else if rr <= 34 {2}
  else {3}
}
#let score_sbp(sbp,ino) = {
  if ino {3}
  else if sbp <= 80 {3}
  else if sbp <= 90 {2}
  else if sbp <= 100 {1}
  else if sbp <= 180 {0}
  else if sbp <= 199 {1}
  else {2}
}
#let score_cons(cons) = {
  if cons <= 1 {0}
  else if cons <= 4 {1}
  else if cons == 5 {2}
  else {3}
}
#let score_urine(ua,ud) = {
  if ua == 2 {if ud == 1 {1} else {0}}
  else if ua == 3 {if ud == 1 {2} else if ud == 2 {1} else {0}}
  else if ua == 4 {if (1,2).contains(ud) {2} else if ud == 3 {1} else {0}}
  else if ua == 5 {if (1,2,3).contains(ud) {2} else {0}}
  else if ua == 6 {if (1,2,3).contains(ud) {2} else if ud == 4 {1} else {0}}
  else if ua == 7 {2}
  else {0}
}
#let can_use(birthday, dt) = true
#let from_arr(arr, birthday) = {
  let bt = float(onlynum(arr.at(1)))
  let pr = int(onlynum(arr.at(2)))
  let rr = int(onlynum(arr.at(3)))
  let sbp = int(onlynum(arr.at(4)))
  let ino = arr.at(5) == "Y"
  let res = arr.at(6) == "Y"
  let cons = int(onlynum(arr.at(7)))
  let ua = int(onlynum(arr.at(8)))
  let ud = int(onlynum(arr.at(9)))
  let score = if bt == 0 or pr == 0 or rr == 0 or sbp == 0 or cons == 0 or ua == 0 or ud == 0 {
    none
  } else {
    score_bt(bt) + score_pr(pr) + score_rr(rr,res) + score_sbp(sbp,ino) + score_cons(cons) + score_urine(ua,ud)
  }
  (label: "MEWS", score: score)
}
#let from_vs(vs, birthday) = {
  let bt = float(onlynum(vs.bt))
  let pr = if vs.pr == none {0} else {vs.pr}
  let rr = if vs.rr == none {0} else {vs.rr}
  let sbp = if vs.sbp == none {0} else {vs.sbp}
  let ino = vs.inotrope == "Y"
  let res = vs.respirator == "Y"
  let cons = if vs.conscious_id == none {0} else {vs.conscious_id}
  let ua = if vs.urine_amount == none {0} else {vs.urine_amount}
  let ud = if vs.urine_duration == none {0} else {vs.urine_duration}
  let score = if bt == 0 or pr == 0 or rr == 0 or sbp == 0 or cons == 0 or ua == 0 or ud == 0 {
    none
  } else {
    score_bt(bt) + score_pr(pr) + score_rr(rr,res) + score_sbp(sbp,ino) + score_cons(cons) + score_urine(ua,ud)
  }
  (label: "MEWS", score: score)
}