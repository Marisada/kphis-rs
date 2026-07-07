#import "../utils.typ": parse_dt, onlynum
#let score_rr(rr) = if rr > 20 {1} else {0}
#let score_sbp(sbp) = if sbp < 100 {1} else {0}
#let score_gcs(eye, verbal, movement) = if eye == 4 and verbal == 5 and movement == 6 {0} else {1}
#let can_use(birthday, dt) = true
#let from_arr(arr, birthday) = {
  let rr = int(onlynum(arr.at(3)))
  let sbp = int(onlynum(arr.at(4)))
  let eye = int(onlynum(arr.at(17)))
  let verbal = {
    let s = arr.at(18)
    if ("t","T").contains(s) {1} else {int(onlynum(s))}
  }
  let movement = int(onlynum(arr.at(19)))
  let score = if rr == 0 or sbp == 0 or eye == 0 or verbal == 0 or movement == 0 {
    none
  } else {
    score_rr(rr) + score_sbp(sbp) + score_gcs(eye, verbal, movement)
  }
  (label: "qSOFA", score: score)
}
#let from_vs(vs, birthday) = {
  let rr = if vs.rr == none {0} else {vs.rr}
  let sbp = if vs.sbp == none {0} else {vs.sbp}
  let eye = if vs.eye == none {0} else {vs.eye}
  let verbal = if vs.verbal == none {0} else {
    let s = vs.verbal
    if ("t","T").contains(s) {1} else {int(onlynum(s))}
  }
  let movement = if vs.movement == none {0} else {vs.movement}
  let score = if rr == 0 or sbp == 0 or eye == 0 or verbal == 0 or movement == 0 {
    none
  } else {
    score_rr(rr) + score_sbp(sbp) + score_gcs(eye, verbal, movement)
  }
  (label: "qSOFA", score: score)
}