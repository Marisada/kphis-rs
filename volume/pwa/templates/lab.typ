#import "customs/config.typ": hospital-name
#import "templates/utils.typ": api, vnan_is_ipd, get_patient_main, date_th, time_th, base64_to_byte, is_lab_ab
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let is_ipd = vnan_is_ipd(data.id)
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let labs = data.at("lab", default: none)
#if labs == none {labs = json(api + "lab/head?with_scan=true&vn=" + data.id)}
#let logo = if labs.len() > 0 [#image("/statics/picture/MOPH.svg",width:40pt)]
// PREPARED FUNCTIONS
#let label_note(l,n) = [#text(weight:700,l) #n]
#let gline() = grid.hline(start:0,stroke:.5pt)
#let render_gr(gr) = (grid.cell(colspan:5,strong[#gr.lab_items_group_name]),..gr.lab_items.filter(item => item.lab_order_result != none and item.lab_order_result != "").map(item => (
  [],[#item.lab_items_name_ref],
  [#if item.lab_order_result != none and item.lab_items_normal_value_ref != "-" {
    let ab_res = is_lab_ab(item.lab_order_result, item.lab_items_normal_value_ref)
    if ab_res == "T" {text(red,weight:700,item.lab_order_result)}
    else if ab_res == "H" {text(red,weight:700,[#item.lab_order_result#h(3pt)H])}
    else if ab_res == "L" {text(red,weight:700,[#item.lab_order_result#h(3pt)L])}
    else {item.lab_order_result}
  } else {item.lab_order_result}],
  [#item.lab_items_unit],[#item.lab_items_normal_value_ref]
)).flatten())
#let render(lab) = [#rect(stroke:.5pt,[
  #grid(columns: (50pt,1fr,50pt),align(center,logo),align(center+horizon,[#strong[#hospital-name]#linebreak()#strong[#lab.form_name]#linebreak()กลุ่มงานเทคนิคการแพทย์]),[])#v(-8pt)
  #label_note([NAME : ],[#pt.pname #pt.fname #pt.lname]) #label_note([SEX : ],pt.sex_name) #label_note([AGE : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([#if is_ipd [AN] else [VN] : ],data.id) #label_note([จุดที่สั่ง : ],lab.department) #if lab.specimen_name_cc != none [#label_note([SPECIMEN : ],lab.specimen_name_cc)]#v(-8pt)
  #grid(columns:(30pt,1fr,1fr,1fr,1fr),inset:3pt,
    gline(),
    strong[TEST],strong[NAME],strong[RESULT],strong[UNIT],strong[NORMAL RANGES],
    gline(),
    ..lab.lab_items_group.map(gr => render_gr(gr)).flatten(),
    gline(),
  )#v(-8pt)
  #grid(columns:(170pt,80pt,1fr),par(leading:5pt,[
    #label_note([Ordered Date : ],[#date_th(lab.order_date) #time_th(lab.order_time)])\
    #label_note([Received Date : ],[#date_th(lab.receive_date) #time_th(lab.receive_time)])\
    #label_note([Reported Date : ],[#date_th(lab.report_date) #time_th(lab.report_time)])
  ]),par(leading:5pt,[L = Low#linebreak()H = High#linebreak()R = Repeated]),[
    #label_note([Reported by : ],lab.reporter_name)\
    #label_note([Approved by : ],lab.approve_staff)
  ])
  #align(center,[\*THE ABOVE RESULT REFLECTS THE ANALYSIS ONLY OF THE SAMPLE, IN ITS CONDITION RECEIVED\*])
])
#for img in lab.scan_images {
  image(base64_to_byte(img.image.image),width:100%)
}]
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",margin:1cm)
#if labs.len() > 0 {
  for lab in labs {render(lab)}
}