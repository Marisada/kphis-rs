#import "templates/utils.typ": api, get_patient_main, date_th, time_th, datetime_th, parse_dt
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let vs_data = data.at("vs",default: none)
#if vs_data == none {vs_data = json(api + "ipd/vital-sign?an=" + data.id)}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let table_h(c) = [#align(center,strong(c))]
#let cell(c) = table.cell(align:center+horizon,[#c])
#let item(vs) = (cell(datetime_th(vs.vs_datetime)),
  cell(vs.bt),cell(vs.pr),cell(vs.rr),cell[#vs.sbp/#vs.dbp],cell(vs.map),cell(vs.sat),
  cell(if vs.amphetamine_awq == none {none} else {vs.amphetamine_awq.split(",").first()}),
  cell(if vs.amphetamine_awq == none {none} else {vs.amphetamine_awq.split(",").at(1,default:none)}),
  cell(if vs.amphetamine_awq == none {none} else {vs.amphetamine_awq.split(",").at(2,default:none)}),
  cell(if vs.amphetamine_awq == none {none} else {vs.amphetamine_awq.split(",").at(3,default:none)}),
  cell(if vs.aggression_oas == none {none} else {vs.aggression_oas.split(",").at(3,default:none)}),
  cell(vs.motivation_scale),cell(vs.craving_scale),cell(vs.stage_of_change_name),
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
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an)],
)
#let awq = vs_data.filter(vs => vs.amphetamine_awq != none)
#let aws = vs_data.filter(vs => vs.alcohol_aws != none)
#let ciwa = vs_data.filter(vs => vs.alcohol_ciwa != none)
#let has_o2 = vs_data.any(vs => vs.o2_name != none)
#let has_o2ra = vs_data.any(vs => vs.sat_room_air != none)
#let cols = (90pt,22pt,22pt,18pt,35pt,22pt,24pt,25pt,18pt,18pt,18pt,22pt,18pt,22pt,78pt,1fr,120pt)
#let headers = (table_h[วันที่ เวลา],table_h[BT],table_h[PR],table_h[RR],table_h[BP],table_h[MAP],table_h[O#sub("2")sat],table_h[AWQ],table_h[H],table_h[A],table_h[R],table_h[OAS],table_h[MS],table_h[CVS],table_h[Stage of Change],table_h[Medication],table_h[ผู้บันทึก])
#if aws.len() > 0 {
  cols.insert(12,20pt)
  headers.insert(12,table_h[AWS])
}
#if ciwa.len() > 0 {
  cols.insert(12,24pt)
  headers.insert(12,table_h[CIWA])
}
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
  if aws.len() > 0 {items.insert(12,cell(if vs.alcohol_aws == none {none} else {vs.alcohol_aws.split(",").first()}))}
  if ciwa.len() > 0 {items.insert(12,cell(if vs.alcohol_ciwa == none {none} else {vs.alcohol_ciwa.split(",").first()}))}
  if has_o2 {items.insert(7,table.cell(align:center+horizon,
    if vs.o2_name == none [Room Air] 
    else if vs.o2_name == "Tube" [#vs.tube_name#if vs.tube_no != none [ #calc.round(decimal(vs.tube_no))]#if vs.tube_mark != none [ d#calc.round(decimal(vs.tube_mark))]]
    else [#vs.o2_name#if vs.o2_flow != none [ #calc.round(decimal(vs.o2_flow))LPM]#if vs.fio2 != none [ #calc.round(decimal(vs.fio2),digits:1)]]))}
  if has_o2ra {items.insert(6,cell(vs.sat_room_air))}
  items
}).flatten())
#let awq_complete = awq.filter(vs => vs.amphetamine_awq.contains(","))
#if awq_complete.len() > 0 [#align(center,text(20pt,[AWQv2]))#v(-15pt)#table(columns:(90pt,1fr,1fr,1fr,40pt,1fr,1fr,1fr,1fr,40pt,1fr,1fr,1fr,40pt,50pt,120pt),stroke:.5pt,
  table.header(table.cell(rowspan:2,align:center+horizon,table_h[วันที่ เวลา]),table.cell(colspan:4,table_h[Hyperarousal]),table.cell(rowspan:2,align:center+horizon,table_h[รู้สึกซึมเศร้า]),table.cell(colspan:4,table_h[Anxiety]),table.cell(colspan:4,table_h[Reversed Vegetative]),table.cell(rowspan:2,align:center+horizon,table_h[รวมทั้งหมด]),table.cell(rowspan:2,align:center+horizon,table_h[ผู้ประเมิน]),
  table_h[รู้สึกอยากยา],table_h[กระวนกระวาย],cell(table_h[ฝันร้าย]),cell(table_h[รวม]),cell(table_h[รู้สึกเบื่อ]),table_h[วิตก กังวล],cell(table_h[เชื่องช้า]),cell(table_h[รวม]),cell(table_h[ไม่มีแรง]),table_h[อยากอาหาร],cell(table_h[นอนมาก]),cell(table_h[รวม])),
  ..awq_complete.map(vs => {
    let split = vs.amphetamine_awq.split(",");
    (cell(datetime_th(vs.vs_datetime)),cell(split.at(4,default:none)),cell(split.at(9,default:none)),cell(split.at(12,default:none)),cell(split.at(1,default:none)),
    cell(split.at(5,default:none)),cell(split.at(6,default:none)),cell(split.at(7,default:none)),cell(split.at(8,default:none)),cell(split.at(2,default:none)),
    cell(split.at(10,default:none)),cell(split.at(11,default:none)),cell(split.at(13,default:none)),cell(split.at(3,default:none)),
    cell(split.at(0,default:none)),
    table.cell(align:horizon,text(size:10pt,if vs.update_opduser_name == none {vs.create_opduser_name} else {vs.update_opduser_name})))
  }).flatten()
)]
#let ciwa_complete = ciwa.filter(vs => vs.alcohol_ciwa.contains(","))
#if ciwa_complete.len() > 0 [#align(center,text(20pt,[CIWA-Ar]))#v(-15pt)#table(columns:(90pt,1fr,1fr,1fr,1fr,1fr,1fr,1fr,1fr,1fr,1fr,50pt,120pt),stroke:.5pt,
  table.header(cell(table_h[วันที่ เวลา]),table_h[คลื่นไส้อาเจียน],table_h[สัมผัส#linebreak()ผิดปกติ],table_h[อาการ#linebreak()มือสั่น],table_h[รับรู้เสียง#linebreak()ผิดปกติ],table_h[เหงื่อออกเป็นพักๆ],table_h[รับรู้ทางตาผิดปกติ],table_h[อาการวิตกกังวล],table_h[ปวดหัว#linebreak()มึนตื้อ],table_h[กระวนกระวาย],table_h[ไม่รู้เวลา สถานที่],cell(table_h[รวมทั้งหมด]),cell(table_h[ผู้ประเมิน])),
  ..ciwa_complete.map(vs => {
    let split = vs.alcohol_ciwa.split(",");
    (cell(datetime_th(vs.vs_datetime)),cell(split.at(1,default:none)),cell(split.at(2,default:none)),cell(split.at(3,default:none)),cell(split.at(4,default:none)),
    cell(split.at(5,default:none)),cell(split.at(6,default:none)),cell(split.at(7,default:none)),cell(split.at(8,default:none)),
    cell(split.at(9,default:none)),cell(split.at(10,default:none)),cell(split.at(0,default:none)),
    table.cell(align:horizon,text(size:10pt,if vs.update_opduser_name == none {vs.create_opduser_name} else {vs.update_opduser_name})))
  }).flatten()
)]
#let aws_complete = aws.filter(vs => vs.alcohol_aws.contains(","))
#if aws_complete.len() > 0 [#align(center,text(20pt,[AWS]))#v(-15pt)#table(columns:(90pt,1fr,1fr,1fr,1fr,1fr,1fr,1fr,50pt,120pt),stroke:.5pt,
  table.header(cell(table_h[วันที่ เวลา]),table_h[เหงื่อออก],table_h[อาการสั่น],table_h[วิตกกังวล],table_h[กระสับกระส่าย],table_h[อุณหภูมิ],table_h[ประสาทหลอน],table_h[วัน เวลา สถานที่],cell(table_h[รวมทั้งหมด]),cell(table_h[ผู้ประเมิน])),
  ..aws_complete.map(vs => {
    let split = vs.alcohol_aws.split(",");
    (cell(datetime_th(vs.vs_datetime)),cell(split.at(1,default:none)),cell(split.at(2,default:none)),cell(split.at(3,default:none)),cell(split.at(4,default:none)),
    cell(split.at(5,default:none)),cell(split.at(6,default:none)),cell(split.at(7,default:none)),cell(split.at(0,default:none)),
    table.cell(align:horizon,text(size:10pt,if vs.update_opduser_name == none {vs.create_opduser_name} else {vs.update_opduser_name})))
  }).flatten()
)]