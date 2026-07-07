#import "templates/utils.typ": api, get_patient_main, date_th, time_th, datetime_th, parse_dt
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let vs_data = data.at("vs",default: none)
#if vs_data == none {vs_data = json(api + "ipd/vital-sign?an=" + data.id)}
// PREPARED FUNCTIONS
#let pupil(i) = {
  ((i == 1,"RTL"),
   (i == 2,"sRTL"),
   (i == 3,"Fixed"),
   (i == 4,"Dilated"),
   (true, "")).find(t => t.at(0)).at(1)
}
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let table_h(c) = [#align(center,strong(c))]
#let cell(c) = table.cell(align:center+horizon,[#c])
#let item(vs) = (cell(datetime_th(vs.vs_datetime)),
  cell(vs.bt),cell(vs.pr),cell(vs.rr),cell[#vs.sbp/#vs.dbp],cell(vs.map),cell(vs.sat),
  cell(if vs.eye != none [E#vs.eye\V#vs.verbal\M#vs.movement]),
  cell(if vs.right_pupil != none [#vs.right_pupil #pupil(vs.right_cha_id)]),
  cell(if vs.left_pupil != none [#vs.left_pupil #pupil(vs.left_cha_id)]),
  cell(vs.rt_arm_name),cell(vs.lt_arm_name),cell(vs.rt_leg_name),cell(vs.lt_leg_name),
  cell(vs.barthel_index),
  table.cell(align:horizon,text(size:10pt,if vs.had_name != none and vs.had_name.trim() != "" [#vs.had_name rate #vs.had_drop])),
  table.cell(align:horizon,text(size:10pt,if vs.update_opduser_name == none {vs.create_opduser_name} else {vs.update_opduser_name}))
)
// PREPARED VARIABLES
#let vs_data = vs_data.filter(vs => {
  vs.vs_datetime != none and (vs.bt != none or vs.pr != none or vs.rr != none or vs.sbp != none or vs.dbp != none or vs.sat != none)
}).sorted(key:vs => parse_dt(vs.vs_datetime))
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[ใบบันทึกสัญญาณชีพ])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([VN : ],pt.vn)],
)
#let has_o2 = vs_data.any(vs => vs.o2_name != none)
#let has_o2ra = vs_data.any(vs => vs.sat_room_air != none)
#let cols = (90pt,22pt,22pt,18pt,35pt,22pt,24pt,35pt,45pt,45pt,15pt,15pt,15pt,15pt,22pt,1fr,120pt)
#let headers = (table_h[วันที่ เวลา],table_h[BT],table_h[PR],table_h[RR],table_h[BP],table_h[MAP],table_h[O#sub("2")sat],table_h[GCS],table_h[Rt pupil],table_h[Lt pupil],table_h[RA],table_h[LA],table_h[RL],table_h[LL],table_h[ADL],table_h[Medication],table_h[ผู้บันทึก])
#if has_o2 {
  cols.insert(7,80pt)
  headers.insert(7,table_h[O#sub("2")])
}
#if has_o2ra {
  cols.insert(6,24pt)
  headers.insert(6,table_h[O#sub("2")RA])
}
#table(columns:cols,stroke:.5pt, table.header(..headers), ..vs_data.map(vs => {
  let items = item(vs)
  if has_o2 {items.insert(7,table.cell(align:center+horizon,
    if vs.o2_name == none [Room Air] 
    else if vs.o2_name == "Tube" [#vs.tube_name#if vs.tube_no != none [ #calc.round(decimal(vs.tube_no))]#if vs.tube_mark != none [ d#calc.round(decimal(vs.tube_mark))]]
    else [#vs.o2_name#if vs.o2_flow != none [ #calc.round(decimal(vs.o2_flow))LPM]#if vs.fio2 != none [ #calc.round(decimal(vs.fio2),digits:1)]]))}
  if has_o2ra {items.insert(6,cell(vs.sat_room_air))}
  items
}).flatten())