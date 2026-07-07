#import "../utils.typ": parse_dt, onlynum
#let score_bt(bt) = {
  if bt < 36.0 {1}
  else if bt <= 38.0 {0}
  else if bt <= 39.0 {1}
  else {2}
}
#let score_pr(pr) = {
  if pr <= 40 {3}
  else if pr <= 50 {1}
  else if pr <= 90 {0}
  else if pr <= 110 {1}
  else if pr <= 139 {2}
  else {3}
}
#let score_rr(rr) = {
  if rr <= 11 {1}
  else if rr <= 20 {0}
  else if rr <= 24 {2}
  else {3}
}
#let score_sbp(sbp) = {
  if sbp <= 90 {3}
  else if sbp <= 100 {2}
  else if sbp <= 110 {1}
  else if sbp <= 219 {0}
  else {3}
}
#let score_sat(sat) = {
  if sat <= 91 {3}
  else if sat <= 93 {2}
  else if sat <= 95 {1}
  else {0}
}
#let score_o2(o2) = {
  if o2 == 0 {0} else {2}
}
#let score_avpu(avpu) = {
  if avpu <= 1 {0} else {3}
}
#let can_use(birthday, dt) = true
#let from_arr(arr, birthday) = {
  let bt = float(onlynum(arr.at(1)))
  let pr = int(onlynum(arr.at(2)))
  let rr = int(onlynum(arr.at(3)))
  let sbp = int(onlynum(arr.at(4)))
  let sat = int(onlynum(arr.at(10)))
  let o2 = int(onlynum(arr.at(11)))
  let avpu = int(onlynum(arr.at(13)))
  let score = if bt == 0 or pr == 0 or rr == 0 or sbp == 0 or sat == 0 or avpu == 0 {
    none
  } else {
    score_bt(bt) + score_pr(pr) + score_rr(rr) + score_sbp(sbp) + score_sat(sat) + score_o2(o2) + score_avpu(avpu)
  }
  (label: "S-NEWS", score: score)
}
#let from_vs(vs, birthday) = {
  let bt = float(onlynum(vs.bt))
  let pr = if vs.pr == none {0} else {vs.pr}
  let rr = if vs.rr == none {0} else {vs.rr}
  let sbp = if vs.sbp == none {0} else {vs.sbp}
  let sat = if vs.sat == none {0} else {vs.sat}
  let o2 = if vs.o2_id == none {0} else {vs.o2_id}
  let avpu = if vs.avpu_id == none {0} else {vs.avpu_id}
  let score = if bt == 0 or pr == 0 or rr == 0 or sbp == 0 or sat == 0 or avpu == 0 {
    none
  } else {
    score_bt(bt) + score_pr(pr) + score_rr(rr) + score_sbp(sbp) + score_sat(sat) + score_o2(o2) + score_avpu(avpu)
  }
  (label: "S-NEWS", score: score)
}