#import "../utils.typ": parse_d, parse_dt, onlynum
#let age_gr(birth,reg) = {
  let d = (reg - birth).days()
  let y = d/365
  if y == 0 {
    if d <= 90 {"M3"}
    else if d <= 180 {"M6"}
    else if d <= 270 {"M9"}
    else {"M12"}
  } else if y == 1 {
    if d < 548 {"M18"}
    else {"M24"}
  } else if y == 2 {"Y3"}
  else if y == 3 {"Y4"}
  else if y < 6 {"Y6"}
  else if y < 8 {"Y8"}
  else if y < 12 {"Y12"}
  else if y < 15 {"Y15"}
  else if y < 18 {"Y18"}
  else {"A"}
}
#let score_pr(pr, gr) = {
  let v = ((gr == "M3",186),
   (gr == "M6",182),
   (gr == "M9",178),
   (gr == "M12",176),
   (gr == "M18",173),
   (gr == "M24",170),
   (gr == "Y3",167),
   (gr == "Y4",164),
   (gr == "Y6",161),
   (gr == "Y8",155),
   (gr == "Y12",147),
   (gr == "Y15",138),
   (gr == "Y18",132),
   (gr == "A",999)).find(t => t.at(0)).at(1)
  if pr > v {1} else {0}
}
#let score_rr(rr, gr) = {
  let v = ((gr == "M3",76),
   (gr == "M6",71),
   (gr == "M9",67),
   (gr == "M12",63),
   (gr == "M18",60),
   (gr == "M24",57),
   (gr == "Y3",54),
   (gr == "Y4",52),
   (gr == "Y6",50),
   (gr == "Y8",46),
   (gr == "Y12",41),
   (gr == "Y15",35),
   (gr == "Y18",32),
   (gr == "A",20)).find(t => t.at(0)).at(1)
  if rr > v {1} else {0}
}
#let score_avpu(avpu) = if avpu > 1 {1} else {0}
#let score_crt(crt) = if crt >= 3 {1} else {0}
#let can_use(birthday, dt) = if birthday == none or dt == none {false} else {
  let gr = age_gr(parse_d(birthday), parse_dt(dt))
  gr != "A"
}
#let from_arr(arr, birthday) = {
  let gr = age_gr(parse_d(birthday), parse_dt(arr.at(0)))
  if gr == "A" {none} else {
    let pr = int(onlynum(arr.at(2)))
    let rr = int(onlynum(arr.at(3)))
    let avpu = int(onlynum(arr.at(13)))
    let crt = int(onlynum(arr.at(20)))
    let score = if pr == 0 or rr == 0 or avpu == 0 or crt == 0 {
      none
    } else {
      score_pr(pr, gr) + score_rr(rr, gr) + score_avpu(avpu) + score_crt(crt)
    }
    (label: "LqSOFA", score: score)
  }
}
#let from_vs(vs, birthday) = {
  let gr = age_gr(parse_d(birthday), parse_dt(vs.vs_datetime))
  if gr == "A" {none} else {
    let pr = if vs.pr == none {0} else {vs.pr}
    let rr = if vs.rr == none {0} else {vs.rr}
    let avpu = if vs.avpu_id == none {0} else {vs.avpu_id}
    let crt = if vs.crt == none {0} else {vs.crt}
    let score = if pr == 0 or rr == 0 or avpu == 0 or crt == 0 {
      none
    } else {
      score_pr(pr, gr) + score_rr(rr, gr) + score_avpu(avpu) + score_crt(crt)
    }
    (label: "LqSOFA", score: score)
  }
}