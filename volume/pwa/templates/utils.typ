#import "/customs/config.typ": an-len
#import "@preview/oxifmt:1.0.0": strfmt
#import "@preview/based:0.2.0": base64
#let api = "/api/"
#let vnan_is_ipd(vnan) = if vnan == none {false} else {vnan.len() == an-len}
#let get_patient_main(id) = if id.len() == an-len {
  json(api + "ipd/show-patient-main-an/" + id)
} else {
  json(api + "opd-er/show-patient-main-vn/" + id)
}
#let note_type(ty) = {
  ((ty == "note","Note"),
    (ty == "subjective","Subjective"),
    (ty == "objective","Objective"),
    (ty == "assessment","Assessment"),
    (ty == "plan","Plan"),
    (ty == "problem-list","Problem List"),
    (true, "??")).find(t => t.at(0)).at(1)
}
#let parse_sex(s) = if s == none {none} else {
  ((s == "1","M"),
    (s == "2","F"),
    (true, "")).find(t => t.at(0)).at(1)
}
#let month_th(m) = if m == none {none} else {
  ((m == "01","ม.ค."),
   (m == "02","ก.พ."),
   (m == "03","มี.ค."),
   (m == "04","เม.ย."),
   (m == "05","พ.ค."),
   (m == "06","มิ.ย."),
   (m == "07","ก.ค."),
   (m == "08","ส.ค."),
   (m == "09","ก.ย."),
   (m == "10","ต.ค."),
   (m == "11","พ.ย."),
   (m == "12","ธ.ค."),
   (true, "??")).find(t => t.at(0)).at(1)
}
#let month_th_full(m) = if m == none {none} else {
  ((m == "01","มกราคม"),
   (m == "02","กุมภาพันธ์"),
   (m == "03","มีนาคม"),
   (m == "04","เมษายน"),
   (m == "05","พฤษภาคม"),
   (m == "06","มิถุนายน"),
   (m == "07","กรกฎาคม"),
   (m == "08","สิงหาคม"),
   (m == "09","กันยายน"),
   (m == "10","ตุลาคม"),
   (m == "11","พฤศจิกายน"),
   (m == "12","ธันวาคม"),
   (true, "??")).find(t => t.at(0)).at(1)
}
#let date_th(date) = if date == none {none} else {
  let (y,m,d) = date.split("-")
  str(int(d)) + " " + month_th(m) + str(int(y) + 543)
}
#let date_th_full(date) = if date == none {none} else {
  let (y,m,d) = date.split("-")
  str(int(d)) + " " + month_th_full(m) + " พ.ศ." + str(int(y) + 543)
}
#let time_th(time) = if time == none {none} else {
  let (h,m,_) = time.split(":")
  h + ":" + m + " น."
}
#let datetime_th(dt) = if dt == none {none} else {
  let (d,t) = dt.split()
  date_th(d) + " " + time_th(t)
}
#let datetime_th_checked(dt) = if dt == none {none} else {
  if dt.contains(regex("(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})")) {datetime_th(dt)} else {dt}
}
#let parse_d(date) = if date == none {none} else {
  let (y,mo,d) = date.split("-")
  datetime(year:int(y),month:int(mo),day:int(d),hour:0,minute:0,second:0)
}
#let parse_d_t(date,time) = if date == none or time == none {none} else {
  let (y,mo,d) = date.split("-")
  let t = time.split(".")
  let (h,mi,s) = t.at(0).split(":")
  datetime(year:int(y),month:int(mo),day:int(d),hour:int(h),minute:int(mi),second:int(s))
}
#let parse_dt(dt) = if dt == none {none} else {
  let (d,t) = dt.split()
  parse_d_t(d,t)
}
#let dt2th(dt) = if dt == none {none} else {
  let mo = dt.month()
  let h = dt.hour()
  let m = dt.minute()
  let mos = if mo < 10 {"0" + str(mo)} else {str(mo)}
  let hs = if h < 10 {"0" + str(h)} else {str(h)}
  let ms = if m < 10 {"0" + str(m)} else {str(m)}
  str(dt.day()) + " " + month_th(mos) + str(dt.year() + 543) + " " +  hs + ":" + ms + " น."
}
#let timestamp_th(ts) = if ts == none {none} else {
  let (d,t,z) = ts.split()
  let z3 = z.slice(0,3)
  let zt = if z3 == "+00" [UTC] else {"UTC" + z3}
  date_th(d) + " " + time_th(t) + " (" + zt + ")"
}
#let cid(s) = {
  if s != none and s.len() == 13 {
    [#s.slice(0,1)-#s.slice(1,5)-#s.slice(5,10)-#s.slice(10,12)-#s.slice(12,13)]
  } else {s}
}
#let one_space(s) = if s == none {none} else {s.replace(regex("[ .]{2,}")," ")}
#let if_empty(c,t,f) = if c == none or c == "" [#t] else [#f]
#let onlynum(s) = if s == none {"0"} else {"0" + s.replace(regex("[^.0-9]"),"")}
#let explode_imgs(n,lf,s) = if s == none or n < 1 {none} else [
  #if lf {linebreak()}#s.split(",").map(im => box(inset:(x:1pt,y:-3.5pt),image("/thumbs/"+im,width:(100% / n) - 2pt))).join()#if lf {linebreak()}
]
#let thousands(n) = if n == none [] else [#strfmt("{}",n,fmt-thousands-separator:",")]
#let square_bracket_to_span(s) = if s == none [] else {
  let lts = s.split("[")
  if lts.len() == 0 [#s] else {
    lts.map(lt => {
      let rts = lt.split("]")
      if rts.len() == 2 [#text(fill:red,weight:700,rts.at(0))#rts.at(1)] else [#lt]
    }).join()
  }
}
#let base64_to_byte(s) = if s == none {none} else {
  let split = s.split(",")
  if split.len() == 2 {
    base64.decode(split.at(1))
  } else {none}
}
#let is_num(s) = s.contains(regex("[0-9]")) and s.split(".").len() < 3
#let remove_chars(s) = s.replace(regex("[^0-9.]"),"")
#let is_lab_ab(res,nom) = if res == none or nom == none {"F"} else {
  let res = res.trim()
  let nom = nom.trim()
  let res_lt = res.contains("<")
  let res_gt = res.contains(">")
  let res_eq = res.contains("=")
  if res.contains("-") {"F"} 
  else {
    let res_n = remove_chars(res)
    if is_num(res_n) {
      let res_f = float(res_n)
      if res_lt and not res_eq {
        res_f = res_f - 1.0
      } else if res_gt and not res_eq {
        res_f = res_f + 1.0
      }
      if nom.contains("-") {
        let arr = nom.split("-")
        if arr.len() == 2 {
          let min = remove_chars(arr.at(0))
          let max = remove_chars(arr.at(1))
          if is_num(min) and is_num(max) {
            if res_f < float(min) {"L"} else if res_f > float(max) {"H"} else {"F"}
          } else {"F"}
        } else {"F"}
      } else if nom.contains("<") {
        let arr = nom.split("<")
        if arr.len() == 2 {
          let rt = arr.at(1)
          let rt_n = remove_chars(rt)
          if is_num(rt_n) {
            let rt_f = float(rt_n)
            if rt.starts-with("=") {
              if res_f > rt_f {"T"} else {"F"}
            } else {
              if res_f >= rt_f {"T"} else {"F"}
            }
          } else {"F"}
        } else {"F"}
      } else if nom.contains(">") {
        let arr = nom.split(">")
        if arr.len() == 2 {
          let rt = arr.at(1)
          let rt_n = remove_chars(rt)
          if is_num(rt_n) {
            let rt_f = float(rt_n)
            if rt.starts-with("=") {
              if res_f < rt_f {"T"} else {"F"}
            } else {
              if res_f <= rt_f {"T"} else {"F"}
            }
          } else {"F"}
        } else {"F"}
      } else {"F"}
    } else if res.len() > 0 and nom.len() > 0 {
      if res != nom {"T"} else {"F"}
    } else {"F"}
  }
}