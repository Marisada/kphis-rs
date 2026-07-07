#import "../utils.typ": parse_d, parse_dt, onlynum
#let score_bt(bt, age_y) = {
  if age_y == 0 {
    if bt < 35.0 {2}
    else if bt < 36.0 {1}
    else if bt <= 37.5 {0}
    else if bt <= 39.0 {1}
    else {2}
  } else {
    if bt < 35.0 {2}
    else if bt < 36.0 {1}
    else if bt < 38.5 {0}
    else if bt <= 40.0 {1}
    else {2}
  }
}
#let score_pr(pr, age_y) = {
  if age_y == 0 {
    if pr <= 89 {2}
    else if pr <= 109 {1}
    else if pr <= 160 {0}
    else if pr <= 180 {1}
    else {2}
  } else if age_y == 1 {
    if pr <= 89 {2}
    else if pr <= 99 {1}
    else if pr <= 150 {0}
    else if pr <= 170 {1}
    else {2}
  } else if age_y < 5 {
    if pr <= 79 {2}
    else if pr <= 94 {1}
    else if pr <= 140 {0}
    else if pr <= 160 {1}
    else {2}
  } else if age_y < 13 {
    if pr <= 69 {2}
    else if pr <= 79 {1}
    else if pr <= 120 {0}
    else if pr <= 150 {1}
    else {2}
  } else {
    if pr <= 49 {2}
    else if pr <= 59 {1}
    else if pr <= 100 {0}
    else if pr <= 110 {1}
    else {2}
  }
}
#let score_rr(rr, age_y) = {
  if age_y == 0 {
    if rr <= 24 {2}
    else if rr <= 29 {1}
    else if rr <= 40 {0}
    else if rr <= 50 {1}
    else {2}
  } else if age_y == 1 {
    if rr <= 19 {2}
    else if rr <= 24 {1}
    else if rr <= 35 {0}
    else if rr <= 50 {1}
    else {2}
  } else if age_y < 5 {
    if rr <= 19 {2}
    else if rr <= 24 {1}
    else if rr <= 30 {0}
    else if rr <= 40 {1}
    else {2}
  } else if age_y < 13 {
    if rr <= 14 {2}
    else if rr <= 19 {1}
    else if rr <= 25 {0}
    else if rr <= 40 {1}
    else {2}
  } else {
    if rr <= 11 {2}
    else if rr <= 14 {1}
    else if rr <= 20 {0}
    else if rr <= 25 {1}
    else {2}
  }
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
  else if avpu == 2 {1}
  else {2}
}
#let score_gut(gut) = {
  if gut <= 1 {0}
  else if gut == 2 {1}
  else {2}
}
#let score_other(other) = {
  if other <= 1 {0}
  else if other == 2 {1}
  else {2}
}
#let can_use(birthday, dt) = if birthday == none or dt == none {false} else {
  let rday = parse_dt(dt)
  let bday = parse_d(birthday)
  let age_y = (rday - bday).days() / 365
  age_y < 16
}
#let from_arr(arr, birthday) = {
  let rday = parse_dt(arr.at(0))
  let bday = parse_d(birthday)
  let age_y = (rday - bday).days() / 365
  if age_y < 16 {
    let bt = float(onlynum(arr.at(1)))
    let pr = int(onlynum(arr.at(2)))
    let rr = int(onlynum(arr.at(3)))
    let sat = int(onlynum(arr.at(10)))
    let breath = int(onlynum(arr.at(12)))
    let avpu = int(onlynum(arr.at(13)))
    let gut = int(onlynum(arr.at(14)))
    let other = int(onlynum(arr.at(15)))
    let score = if bt == 0 or pr == 0 or rr == 0 or sat == 0 or breath == 0 or avpu == 0 or gut == 0 or other == 0 {
      none
    } else {
      score_bt(bt,age_y) + score_pr(pr,age_y) + score_rr(rr,age_y) + score_sat(sat) + score_breath(breath) + score_avpu(avpu) + score_gut(gut) + score_other(other)
    }
    (label: "POPS", score: score)
  } else {none}
}
#let from_vs(vs, birthday) = {
  let rday = parse_dt(vs.vs_datetime)
  let bday = parse_d(birthday)
  let age_y = (rday - bday).days() / 365
  if age_y < 16 {
    let bt = float(onlynum(vs.bt))
    let pr = if vs.pr == none {0} else {vs.pr}
    let rr = if vs.rr == none {0} else {vs.rr}
    let sat = if vs.sat == none {0} else {vs.sat}
    let breath = if vs.breathing_id == none {0} else {vs.breathing_id}
    let avpu = if vs.avpu_id == none {0} else {vs.avpu_id}
    let gut = if vs.gut_feeling_id == none {0} else {vs.gut_feeling_id}
    let other = if vs.pops_other_id == none {0} else {vs.pops_other_id}
    let score = if bt == 0 or pr == 0 or rr == 0 or sat == 0 or breath == 0 or avpu == 0 or gut == 0 or other == 0 {
      none
    } else {
      score_bt(bt,age_y) + score_pr(pr,age_y) + score_rr(rr,age_y) + score_sat(sat) + score_breath(breath) + score_avpu(avpu) + score_gut(gut) + score_other(other)
    }
    (label: "POPS", score: score)
  } else {none}
}