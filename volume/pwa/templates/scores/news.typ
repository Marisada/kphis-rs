#import "../utils.typ": parse_d, parse_dt, onlynum
#let score_bt(bt) = {
  if bt < 35.0 {2}
  else if bt < 36.5 {1}
  else if bt < 37.5 {0}
  else if bt < 38.0 {1}
  else {2}
}
#let score_pr(pr) = {
  if pr <= 59 {2}
  else if pr <= 100 {1}
  else if pr <= 159 {0}
  else if pr <= 180 {1}
  else {2}
}
#let score_rr(rr) = {
  if rr <= 29 {2}
  else if rr <= 39 {1}
  else if rr <= 60 {0}
  else if rr <= 80 {1}
  else {2}
}
#let score_sat(sat) = {
  if sat <= 89 {2}
  else if sat <= 94 {1}
  else {0}
}
#let score_breath(breath) = {
  if breath <= 1 {0}
  else if breath <= 3 {1}
  else {2}
}
#let score_avpu(avpu) = {
  if avpu <= 1 {0}
  else if avpu <= 3 {1}
  else {2}
}
#let score_gut(gut) = {
  if gut <= 1 {0}
  else if gut <= 3 {1}
  else {2}
}
#let can_use(birthday, dt) = if birthday == none or dt == none {false} else {
  let rday = parse_dt(dt)
  let bday = parse_d(birthday)
  let age_d = (rday - bday).days()
  age_d < 31
}
#let from_arr(arr, birthday) = {
  let rday = parse_dt(arr.at(0))
  let bday = parse_d(birthday)
  let age_d = (rday - bday).days()
  if age_d < 31 {
    let bt = float(onlynum(arr.at(1)))
    let pr = int(onlynum(arr.at(2)))
    let rr = int(onlynum(arr.at(3)))
    let sat = int(onlynum(arr.at(10)))
    let breath = int(onlynum(arr.at(12)))
    let avpu = int(onlynum(arr.at(13)))
    let gut = int(onlynum(arr.at(14)))
    let score = if bt == 0 or pr == 0 or rr == 0 or sat == 0 or breath == 0 or avpu == 0 or gut == 0 {
      none
    } else {
      score_bt(bt) + score_pr(pr) + score_rr(rr) + score_sat(sat) + score_breath(breath) + score_avpu(avpu) + score_gut(gut)
    }
    (label: "NEWS", score: score)
  } else {none}
}
#let from_vs(vs, birthday) = {
  let rday = parse_dt(vs.vs_datetime)
  let bday = parse_d(birthday)
  let age_d = (rday - bday).days()
  if age_d < 31 {
    let bt = float(onlynum(vs.bt))
    let pr = if vs.pr == none {0} else {vs.pr}
    let rr = if vs.rr == none {0} else {vs.rr}
    let sat = if vs.sat == none {0} else {vs.sat}
    let breath = if vs.breathing_id == none {0} else {vs.breathing_id}
    let avpu = if vs.avpu_id == none {0} else {vs.avpu_id}
    let gut = if vs.gut_feeling_id == none {0} else {vs.gut_feeling_id}
    let score = if bt == 0 or pr == 0 or rr == 0 or sat == 0 or breath == 0 or avpu == 0 or gut == 0 {
      none
    } else {
      score_bt(bt) + score_pr(pr) + score_rr(rr) + score_sat(sat) + score_breath(breath) + score_avpu(avpu) + score_gut(gut)
    }
    (label: "NEWS", score: score)
  } else {none}
}