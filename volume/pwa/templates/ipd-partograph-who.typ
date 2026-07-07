#import "@preview/cetz:0.5.2"
#import "@preview/cetz-plot:0.1.4": plot
#import "@preview/tiptoe:0.4.0"
#import "templates/utils.typ": api, get_patient_main, datetime_th, date_th, month_th, time_th, parse_d_t, parse_dt
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let vs_d = data.at("vs", default: none)
#if vs_d == none {vs_d = json(api + "ipd/vital-sign?an=" + data.id)}
// PREPARED FUNCTIONS
#let to_int(s) = if s == none {0} else {
  let r = s.replace(regex("\D"),"")
  if r == "" {0} else {int(r)}
}
#let int_num(s) = if s == none {0} else {
  let sp = s.split("'")
  if sp.len() > 1 {
    let m = to_int(sp.at(0))
    let s = to_int(sp.at(1))
    if m > 0 or s > 0 {600 / (m*60+s)} else {0}
  } else {
    let s = to_int(sp.at(0))
    if s > 0 {600/s} else {0}
  }
}
#let table_h(c) = [#align(center,strong(c))]
#let label_note(l,n) = [#text(weight:700,l) #n]
#let txtb(b,c) = text(size:14pt,top-edge:1pt,baseline:b,c)
#let dt_hr(dt) = dt - duration(minutes:dt.minute()) - duration(seconds:dt.second())
#let near_hr(vs) = {
  let dt = parse_dt(vs.vs_datetime)
  if dt.minute() > 29 {dt_hr(dt) + duration(hours:1)} 
  else {dt_hr(dt)}
}
#let add_rdt(vs) = {
  let dt = parse_dt(vs.vs_datetime)
  vs.adt = dt
  vs.rdt = if dt.minute() < 15 {dt_hr(dt)}
  else if dt.minute() < 45 {dt_hr(dt) + duration(minutes:30)}
  else {dt_hr(dt) + duration(hours:1)};vs
}
#let fill_1 = tiling(size:(4pt,4pt),place(dx:2pt,dy:2pt,square(size:2pt,fill:luma(120))))
#let fill_2 = tiling(relative:"parent",size:(3pt,3pt),line(stroke:.5pt,start:(0%,100%),end:(100%,0%)))
#let dura_fill(d) = {
  ((d < 20,fill_1),
   (d <= 40,fill_2),
   (true,luma(150))).find(t => t.at(0)).at(1)
}
#let max_v(ar,min) = {
  let a = ar.filter(((_,v)) => v != none).map(((_,v)) => float(v))
  if a.len() == 0 {min} else {a.sorted().last()}
}
#let dy(ofs,max,mul,v) = (ofs+(max - max_v(v,max))*mul)*1pt
#let plot_dot(x,y,col,ymin,ymax,color,data) = if data.len() > 0 {
  cetz.canvas(plot.plot(
    size:(x,y),
    axis-style:none,
    plot-style:(stroke:(1pt + color)),
    mark-style:(stroke:none,fill:color),
    x-min:0,x-max:col,
    y-min:ymin,y-max:ymax,{
      plot.add(data,mark:"o",mark-size:4pt)
    }
  ))
}
#let plot_x(x,y,col,ymin,ymax,color,data) = if data.len() > 0 {
  cetz.canvas(plot.plot(
    size:(x,y),
    axis-style:none,
    plot-style:(stroke:(1pt+color)),
    mark-style:(stroke:2pt+color,fill:none),
    x-min:0,x-max:col,
    y-min:ymin,y-max:ymax,{
      plot.add(data,mark:"x",mark-size:8pt)
    }
  ))
}
#let plot_o(x,y,col,ymin,ymax,color,data) = if data.len() > 0 {
  cetz.canvas(plot.plot(
    size:(x,y),
    axis-style:none,
    plot-style:(stroke:(1pt+color)),
    mark-style:(stroke:2pt+color,fill:white),
    x-min:0,x-max:col,
    y-min:ymin,y-max:ymax,{
      plot.add(data,mark:"o",mark-size:8pt)
    }
  ))
}
#let cell(c) = table.cell(align:center+horizon,[#c])
#let tbl_row(vs) = (cell(datetime_th(vs.vs_datetime)),
  cell(vs.bt),cell(vs.pr),cell(vs.rr),cell([#vs.sbp/#vs.dbp]),cell(vs.lr_pos),cell(vs.lr_fsh),
  cell(vs.lr_cer),cell(vs.lr_int),cell(vs.lr_dur),cell(vs.lr_eff),cell(vs.lr_sta_name),
  cell(vs.lr_mem_name),cell(vs.lr_af),table.cell(align:horizon,vs.update_opduser_name)
)
// PREPARED VARIABLES
#let vs_d = vs_d.sorted(key: vs => parse_dt(vs.vs_datetime))
#let vs_act_pos = vs_d.position(vs => to_int(vs.lr_cer) > 2)
#let (vs_l,vs_a) = if vs_d.len() == 0 {((),())} else if vs_act_pos == none {
  let last_vs = near_hr(vs_d.last())
  (vs_d.filter(vs => parse_dt(vs.vs_datetime) > last_vs - duration(hours:8,minutes:15)).map(add_rdt).dedup(key:vs => vs.rdt),())
} else {
  let a = vs_d.slice(vs_act_pos).map(add_rdt).dedup(key:vs => vs.rdt)
  let a_st = a.first()
  let a_stt = near_hr(a_st)
  let cx_over_3 = int(a_st.lr_cer) - 3
  let l = vs_d.slice(0,vs_act_pos).filter(vs => {
    parse_dt(vs.vs_datetime) > a_stt - duration(hours:8+cx_over_3,minutes:15)
  }).map(add_rdt).dedup(key:vs => vs.rdt)
  let need_tr = if l.len() > 0 and a.len() > 0 {
    a.first().rdt - l.last().rdt > duration(minutes:30)
  } else {false}
  (if need_tr or l.len() == 0 {l.push(a.first());l} else {l},a)
}
#let apply_rdt_map(vss,rdt_map) = vss.map(vs => {
  let kp = rdt_map.find(((d,i)) => d == vs.rdt)
  vs.i = if kp == none {none} else {let (d,i) = kp;i};vs
})
#let apply_48_at(vss,at) = range(48).map(i => {
  let kp = vss.find(vs => vs.i == i)
  if kp == none [] else {table.cell(align:center,inset:(top:3pt,bottom:0pt),text(10pt,[#kp.at(at)]))}
})
#let apply_48_nx_at(vss,at) = range(48).map(i => {
  let kp = vss.find(vs => vs.i == i)
  if kp == none {table.cell(stroke:(x:none),[])} else {table.cell(stroke:(x:none),align:center,inset:(top:3pt,bottom:0pt),text(10pt,kp.at(at)))}
})
#let apply_48_to_24(vss,at) = range(48).map(i => {
  let kp = vss.find(vs => vs.i == i)
  if kp != none {kp.at(at)} else {none}
}).chunks(2).map(c => table.cell(align:center,inset:(x:0pt,y:5pt),text(12pt,[
  #let val = c.filter(x => x != none)
  #if val.len() > 0 [#val.last()] else [#v(5pt)]
])))
#let vs_l = if vs_l.len() > 0 {
  let start_l = vs_l.first().rdt
  let end_l = vs_l.last().rdt
  let max_i = int((end_l - start_l) / duration(minutes:30))
  let rdt_map = range(max_i+1).map(i => (start_l + duration(minutes:(30*i)),i))
  apply_rdt_map(vs_l,rdt_map)
} else {vs_l}
#let vs_a = if vs_a.len() > 0 {
  let start = vs_a.first()
  let start_a = start.rdt
  let end_a = vs_a.last().rdt
  let start_cx = to_int(start.lr_cer)
  let start_i = (start_cx+5)*2
  let max_i = start_i + int((end_a - start_a) / duration(minutes:30))
  let rdt_map = range(start_i,max_i+1).map(i => (start_a + duration(minutes:(30*(i - start_i))),i))
  apply_rdt_map(vs_a,rdt_map)
} else {vs_a}
#let start_l = if vs_l.len() > 0 {vs_l.first().adt.display("[hour]:[minute]")} else {none}
#let (start_at,act_cx,end_at) = if vs_a.len() > 0 {
  let z = vs_a.first()
  (z.i,int(z.lr_cer),vs_a.last().i)
} else {(0,0,0)}
#let end_lt = if vs_l.len() > 0 {vs_l.last().i} else {0}
#let all = vs_l + vs_a
#let (all_0i,all_0dt) = if all.len() > 0 {(all.first().i,all.first().rdt)} else {(0,none)}
#let rdt_48 = range(48).map(i => {
  if all_0dt == none {(none,i)} else {
    let start_dt = all_0dt - duration(minutes:(all_0i*30))
    (start_dt + duration(minutes:i*30),i)
  }
})
// membrane
#let mem_d = all.filter(vs => vs.lr_mem_name != none)
#let mem = apply_48_nx_at(mem_d,"lr_mem_name")
// liqour
#let liq_d = all.filter(vs => vs.lr_af != none)
#let liq = apply_48_at(liq_d,"lr_af")
// moulding
#let mou_d = all.filter(vs => vs.lr_moulding_name != none)
#let mou = apply_48_at(mou_d,"lr_moulding_name")
// time
#let tim = range(49).map(i => {
  let kp = all.find(vs => vs.i == i)
  if kp != none {kp.adt.display("[hour]:[minute]")} else {" "}
}).slice(1).chunks(2).map(c => table.cell(align:right,inset:(x:0pt,y:12pt),
  rotate(270deg,text(14pt,baseline:3pt,top-edge:1pt,c.join(linebreak())))
))
// contraction
#let last_int = 0
#let last_dur = 0
#let ctd = ()
#for i in range(48) {
  let item = all.filter(vs => vs.lr_int != none and vs.lr_dur != none).find(vs => vs.i == i)
  ctd.push(if item == none {
    if ((end_lt > 0 and i <= end_lt) or (vs_a.len() > 0 and (i >= start_at and i <= end_at))) {
      (i,last_int,last_dur)
    } else {(i,0,0)}
  } else {
    last_int = int_num(item.lr_int)
    last_dur = if item.lr_dur == none {0} else {item.lr_dur}
    (i,last_int,last_dur)
  })
}
// oxytocin u/l
#let oxu_d = all.filter(vs => vs.lr_oxytocin_unit != none)
#let oxu = apply_48_at(oxu_d,"lr_oxytocin_unit")
// oxytocin rate
#let oxr_d = all.filter(vs => vs.lr_oxytocin_rate != none)
#let oxr = apply_48_at(oxr_d,"lr_oxytocin_rate")
// drugs
#let drg = range(48).map(i => {
  let kp = all.find(vs => vs.i == i)
  if kp != none [#kp.had_name #kp.had_drop] else {none}
}).chunks(2).map(c => table.cell(colspan:2,align:center,inset:(y:40pt),
  rotate(270deg,box(width:100pt,height:20pt,text(10pt,baseline:8pt,top-edge:1pt,[
    #let val = c.filter(x => x != none)
    #if val.len() > 0 {val.last()} else {""}
  ])))
))
// temp
#let bt_d = all.filter(vs => vs.bt != none)
#let bt = apply_48_to_24(bt_d,"bt")
// urine protein
#let upt_d = all.filter(vs => vs.urine_protein_name != none)
#let upt = apply_48_to_24(upt_d,"urine_protein_name")
// urine sugar
#let usg_d = all.filter(vs => vs.urine_sugar_name != none)
#let usg = apply_48_to_24(usg_d,"urine_sugar_name")
// urine volume
#let uvl_d = all.filter(vs => vs.lr_urine_vol != none)
#let uvl = apply_48_to_24(uvl_d,"lr_urine_vol")
// Start render
#set text(font: "TH Sarabun New", size: 14pt)
#set table(stroke:0.5pt)
#set page(
  paper: "a4",
  margin: (x:0.7cm,top:30pt,bottom:0.5cm),
  header-ascent: 8pt,
  header: [#h(1fr) #text(size:22pt,weight:700)[*Partograph*] #h(1fr)],
)
// patient info
#table(columns:(6fr,3fr,2fr,3fr,3fr,3fr),
    table.cell(colspan:2,stroke:(x:none,top:none),label_note([Name],[#pt.pname #pt.fname #pt.lname])),
    table.cell(stroke:(x:none,top:none),label_note([Gravida],[#pt.g])),
    table.cell(stroke:(x:none,top:none),label_note([Para.],[#pt.p])),
    table.cell(stroke:(x:none,top:none),label_note([HN],[#pt.hn])),
    table.cell(stroke:(x:none,top:none),label_note([AN],[#pt.an])),
    table.cell(stroke:(x:none),label_note([Date of admission],date_th(pt.regdate))),
    table.cell(colspan:2,stroke:(x:none),label_note([Time of admission],time_th(pt.regtime))),
    table.cell(colspan:2,stroke:(x:none),label_note([Ruptured membranes],[#pt.mem_ruptured_hours])),table.cell(stroke:(x:none),[*hours*]))
#v(-15pt)
#grid(columns:(45pt,15pt,10pt,7pt,1fr),
  // FHS + membrane
  grid.cell(align:right+horizon,txtb(3pt,[Fetal\ heart\ rate])),
  grid.cell(colspan:2,align:right+horizon,range(180,90,step:-10).map(i => txtb(3pt,[#i])).join(linebreak())),[],
  table(columns:((10pt,)*48).flatten(),
    table.cell(stroke:(x:none,top:none),colspan:48,[]),..(([],)*48*8),..mem),
  // Liqour + Moulding
  grid.cell(colspan:3,align:right,txtb(7pt,[Liquor#linebreak()Moulding])),[],
  table(columns:((10pt,)*48).flatten(),..liq,..mou),
  // space
  grid.cell(colspan:4,[]),
  table(columns:((10pt,)*48).flatten(),
    table.cell(stroke:(x:none,top:none),colspan:48,[])), // may be position here
  // Cx + Station
  grid.cell(colspan:2,align:left,[#v(38pt)#txtb(3pt,[Cervix\ (cm)\ \[plot X\]])#v(40pt)]),
  grid.cell(rowspan:2,align:right+horizon,range(10,-1,step:-1).map(i => txtb(3pt,[#v(-10.6pt)#i#v(-10.6pt)])).join(linebreak())),
  grid.cell(rowspan:2,[]),
  grid.cell(rowspan:3,table(columns:((20pt,)*24).flatten(),
    ..((v(10pt),)*24*9),
    ..range(1,25).map(i => table.cell(inset:(x:1pt,y:2pt),align:right+bottom,[#v(9.4pt)#i])),
    ..tim,
  )),
  grid.cell(colspan:2,align:right,[#v(39pt)#txtb(3pt,[Descent\ of head\ \[plot O\]])#v(40pt)]),
  grid.cell(colspan:2,align:right+horizon,txtb(3pt,[Time])),
  grid.cell(colspan:2,inset:(y:12pt),align(bottom+center,rotate(270deg,[#start_l]))),
  // space
  grid.cell(colspan:5,v(10pt)),
  // Contraction
  grid.cell(colspan:2,align:right+horizon,txtb(3pt,[Contractions\ per\ 10 mins])),
  grid.cell(align:right+horizon,range(5,0,step:-1).map(i => txtb(3pt,[#i])).join(linebreak())),[],
  table(columns:((10pt,)*48).flatten(),
    ..range(5,0,step:-1).map(r => {
      ctd.map(((i,c,d)) => {
        if r <= c {table.cell(fill:dura_fill(d),[])} else []
      })
    }).flatten()),
  // space
  grid.cell(colspan:5,v(10pt)),
  // Oxytocin
  grid.cell(colspan:3,align:right+horizon,txtb(3pt,[Oxytocin U/L\ drops/min])),[],
  table(columns:((10pt,)*48).flatten(),..oxu,..oxr),
  // space
  grid.cell(colspan:5,v(10pt)),
  // Mother
  grid.cell(colspan:3,align:right+horizon,txtb(3pt,[#v(35pt)Drugs\ given\ and\ IV fluids#v(34pt)])),
  grid.cell(rowspan:2,[]),
  grid.cell(rowspan:2,table(columns:((10pt,)*48).flatten(),..drg,..(([],)*48*12))),
  grid.cell(align:right+horizon,txtb(3pt,[Pulse#h(5pt)#v(30pt)BP#h(5pt)#v(30pt)])),
  grid.cell(colspan:2,align:right+horizon,range(180,50,step:-10).map(i => txtb(1pt,[#i])).join(linebreak())),
  // space
  grid.cell(colspan:5,v(5pt)),
  // Temp
  grid.cell(colspan:3,align:right+horizon,txtb(3pt,[Temp \u{00b0}C])),[],
  table(columns:((20pt,)*24).flatten(),..bt),
  // space
  grid.cell(colspan:5,v(5pt)),
  // Urine
  grid.cell(colspan:3,align:right+horizon,txtb(3pt,[protein#v(-3pt)Urine#h(25pt)sugar#v(-3pt)volume])),[],
  table(columns:((20pt,)*24).flatten(),..upt,..usg,..uvl),
)
#let line_bar(l,d) = tiptoe.line(stroke:0.5pt,toe:tiptoe.bar.with(width:10pt),angle:(d),length:l)
#let arr_head = tiptoe.combine(tiptoe.bar.with(width:10pt),tiptoe.triangle)
#let arrow_bar(l) = tiptoe.line(stroke:blue+0.5pt,tip:arr_head,toe:arr_head,angle:(90deg),length:l)
// draw X label
#place(top,dx:12pt,dy:166pt,line_bar(30pt,90deg))
#place(top,dx:12pt,dy:366pt,line_bar(130pt,270deg))
// draw O label
#place(top,dx:42pt,dy:266pt,line_bar(30pt,90deg))
#place(top,dx:42pt,dy:366pt,line_bar(30pt,270deg))
// draw BP label
#place(top,dx:42pt,dy:619pt,text(red,size:36pt,[\u{2022}]))
#place(top,dx:45pt,dy:650pt,arrow_bar(55pt))
// draw urine label
#place(top,dx:33pt,dy:762pt,curve(stroke:0.5pt,curve.cubic((-7pt,1pt),(5pt,12pt),(-5pt,15pt))))
#place(top,dx:28pt,dy:777pt,curve(stroke:0.5pt,curve.cubic((10pt,3pt),(-3pt,15pt),(5pt,15pt))))
// draw Cx line
#place(top,dx:77pt,dy:305pt,line(length:160pt))
#place(top,dx:237pt,dy:305pt,line(length:61pt,angle:90deg))
#place(top,dx:237pt,dy:305pt,line(length:198pt,angle:315deg))
#place(top,dx:317pt,dy:305pt,line(length:198pt,angle:315deg))
// draw text
#place(top,dx:78pt,dy:295pt,block(fill:white,inset:1pt,[Latent Phase]))
#place(top,dx:240pt,dy:175pt,block(fill:white,inset:1pt,[Active Phase]))
// #place(top,dx:310pt,dy:206pt,rotate(315deg,block(fill:white,inset:1pt,[Alert])))
// #place(top,dx:387pt,dy:206pt,rotate(315deg,block(fill:white,inset:1pt,[Action])))
#place(top,dx:355pt,dy:156pt,block(fill:white,inset:1pt,[Alert Line]))
#place(top,dx:430pt,dy:156pt,block(fill:white,inset:1pt,[Action Line]))
// plot FSH
#let fsh_l = vs_l.filter(vs => vs.lr_fsh != none).map(vs => (vs.i,vs.lr_fsh))
#let fsh_a = vs_a.filter(vs => vs.lr_fsh != none).map(vs => (vs.i,vs.lr_fsh))
#place(top,dx:75pt,dy:dy(43,180,1,fsh_l),plot_dot(16.94,2.83,48,100,180,red,fsh_l))
#place(top,dx:(75+start_at*10)*1pt,dy:dy(43,180,1,fsh_a),plot_dot(16.94,2.83,48,100,180,red,fsh_a))
// plot Descend
#let st_l = vs_l.filter(vs => vs.lr_sta != none).map(vs => (vs.i,6 - vs.lr_sta))
#let st_a = vs_a.filter(vs => vs.lr_sta != none).map(vs => (vs.i,6 - vs.lr_sta))
#place(top,dx:73pt,dy:dy(261,5,20,st_l),plot_o(16.94,7.06,48,0,10,purple,st_l))
#place(top,dx:(73+start_at*10)*1pt,dy:dy(261,5,20,st_a),plot_o(16.94,7.06,48,0,10,purple,st_a))
// plot Cx
#let cx_l = vs_l.filter(vs => vs.lr_cer != none).map(vs => (vs.i,to_int(vs.lr_cer)))
#let cx_a = vs_a.filter(vs => vs.lr_cer != none).map(vs => (vs.i,to_int(vs.lr_cer)))
#place(top,dx:73pt,dy:dy(161,10,20,cx_l),plot_x(16.94,7.06,48,0,10,blue,cx_l))
#place(top,dx:(73+start_at*10)*1pt,dy:dy(161,10,20,cx_a),plot_x(16.94,7.06,48,0,10,blue,cx_a))
// plot TR
#if start_at > end_lt {
  let tr_len = (start_at - end_lt)*10
  place(top,dx:(77+end_lt*10)*1pt,dy:(365 - act_cx*20)*1pt,curve(
    stroke:(paint:blue,thickness:2pt,dash:"dashed"),
    curve.quad((tr_len/2*1pt,-50pt),(tr_len*1pt,0pt)),
  ))
  place(top,dx:(70+end_lt*10+tr_len/2)*1pt,dy:(328 - act_cx*20)*1pt,block(fill:white,inset:1pt,text(weight:700,blue,[TR])))
}
// plot BP
#let bps = all.filter(vs => vs.sbp != none and vs.dbp != none).map(vs => (vs.i,vs.sbp,vs.dbp))
#for (i,s,d) in bps {
  place(top,dx:(77+(i*10))*1pt,dy:(785-s)*1pt,tiptoe.line(stroke:blue,tip:arr_head,toe:arr_head,angle:(90deg),length:(s - d)*1pt))} 
// plot HR
#let hr_l = vs_l.filter(vs => vs.pr != none).map(vs => (vs.i,vs.pr))
#let hr_a = vs_a.filter(vs => vs.pr != none).map(vs => (vs.i,vs.pr))
#place(top,dx:75pt,dy:dy(603,180,1,hr_l),plot_dot(16.94,4.24,48,60,180,red,hr_l))
#place(top,dx:(75+start_at*10)*1pt,dy:dy(603,180,1,hr_a),plot_dot(16.94,4.24,48,60,180,red,hr_a))
// table
#let vs_rows = vs_d.filter(vs => {
  vs.bt != none or vs.pr != none or vs.rr != none or vs.sbp != none or vs.dbp != none
}).sorted(key:vs => parse_dt(vs.vs_datetime)).map(vs => tbl_row(vs)).flatten()
#set page(paper:"a4",margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[บันทึกการคลอด])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an)])
#table(columns:(100pt,25pt,20pt,20pt,40pt,25pt,25pt,20pt,30pt,20pt,20pt,25pt,25pt,20pt,1fr),stroke:.5pt,
  table.header(
    table_h[วันที่ เวลา],table_h[BT],table_h[PR],table_h[RR],table_h[BP],table_h[Pos],table_h[FHS],
    table_h[Cx],table_h[Int],table_h[Dur],table_h[Eff],table_h[Sta],table_h[Mem],table_h[AF],table_h[Remark],
  ),..vs_rows)