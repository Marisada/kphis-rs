#import "../utils.typ": parse_d, parse_dt, onlynum
#let age_gr(birth,reg) = {
  let d = (reg - birth).days()
  let y = d/365
  if y == 0 {
    if d <= 30 {"M1"}
    else {"M12"}
  } else if y < 5 {"Y5"}
  else if y <= 10 {"Y10"}
  else if y < 16 {"Y16"}
  else {"A"}
}
#let score_bt(bt, gr) = {
  if ("M1","A").contains(gr) {0}
  else if gr == "M12" {
    if bt < 36.0 {2}
    else if bt < 36.5 {1}
    else if bt <= 38.0 {0}
    else {1}
  } else {
    if bt < 36.0 {1}
    else if bt <= 38.5 {0}
    else {1}
  }
}
#let score_pr(pr, gr) = {
  if ("M1","A").contains(gr) {0}
  else if gr == "M12" {
    if pr <= 59 {3}
    else if pr <= 80 {2}
    else if pr <= 89 {1}
    else if pr <= 130 {0}
    else if pr <= 140 {1}
    else if pr <= 180 {2}
    else {3}
  } else if gr == "Y5" {
    if pr <= 59 {3}
    else if pr <= 69 {1}
    else if pr <= 120 {0}
    else if pr <= 130 {1}
    else if pr <= 150 {2}
    else {3}
  } else if gr == "Y10" {
    if pr <= 40 {3}
    else if pr <= 60 {1}
    else if pr <= 110 {0}
    else if pr <= 120 {1}
    else if pr <= 140 {2}
    else {3}
  } else {
    if pr <= 40 {3}
    else if pr <= 60 {1}
    else if pr <= 100 {0}
    else if pr <= 120 {1}
    else if pr <= 140 {2}
    else {3}
  }
}
#let score_rr(rr, gr) = {
  if ("M1","A").contains(gr) {0}
  else if gr == "M12" {
    if rr <= 9 {3}
    else if rr <= 20 {2}
    else if rr <= 30 {1}
    else if rr <= 50 {0}
    else if rr <= 60 {1}
    else if rr <= 70 {2}
    else {3}
  } else if gr == "Y5" {
    if rr <= 14 {3}
    else if rr <= 19 {1}
    else if rr <= 30 {0}
    else if rr <= 40 {1}
    else if rr <= 45 {2}
    else {3}
  } else if gr == "Y10" {
    if rr <= 11 {3}
    else if rr <= 14 {1}
    else if rr <= 24 {0}
    else if rr <= 30 {1}
    else {3}
  } else {
    if rr <= 9 {3}
    else if rr <= 14 {1}
    else if rr <= 22 {0}
    else if rr <= 25 {1}
    else if rr <= 30 {2}
    else {3}
  }
}
#let score_sbp(sbp, gr) = {
  if ("M1","A").contains(gr) {0}
  else if gr == "M12" {
    if sbp <= 69 {3}
    else if sbp <= 105 {0}
    else if sbp <= 120 {1}
    else {3}
  } else if gr == "Y5" {
    if sbp <= 74 {3}
    else if sbp <= 84 {1}
    else if sbp <= 110 {0}
    else if sbp <= 130 {1}
    else {3}
  } else if gr == "Y10" {
    if sbp <= 79 {3}
    else if sbp <= 89 {1}
    else if sbp <= 120 {0}
    else if sbp <= 130 {1}
    else if sbp <= 150 {2}
    else {3}
  } else {
    if sbp <= 89 {3}
    else if sbp <= 125 {0}
    else if sbp <= 130 {1}
    else if sbp <= 150 {2}
    else {3}
  }
}
#let score_sat(sat) = {
  if sat <= 84 {3}
  else if sat <= 89 {2}
  else if sat <= 93 {1}
  else {0}
}
#let score_o2(o2) = {
  if o2 == 4 {2}
  else if o2 > 0 {1}
  else {0}
}
#let score_avpu(avpu) = {
  if avpu <= 1 {0}
  else if avpu == 2 {1}
  else if avpu == 3 {2}
  else {3}
}
#let score_other(other) = {
  if other <= 1 {0} else {2}
}
#let can_use(birthday, dt) = if birthday == none or dt == none {false} else {
  let gr = age_gr(parse_d(birthday), parse_dt(dt))
  not ("M1","A").contains(gr)
}
#let from_arr(arr, birthday) = {
  let gr = age_gr(parse_d(birthday), parse_dt(arr.at(0)))
  if ("M1","A").contains(gr) {none} else {
    let bt = float(onlynum(arr.at(1)))
    let pr = int(onlynum(arr.at(2)))
    let rr = int(onlynum(arr.at(3)))
    let sbp = int(onlynum(arr.at(4)))
    let sat = int(onlynum(arr.at(10)))
    let o2 = int(onlynum(arr.at(11)))
    let avpu = int(onlynum(arr.at(13)))
    let other = int(onlynum(arr.at(15)))
    let score = if bt == 0 or pr == 0 or rr == 0 or sbp == 0 or sat == 0 or avpu == 0 or other == 0 {
      none
    } else {
      score_bt(bt,gr) + score_pr(pr,gr) + score_rr(rr,gr) + score_sbp(sbp,gr) + score_sat(sat) + score_o2(o2) + score_avpu(avpu) + score_other(other)
    }
    (label: "PEWS", score: score)
  }
}
#let from_vs(vs, birthday) = {
  let gr = age_gr(parse_d(birthday), parse_dt(vs.vs_datetime))
  if ("M1","A").contains(gr) {none} else {
    let bt = float(onlynum(vs.bt))
    let pr = if vs.pr == none {0} else {vs.pr}
    let rr = if vs.rr == none {0} else {vs.rr}
    let sbp = if vs.sbp == none {0} else {vs.sbp}
    let sat = if vs.sat == none {0} else {vs.sat}
    let o2 = if vs.o2_id == none {0} else {vs.o2_id}
    let avpu = if vs.avpu_id == none {0} else {vs.avpu_id}
    let other = if vs.pops_other_id == none {0} else {vs.pops_other_id}
    let score = if bt == 0 or pr == 0 or rr == 0 or sbp == 0 or sat == 0 or avpu == 0 or other == 0 {
      none
    } else {
      score_bt(bt,gr) + score_pr(pr,gr) + score_rr(rr,gr) + score_sbp(sbp,gr) + score_sat(sat) + score_o2(o2) + score_avpu(avpu) + score_other(other)
    }
    (label: "PEWS", score: score)
  }
}