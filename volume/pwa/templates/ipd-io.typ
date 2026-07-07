#import "templates/utils.typ": api, get_patient_main, month_th, date_th, time_th, parse_d, thousands, square_bracket_to_span
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let io_data = data.at("io",default: none)
#if io_data == none {io_data = json(api + "ipd/io?an=" + data.id)}
// PREPARED FUNCTIONS
#let dt_date_th(dt) = if dt == none {none} else {
  let (date,time) = dt.split()
  let (y,m,d) = date.split("-")
  str(int(d)) + " " + month_th(m) + str(int(y) + 543)
}
#let shift_th(s) = {
  ((s == "Night","เวรดึก"),
   (s == "Day","เวรเช้า"),
   (s == "Evening","เวรบ่าย"),
   (true, "??")).find(t => t.at(0)).at(1)
}
#let parenteral(s) = {
  ((s == "iv","IV"),
   (s == "medication","Med"),
   (s == "blood_component","Blood"),
   (s == "other","Other"),
   (true, "??")).find(t => t.at(0)).at(1)
}
#let output(s) = {
  ((s == "urine","Urine"),
   (s == "vomit","Vomit"),
   (s == "gastric_content","Gastric"),
   (s == "drain_tube","Drain"),
   (s == "dyalysis","Dialysis"),
   (s == "other","Other"),
   (true, "??")).find(t => t.at(0)).at(1)
}
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let sum_at(ios,at) = ios.map(io => io.at(at)).sum()
#let cell(col,row,c) = table.cell(colspan:col,rowspan:row,align:center+horizon,c)
#let cell_num(col,row,c) = table.cell(colspan:col,rowspan:row,align:center+horizon,thousands(c))
#let item(io) = (cell(1,1,date_th(io.io_date)),cell(1,1,time_th(io.io_time)),
  cell(1,1,parenteral(io.io_parenteral_type)),cell(1,1,square_bracket_to_span(io.io_parenteral_name)),cell_num(1,1,io.io_parenteral_amount),cell_num(1,1,io.io_parenteral_absorb),cell_num(1,1,io.io_parenteral_carry_forward),cell(1,1,io.io_parenteral_remark),
  cell(1,1,io.io_oral_name),cell_num(1,1,io.io_oral_amount),cell_num(1,1,io.io_oral_absorb),cell_num(1,1,io.io_oral_carry_forward),cell(1,1,io.io_oral_remark),
  cell(1,1,output(io.io_output_type)),cell_num(1,1,io.io_output_amount),cell(1,1,io.io_output_remark),cell(1,1,io.user_name)
)
#let shift_rows(ios,s) = {
  let io_shift = ios.filter(io => io.shift == s)
  if io_shift.len() == 0 {()} else {(
    ..io_shift.map(io => item(io)).flatten(),
    table.cell(colspan:2,strong[รวม #shift_th(s)]),cell(6,1,strong[#thousands(sum_at(io_shift,"io_parenteral_absorb")) c.c.]),cell(5,1,strong[#thousands(sum_at(io_shift,"io_oral_absorb")) c.c.]),cell(3,1,strong[#thousands(sum_at(io_shift,"io_output_amount")) c.c.]),[]
  )}
}
#let date_rows(ios,d) = {
  let io_oneday = ios.filter(io => io.shift_date == d)
  if io_oneday.len() == 0 {()} else {(
  ..shift_rows(io_oneday,"Night"),..shift_rows(io_oneday,"Day"),..shift_rows(io_oneday,"Evening"),
    table.cell(colspan:2,strong[รวม 24 ชั่วโมง]),cell(6,1,strong[#thousands(sum_at(io_oneday,"io_parenteral_absorb")) c.c.]),cell(5,1,strong[#thousands(sum_at(io_oneday,"io_oral_absorb")) c.c.]),cell(3,1,strong[#thousands(sum_at(io_oneday,"io_output_amount")) c.c.]),[],
    table.cell(colspan:2,strong[รวม #dt_date_th(d.display())]),cell(11,1,strong[#thousands(sum_at(io_oneday,"io_parenteral_absorb")+sum_at(io_oneday,"io_oral_absorb")) c.c.]),cell(3,1,strong[#sum_at(io_oneday,"io_output_amount") c.c.]),[]
  )}
}
// PREPARED VARIABLES
#let io_data = io_data.map(io => {
  io.shift_date = parse_d(io.shift_date)
  io.io_oral_amount = if io.io_oral_amount == none {.0} else {float(io.io_oral_amount)}
  io.io_oral_absorb = if io.io_oral_absorb == none {.0} else {float(io.io_oral_absorb)}
  io.io_oral_carry_forward = if io.io_oral_carry_forward == none {.0} else {float(io.io_oral_carry_forward)}
  io.io_parenteral_amount = if io.io_parenteral_amount == none {.0} else {float(io.io_parenteral_amount)}
  io.io_parenteral_absorb = if io.io_parenteral_absorb == none {.0} else {float(io.io_parenteral_absorb)}
  io.io_parenteral_carry_forward = if io.io_parenteral_carry_forward == none {.0} else {float(io.io_parenteral_carry_forward)}
  io.io_output_amount = if io.io_output_amount == none {.0} else {float(io.io_output_amount)}
  io
})
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[ใบ Record Intake - Output])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an)],
)
#let dates = io_data.map(io => io.shift_date).dedup()
#table(columns:(60pt,40pt,40pt,80pt,35pt,35pt,35pt,50pt,60pt,35pt,35pt,35pt,50pt,40pt,35pt,50pt,1fr),stroke:.5pt,
  table.header(
    cell(1,2,strong[วันที่]),cell(1,2,strong[เวลา]),cell(6,1,strong[Parenteral fluid]),cell(5,1,strong[Oral fluid]),cell(3,1,strong[Output]),cell(1,2,strong[ผู้บันทึก]),
    cell(1,1,strong[ประเภท]),cell(1,1,strong[ชื่อสารน้ำ/เลือด]),cell(1,1,strong[ปริมาณ]),cell(1,1,strong[ได้รับ]),cell(1,1,strong[ยกไป]),cell(1,1,strong[หมายเหตุ]),
    cell(1,1,strong[ชื่อของเหลว]),cell(1,1,strong[ปริมาณ]),cell(1,1,strong[ได้รับ]),cell(1,1,strong[ยกไป]),cell(1,1,strong[หมายเหตุ]),
    cell(1,1,strong[ประเภท]),cell(1,1,strong[ปริมาณ]),cell(1,1,strong[หมายเหตุ])
  ),..dates.map(d => date_rows(io_data,d)).flatten()
)