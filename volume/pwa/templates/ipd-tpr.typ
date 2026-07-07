#import "@preview/cetz:0.5.2"
#import "@preview/cetz-plot:0.1.4": plot
#import "templates/utils.typ": api, vnan_is_ipd, get_patient_main, month_th, time_th, parse_d_t, parse_dt, thousands
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let is_ipd = vnan_is_ipd(data.id)
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let vs_d = data.at("vs", default: none)
#if vs_d == none {vs_d = json(api + "ipd/vital-sign?an=" + data.id)}
#let io_d = data.at("io", default: none)
#if io_d == none {io_d = json(api + "ipd/io?an=" + data.id)}
#let op_d = data.at("op", default: none)
#if op_d == none {op_d = json(api + "his/operation-admit-an/" + data.id)}
#let doctor_data = data.at("doctor", default: none)
#if doctor_data == none {doctor_data = json(api + "ipd/doctor-in-charge?an=" + data.id)}
// PREPARED FUNCTIONS
#let dchtype(t) = {
  ((t == "01","แพทย์อนุญาต"),
   (t == "02","ผู้ป่วยปฏิเสธการรักษา"),
   (t == "03","ผู้ป่วยหนีออกจาก รพ."),
   (t == "04","การส่งตัว"),
   (t == "05","เหตุอื่นๆ"),
   (t == "08","การเสียชีวิต ได้ชันสูตรศพ"),
   (t == "09","การเสียชีวิต ไม่ได้ชันสูตรศพ"),
   (true, "ไม่ระบุรายละเอียด")).find(t => t.at(0)).at(1)
}
#let parse_d2(date) = if date == none {none} else {
  let (y,mo,d) = date.split("-")
  datetime(year:int(y),month:int(mo),day:int(d),hour:2,minute:0,second:0)
}
#let dt_date_th(dt) = if dt == none {none} else {
  let (date,time) = dt.split(" ")
  let (y,m,d) = date.split("-")
  str(int(d))+" "+month_th(m)+str(int(y)+543)
}
#let weekly(s,e) = {
  let dt0 = datetime(year:s.year(),month:s.month(),day:s.day(),hour:2,minute:0,second:0)
  let dt1 = datetime(year:e.year(),month:e.month(),day:e.day(),hour:22,minute:0,second:0)
  let wks = calc.ceil((dt1 - dt0).weeks())
  let res = ()
  for w in range(wks) {res.push(dt0 + duration(weeks:w))};res
}
#let p42(dt0) = {
  let res = ()
  for d in range(42) {res.push((d, dt0 + duration(hours:4*d)))};res
}
#let opener = table.cell(stroke:(right:.5pt,rest:none),[])
#let grouper(s,c) = table.cell(rowspan:s,align:center+horizon,rotate(270deg,reflow:true,c))
#let borderless(c) = table.cell(stroke:none,c)
#let underline(c) = table.cell(colspan:2,align:center,stroke:(bottom:.5pt,rest:none),text(size:12pt,c))
#let row_top(s,c) = table.cell(rowspan:s,align:top+right,stroke:(bottom:none),inset:2pt,text(size:12pt,c))
#let row_mid(s,c) = table.cell(rowspan:s,align:horizon+right,stroke:(y:none),inset:2pt,text(size:12pt,c))
#let row_bottom(s,c) = table.cell(rowspan:s,align:bottom+right,stroke:(top:none),inset:2pt,text(size:12pt,c))
#let label_fit(c) = text(size:12pt,align(center,c))
#let label_left(s,c) = table.cell(colspan:s,align:left,c)
#let label_center(s,c) = table.cell(colspan:s,align:center,c)
#let label_lt(s,c) = table.cell(colspan:s,stroke:(left:none,bottom:none),c)
#let label_mid(s,c) = table.cell(colspan:s,stroke:(bottom:none),c)
#let label_rt(s,c) = table.cell(colspan:s,stroke:(right:none,bottom:none),c)
#let input_col(col,s,c) = {
  let x = if c != none {c} else []
  table.cell(colspan:col,align:center+horizon,text(black,size:s*1pt,x))
}
#let input_lt(s,c) = table.cell(colspan:s,align:left+bottom,stroke:(left:none,top:none),text(black,size:18pt,c))
#let input_mid(s,c) = table.cell(colspan:s,align:left+bottom,stroke:(top:none),text(black,size:18pt,c))
#let input_rt(s,c) = table.cell(colspan:s,align:left+bottom,stroke:(right:none,top:none),text(black,size:18pt,c))
#let periods = ([2],[6],[10],[14],[18],[22]).map(label_fit)
#let nmes = ([ด],[ช],[บ]).map(x => label_center(2,x))
#let min_i(ar) = if ar.len() == 0 {0} else {
  ar.map(((i,_)) => i).sorted().first()
}
#let max_v(ar,min) = {
  let a = ar.filter(((_,v)) => v != none).map(((_,v)) => float(v))
  if a.len() == 0 {min} else {a.sorted().last()}
}
#let dx(v) = {(v*11.8pt) + 82pt}
#let dy = 57.5
#let dy_bt(pt) = (dy+((41-pt)*40))*1pt
#let dy_pr(pt) = (dy+((160-pt)*2))*1pt
#let dy_pain(pt) = (dy+200+(10-pt)*4)*1pt
// PREPARED VARIABLES
#let vs_data = if vs_d.len() > 0 {vs_d.sorted(key:vs => parse_dt(vs.vs_datetime))} else {()}
#let io_data = if io_d.len() > 0 {io_d.sorted(key:io => parse_d_t(io.io_date,io.io_time))} else {()}
#let op_data = if op_d.len() > 0 {
  let valid = op_d.filter(op => op.end_datetime != none)
  if valid.len() > 0 {valid.sorted(key:op => parse_dt(op.end_datetime))} else {()}
} else {()}
#let (vstdate, vsttime) = if is_ipd {(pt.regdate,pt.regtime)} else {(pt.vstdate,pt.vsttime)}
#let start_dt = if vstdate != none and vsttime != none {
  parse_d_t(vstdate, vsttime)
} else if vs.len() > 0 {
  parse_dt(vs_data.first().vs_datetime)
} else if io_data.len() > 0 {
  let first = io_data.first()
  parse_d_t(first.io_date,first.io_time)
} else {
  panic("cannot calculate datetime range")
}
#let dch_dt = if is_ipd {
  if pt.dchdate != none and pt.dchtime != none {
    parse_d_t(pt.dchdate, pt.dchtime)
  } else {none}
} else if pt.latest_vs_datetime != none {
    parse_dt(pt.latest_vs_datetime)
} else {none}
#let end_dt = if dch_dt != none {dch_dt} else if vs_data.len() > 0 {
  parse_dt(vs_data.map(vs => vs.vs_datetime).sorted().last())
} else if io_data.len() > 0 {
  let last = io_data.last()
  parse_d_t(last.io_date,last.io_time)
} else {start_dt}
// Create pages data
#let pages = weekly(start_dt,end_dt).enumerate().map(((i,dt0)) => {
  let res = (page:i)
  let d7 = range(7).map(r => dt0 + duration(days:r))
  res.tmp42 = p42(dt0)
  let vs42 = res.tmp42.map(((i,p)) => {
    let in4hr = vs_data.filter(vs => {
      let vs_dt = parse_dt(vs.vs_datetime)
      vs.bt != none and vs.pr != none and vs_dt > p - duration(hours:2) and vs_dt <= p + duration(hours:2)
    })
    let v = if in4hr.len() > 0 {
      let (_,vsx) = in4hr.map(vs => {
        let vs_dt = parse_dt(vs.vs_datetime)
        if vs_dt > p {(vs_dt - p,vs)} else {(p - vs_dt,vs)}
      }).sorted(key:((x,y)) => x).first()
      vsx
    } else {none}
    (i,v)
  })
  let vsp_at(name,min,max) = vs42.filter(((_,v)) => v != none and v.at(name) != none).map(((i,vs)) => {
    let v = float(vs.at(name))
    if v < min {(i,min)} else if v > max {(i,max)} else {(i,v)}
  })
  let vs_at(name) = vs42.map(((i,vs)) => {
    if vs == none {none} else {vs.at(name)}
  })
  let vs12_at(name) = vs42.chunks(3).map(triad => {
    triad.filter(((_,vs)) => vs != none and vs.at(name) != none).map(((_,vs)) => {
      let v = vs.at(name)
      if v == "ใส่สายสวนฯ" {"R"} else {v}
    }).at(0, default: none)
  })
  let io_at(name,type_key,type_value) = {
    d7.map(d => {
      let hr24 = io_data.filter(io => {
        let type_chk = if type_key == none {true} else {
          io.at(type_key) == type_value
        }
        parse_d_t(io.shift_date,"02:00:00.0") == d and type_chk
      })
      if hr24.len() > 0 {
        let night = hr24.filter(io => io.shift == "Night")
        let day = hr24.filter(io => io.shift == "Day")
        let evening = hr24.filter(io => io.shift == "Evening")
        let sum(ar) = ar.map(io => {
          let v = io.at(name)
          if v == none {0} else {float(v)}
        }).sum(default:none)
        (sum(night),sum(day),sum(evening))
      } else {(none,none,none)}
    }).flatten()
  }
  let io_in8 = {
    d7.map(d => {
      let hr24 = io_data.filter(io => parse_d_t(io.shift_date,"02:00:00.0") == d)
      if hr24.len() > 0 {
        let night = hr24.filter(io => io.shift == "Night")
        let day = hr24.filter(io => io.shift == "Day")
        let evening = hr24.filter(io => io.shift == "Evening")
        let sum(ar) = ar.map(io => {
          let oral = io.io_oral_absorb
          let paren = io.io_parenteral_absorb
          (if oral == none {0} else {float(oral)}) + (if paren == none {0} else {float(paren)})
        }).sum(default:none)
      (sum(night),sum(day),sum(evening))
      } else {(none,none,none)}
    }).flatten()
  }
  let io_in24 = {
    d7.map(d => {
      let hr24 = io_data.filter(io => parse_d_t(io.shift_date,"02:00:00.0") == d)
      if hr24.len() > 0 { hr24.map(io => {
        let oral = io.io_oral_absorb
        let paren = io.io_parenteral_absorb
        (if oral == none {0} else {float(oral)}) + (if paren == none {0} else {float(paren)})
      }).sum(default:none)} else {none}
    })
  }
  let io_out24 = {
    d7.map(d => {
      let hr24 = io_data.filter(io => parse_d_t(io.shift_date,"02:00:00.0") == d)
      if hr24.len() > 0 { hr24.map(io => {
        let out = io.io_output_amount
        if out == none {0} else {float(out)}
      }).sum(default:none)} else {none}
    })
  }
  res.date = d7.map(d => if d > end_dt {none} else {dt_date_th(d.display())})
  res.admdate = d7.enumerate().map(((j,d)) => {
    if (d - duration(hours:2)) > end_dt {none} else {(i*7)+j+1}
  })
  let op_dt = op_data.map(op => parse_dt(op.end_datetime))
  res.opdate = d7.map(d => if (d - duration(hours:2)) > end_dt {none} else {
    let at24 = d + duration(hours:22)
    let op_pass = op_dt.filter(dt => dt < at24)
    if op_pass.len() > 0 {
      calc.ceil((at24 - op_pass.last()).days())
    } else {none}
  })
  res.diet = d7.map(d => {
    let hr24 = vs_data.filter(vs => parse_dt(vs.vs_datetime).ordinal() == d.ordinal())
    let diets = hr24.filter(vs => vs.diet != none).map(vs => vs.diet)
    if diets.len() > 0 [#diets.join("/")] else []
  })
  res.bt = vsp_at("bt",35,41)
  res.pr = vsp_at("pr",40,160)
  res.pain = vsp_at("pain",0,10)
  res.rr = vs_at("rr")
  res.sat = vs_at("sat")
  res.sbp = vs_at("sbp")
  res.dbp = vs_at("dbp")
  res.bw_ht = d7.map(d => {
    let hr24 = vs_data.filter(vs => parse_dt(vs.vs_datetime).ordinal() == d.ordinal())
    let bw_ar = hr24.filter(vs => vs.bw != none)
    let bw = if bw_ar.len() > 0 {bw_ar.first().bw} else {none}
    let ht_ar = hr24.filter(vs => vs.height != none)
    let ht = if ht_ar.len() > 0 {ht_ar.first().height} else {none}
    if bw != none or ht != none {
      let bw_i = decimal(bw)
      [#if bw_i < 10 [#str(calc.round(bw_i * 1000)) g] else [#str(calc.round(bw_i,digits:2)) kg] #ht cm]
    } else []
  })
  res.oral = io_at("io_oral_absorb",none,none)
  res.iv = io_at("io_parenteral_absorb","io_parenteral_type","iv")
  res.med = io_at("io_parenteral_absorb","io_parenteral_type","medication")
  res.blood = io_at("io_parenteral_absorb","io_parenteral_type","blood_component")
  res.in_other = io_at("io_parenteral_absorb","io_parenteral_type","other")
  res.in_total8 = io_in8
  res.in_total24 = io_in24
  res.urine = io_at("io_output_amount","io_output_type","urine")
  res.vomit = io_at("io_output_amount","io_output_type","vomit")
  res.gastric = io_at("io_output_amount","io_output_type","gastric_content")
  res.drain = io_at("io_output_amount","io_output_type","drain_tube")
  res.dialysis = io_at("io_output_amount","io_output_type","dyalysis")
  res.out_other = io_at("io_output_amount","io_output_type","other")
  res.out_total8 = io_at("io_output_amount",none,none)
  res.out_total24 = io_out24
  res.urines = vs12_at("urine")
  res.feces = vs12_at("feces")
  res.oas = d7.map(d => {
    let hr24 = vs_data.filter(vs => parse_dt(vs.vs_datetime).ordinal() == d.ordinal())
    let oas_ar = hr24.filter(vs => vs.aggression_oas != none)
    if oas_ar.len() > 0 {oas_ar.first().aggression_oas} else {none}
  })
  res
})
// Plot template
#let line_plot(ymin,ymax,color,data) = if data.len() > 0 {
  cetz.canvas(plot.plot(
    size:(16.95,8.45),
    axis-style:none,
    plot-style:(stroke:(.5pt + color)),
    mark-style:(stroke:none,fill:color),
    x-min:0,x-max:41,
    y-min:ymin,y-max:ymax,{
      plot.add(data,mark:"o",mark-size:3pt)
    }
  ))
}
#let line_plot_pain(color,data) = if data.len() > 0 {
  cetz.canvas(plot.plot(
    size:(16.95,1.4),
    axis-style:none,
    plot-style:(stroke:(.5pt + color)),
    mark-style:(stroke:none,fill:color),
    x-min:0,x-max:41,
    y-min:0,y-max:10,{
      plot.add(data,mark:"o",mark-size:3pt)
    }
  ))
}
// Start render
#set text(
    font: "TH Sarabun New",
    size: 14pt,
    fill:eastern,
)
#set page(
  paper:"a4",
  margin: (x:0.5cm,top:50pt,bottom:100pt),
  header-ascent: 9pt,
  footer-descent: 0pt,
  header: [#h(2fr) #text(size:36pt,weight:700)[*ฟอร์มปรอท*] #h(1fr) #text(size:24pt,weight:700,[แบบ ร.บ. 2 ต. 02])],
  footer: [#table(
    columns:(4fr,2fr,1fr,3fr,3fr),
    rows:(10pt,20pt,10pt,20pt),
    label_lt(2,[Name of Patient]),label_mid(2,[Age]),label_rt(1,[Hospital Number]),
    input_lt(2,[#pt.pname #pt.fname #pt.lname]),input_mid(2,{
      let age_y = pt.age_y
      let age_m = pt.age_m
      let age_d = pt.age_d
      if age_y != none [#age_y ปี] else if age_m != none [#age_m เดือน] else if age_d != none [#age_d วัน] else []
    }),input_rt(1,[#pt.hn]),
    label_lt(1,[Department of Service]),label_mid(2,[Ward]),label_rt(2,[Attending Physician]),
    input_lt(1,pt.at("spclty_name", default: none)),input_mid(2,pt.at("ward_name", default: none)),input_rt(2,{
      if doctor_data.len() > 0 {
        let main = doctor_data.filter(dr => dr.status == "Y")
        if main.len() > 0 [#main.first().doctor_name] else []
      } else []
    }),
  )
  #text(size:33pt,weight:700,baseline:-35pt,align(center,[T.P.R.]))]
)
#set table(
  stroke:(x,y) => (
    left:if calc.rem(x,6) == 4 or x < 5 {.5pt+eastern} else {.25pt+eastern},
    right:if (1,4,20,40,43,44,45).contains(x) {.5pt+eastern} else {.25pt+eastern},
    top:if y == 44 {.5pt+red} else if x > 3 and (calc.rem(y,10) != 4 and y > 4 and y < 65) {.25pt+eastern} else {.5pt+eastern},
    bottom:if (9,145).contains(y) {2pt+red} else {.5pt+eastern},
  ),
  inset:(x,y) => (
    if x > 3 and y > 3 and y < 65 {2pt} else {4pt}
  )
)
// Page template
#let body(p) = [
  #let has_oas = p.oas.filter(x => x != none).len() > 0
  #table(
    columns:((12pt,28pt,24pt,14pt),(11.7pt,)*42).flatten(),
    // Date counter
    opener,label_center(3,align(center,[Date])),..p.date.map(x => input_col(6,14,x)),
    opener,borderless[Day],underline[Admission],..p.admdate.map(x => input_col(6,14,[#x])),
    opener,borderless[after],underline[Operation],..p.opdate.map(x => input_col(6,14,[#x])),
    opener,align(center,[Pulse]),borderless(align(center,[F])),borderless(align(center,[C])),..(periods*7),
    // Plot area
    grouper(60,[( Indicate Pulse in Red )]),
    row_top(3,pad(right:6pt,[160])),row_top(7,[105.8]),row_top(7,[41]),..(([],)*126),
    row_mid(4,[150 --]),..(([],)*168),
    row_mid(6,[140 --]),row_mid(6,[104.0]),row_mid(6,[40]),..(([],)*252),
    row_mid(4,[130 --]),row_mid(14,[102.2]),row_mid(14,[39]),..(([],)*168),
    row_mid(6,[120 --]),..(([],)*252),
    row_mid(4,[110 --]),..(([],)*168),
    row_mid(6,[100 --]),row_mid(6,[100.4]),row_mid(6,[38]),..(([],)*252),
    row_mid(4,[90 --]),row_mid(14,[98.6]),row_mid(14,[37]),..(([],)*168),
    row_mid(6,[80 --]),..(([],)*252),
    row_mid(4,[70 --]),..(([],)*168),
    row_mid(6,[60 --]),row_mid(6,[96.8]),row_mid(6,[36]),..(([],)*252),
    row_mid(4,[50 --]),row_mid(4,[PS]),row_mid(4,text(8pt,[0-10])),..(([],)*168),
    row_bottom(3,pad(right:6pt,[40])),row_bottom(3,[95.0]),row_bottom(3,[35]),..(([],)*126),
    // RR,BP
    opener,label_left(3,[Respirations]),..p.rr.map(x => input_col(1,9,[#x])),
    opener,table.cell(rowspan:2,stroke:none,inset:(top:9pt),text(size:22pt,[B.P.])),underline[Systolic],..p.sbp.map(x => input_col(1,9,[#x])),
    opener,underline[Diastolic],..p.dbp.map(x => input_col(1,9,[#x])),
    opener,label_left(3,[O#sub([2]) saturation]),..p.sat.map(x => input_col(1,9,[#x])),
    opener,label_left(3,[Wt. and Ht.]),..p.bw_ht.map(x => input_col(6,11,[#x])),
    opener,label_left(3,[Diet.]),..p.diet.map(x => input_col(6,8,[#x])),
    // IO header
    opener,label_left(3,[\u{00a0}]),..(nmes*7),
    // Input
    grouper(7,[Fluid Intake]),label_left(3,[Oral Fluid]),..p.oral.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Parenteral]),..p.iv.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Medication]),..p.med.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Blood]),..p.blood.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Other]),..p.in_other.map(x => input_col(2,10,thousands(x))),
    label_center(3,([Total])),..p.in_total8.map(x => input_col(2,10,thousands(x))),
    label_center(3,([Total 24 hr])),..p.in_total24.map(x => input_col(6,14,thousands(x))),
    // Output
    grouper(8,[Fluid Output]),label_left(3,[Urine]),..p.urine.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Emesis]),..p.vomit.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Gastric Fluid]),..p.gastric.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Drainage]),..p.drain.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Dialysis]),..p.dialysis.map(x => input_col(2,10,thousands(x))),
    label_left(3,[Other]),..p.out_other.map(x => input_col(2,10,thousands(x))),
    label_center(3,[Total]),..p.out_total8.map(x => input_col(2,10,thousands(x))),
    label_center(3,[Total 24 hr]),..p.out_total24.map(x => input_col(6,14,thousands(x))),
    // Others
    opener,label_left(3,[Stools]),..p.feces.map(x => input_col(3,11,[#x])),
    opener,label_left(3,[Urine]),..p.urines.map(x => input_col(3,11,[#x])),
    opener,label_left(3,[Medication]),..((input_col(6,14,[]),)*7),
    opener,label_left(3,if has_oas [OAS] else [\u{00a0}]),..if has_oas {
      p.oas.map(x => input_col(6,11,[#x]))
    } else {
      ((input_col(6,14,[]),)*7)
    },
  )
  // Plot position
  #let max_bt = max_v(p.bt,35)
  #let max_pr = max_v(p.pr,40)
  #let max_pain = max_v(p.pain,0)
  #place(alignment.top,dx:dx(min_i(p.bt)),dy:dy_bt(max_bt),line_plot(35,41,blue,p.bt))
  #place(alignment.top,dx:dx(min_i(p.pr)),dy:dy_pr(max_pr),line_plot(40,160,red,p.pr))
  #place(alignment.top,dx:dx(min_i(p.pain)),dy:dy_pain(max_pain),line_plot_pain(purple,p.pain))
  #if dch_dt != none {
    let dch = p.tmp42.find(((i,dt)) => dt > dch_dt)
    if dch != none {
      let (i,_) = dch
      if i > 39 {i = 39}
      place(alignment.top,dx:dx(i)-5pt,dy:58pt,rotate(270deg,reflow:true,box(width:240pt,align(center,text(blue,size:18pt,[จำหน่ายโดย#dchtype(pt.dchtype)#linebreak()เวลา #time_th(pt.dchtime)])))))
    }
  }
]
// render all pages
#for p in pages {
  body(p)
}
