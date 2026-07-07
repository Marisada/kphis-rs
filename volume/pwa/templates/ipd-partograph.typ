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
#let plot_tri(x,y,col,ymin,ymax,color,data) = if data.len() > 0 {
  cetz.canvas(plot.plot(
    size:(x,y),
    axis-style:none,
    plot-style:(stroke:(1pt+color)),
    mark-style:(stroke:2pt+color,fill:white),
    x-min:0,x-max:col,
    y-min:ymin,y-max:ymax,{
      plot.add(data,mark:"triangle",mark-size:8pt)
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
  (if need_tr {l.push(a.first());l} else {l},a)
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
  if kp == none {table.cell(stroke:(x:none,top:none),[])} else {table.cell(stroke:(x:none,top:none),align:center,inset:(top:3pt,bottom:0pt),text(10pt,kp.at(at)))}
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
  if all_0dt == none or all_0i == none {(none,i)} else {
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
// Start render
#set text(font: "TH Sarabun New", size: 14pt)
#set table(stroke:0.5pt)
#set page(paper:"a4",margin:(x:0.6cm,top:40pt,bottom:40pt),
  header-ascent:8pt,
  header:context[#h(1fr) #text(size:20pt,weight:700,[บันทึกการคลอด])#h(1fr)#counter(page).display("1/1",both:true)],
  footer:[#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an)])
// patient info
#table(columns:(3fr,3fr,3fr,2fr,3fr,3fr,3fr),
    table.cell(stroke:(x:none,top:none),label_note([วันที่],date_th(pt.regdate))),
    table.cell(stroke:(x:none,top:none),label_note([เวลา],time_th(pt.regtime))),
    table.cell(stroke:(x:none,top:none),[#label_note([G],[#pt.g]) #label_note([P],[#pt.p])]),
    table.cell(stroke:(x:none,top:none),label_note([Last],[#pt.last_child ปี])),
    table.cell(stroke:(x:none,top:none),label_note([GA],[#pt.gestational_age+#pt.gestational_day สัปดาห์])),
    table.cell(stroke:(x:none,top:none),label_note([LMP],date_th(pt.lmp))),
    table.cell(stroke:(x:none,top:none),label_note([EDC],date_th(pt.edc))),
    table.cell(colspan:5,stroke:(x:none,top:none),label_note([อาการสำคัญ],[#pt.chief_complaints])),
    table.cell(colspan:2,stroke:(x:none),label_note([Ruptured membranes],if pt.mem_ruptured_hours != none [#pt.mem_ruptured_hours *hours*] else [ไม่มี])))
#v(-15pt)
#grid(columns:(30pt,15pt,5pt,1fr,30pt),
  // Membrane + Liqour + Moulding
  grid.cell(colspan:2,align:right,txtb(7pt,[#v(10pt)Liquor#linebreak()Moulding])),[],
  table(columns:((10pt,)*48).flatten(),..mem,..liq,..mou),[],
  // space
  grid.cell(align:right+horizon,[St]),grid.cell(align:right+horizon,[Cx]),
  grid.cell(colspan:3,v(20pt)),
  // Cx + Station
  grid.cell(align:right+horizon,
    ("","-4","-3","-2","-1","0","+1","+2","+3","+4","").map(i => txtb(12pt,[#v(-10.6pt)#i#v(-10.6pt)])).join(linebreak())),
  grid.cell(align:right+horizon,range(10,-1,step:-1).map(i => txtb(12pt,[#v(-10.6pt)#i#v(-10.6pt)])).join(linebreak())),[],
  grid.cell(rowspan:2,table(columns:((20pt,)*24).flatten(),
    ..((v(10pt),)*24*9),
    ..range(1,25).map(i => table.cell(inset:(x:1pt,y:2pt),align:right+bottom,[#v(9.4pt)#i])),
    ..tim, // Time
  )),
  grid.cell(align:left+horizon,inset:(left:2pt),range(190,80,step:-10).map(i => txtb(12pt,[#v(-10.6pt)#i#v(-10.6pt)])).join(linebreak())),
  grid.cell(align:right+horizon,txtb(12pt,[Time])),
  grid.cell(colspan:2,inset:(y:12pt),align(bottom+center,rotate(270deg,[#start_l]))),
  grid.cell(inset:2pt,align(horizon,[#v(20pt)FHS#v(-12pt)\[\u{2206}\]])),
)
#let line_bar(l,d) = tiptoe.line(stroke:0.5pt,toe:tiptoe.bar.with(width:10pt),angle:(d),length:l)
#let arr_head = tiptoe.combine(tiptoe.bar.with(width:10pt),tiptoe.triangle)
#let arrow_bar(l) = tiptoe.line(stroke:blue+0.5pt,tip:arr_head,toe:arr_head,angle:(90deg),length:l)
// draw line
#place(top,dx:50pt,dy:225pt,line(length:160pt))
#place(top,dx:210pt,dy:225pt,line(length:60pt,angle:90deg))
#place(top,dx:210pt,dy:225pt,line(length:198pt,angle:315deg))
#place(top,dx:290pt,dy:225pt,line(length:198pt,angle:315deg))
// draw text
#place(top,dx:51pt,dy:215pt,block(fill:white,inset:1pt,[Latent Phase]))
#place(top,dx:213pt,dy:75pt,block(fill:white,inset:1pt,[Active Phase]))
#place(top,dx:328pt,dy:75pt,block(fill:white,inset:1pt,[Alert Line]))
#place(top,dx:400pt,dy:75pt,block(fill:white,inset:1pt,[Action Line]))
// plot Descend
#let st_l = vs_l.filter(vs => vs.lr_sta != none).map(vs => (vs.i,6 - vs.lr_sta))
#let st_a = vs_a.filter(vs => vs.lr_sta != none).map(vs => (vs.i,6 - vs.lr_sta))
#place(top,dx:46pt,dy:dy(141,5,20,st_l),plot_o(16.94,7.06,48,0,10,purple,st_l))
#place(top,dx:(46+start_at*10)*1pt,dy:dy(141,5,20,st_a),plot_o(16.94,7.06,48,0,10,purple,st_a))
// plot FSH
#let fsh_l = vs_l.filter(vs => vs.lr_fsh != none).map(vs => (vs.i,vs.lr_fsh))
#let fsh_a = vs_a.filter(vs => vs.lr_fsh != none).map(vs => (vs.i,vs.lr_fsh))
#place(top,dx:46pt,dy:dy(81,190,2,fsh_l),plot_tri(16.94,7.06,48,90,190,red,fsh_l))
#place(top,dx:(46+start_at*10)*1pt,dy:dy(81,190,2,fsh_a),plot_tri(16.94,7.06,48,90,190,red,fsh_a))
// plot Cx
#let cx_l = vs_l.filter(vs => vs.lr_cer != none).map(vs => (vs.i,to_int(vs.lr_cer)))
#let cx_a = vs_a.filter(vs => vs.lr_cer != none).map(vs => (vs.i,to_int(vs.lr_cer)))
#place(top,dx:46pt,dy:dy(81,10,20,cx_l),plot_x(16.94,7.06,48,0,10,blue,cx_l))
#place(top,dx:(46+start_at*10)*1pt,dy:dy(81,10,20,cx_a),plot_x(16.94,7.06,48,0,10,blue,cx_a))
// plot TR
#if start_at > end_lt {
  let tr_len = (start_at - end_lt)*10
  place(top,dx:(50+end_lt*10)*1pt,dy:(285 - act_cx*20)*1pt,curve(
    stroke:(paint:blue,thickness:2pt,dash:"dashed"),
    curve.quad((tr_len/2*1pt,-50pt),(tr_len*1pt,0pt)),
  ))
  place(top,dx:(43+end_lt*10+tr_len/2)*1pt,dy:(248 - act_cx*20)*1pt,block(fill:white,inset:1pt,text(weight:700,blue,[TR])))
}
// table
#let vs_rows = vs_d.filter(vs => {
  vs.bt != none or vs.pr != none or vs.rr != none or vs.sbp != none or vs.dbp != none
}).sorted(key:vs => parse_dt(vs.vs_datetime)).map(vs => tbl_row(vs)).flatten()
#table(columns:(100pt,25pt,20pt,20pt,40pt,25pt,25pt,20pt,30pt,20pt,20pt,25pt,25pt,20pt,1fr),stroke:.5pt,
  table.header(
    table_h[วันที่ เวลา],table_h[BT],table_h[PR],table_h[RR],table_h[BP],table_h[Pos],table_h[FHS],table_h[Cx],table_h[Int],table_h[Dur],table_h[Eff],table_h[Sta],table_h[Mem],table_h[AF],table_h[Remark],
  ),..vs_rows)