#import "customs/config.typ": drug_alert
#import "templates/utils.typ": api, get_patient_main, date_th, month_th, time_th, parse_d_t, parse_d, parse_dt, dt2th, note_type, square_bracket_to_span
#import "templates/vital_sign.typ"
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let order = data.at("order",default: none)
#if order == none {order = json(api + "ipd/order/order?an=" + data.id + "&view_by=doctor")}
#let note = data.at("note", default: none)
#if note == none {note = json(api + "ipd/order/progress-note?an=" + data.id)}
#let cons = data.at("consult",default: none)
#if cons == none {cons = json(api + "ipd/consult-an/" + data.id)}
#let vs = data.at("vs",default: none)
#if vs == none {vs = json(api + "ipd/vital-sign?an=" + data.id)}
#let io = data.at("io",default: none)
#if io == none {io = json(api + "ipd/io?an=" + data.id)}
#let lab = data.at("lab", default: none)
#if lab == none {lab = json(api + "lab/head?vn=" + data.id)}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:18pt,weight:700,c))
#let dt2d(dt) = if dt == none {none} else {dt.display("[year]-[month]-[day]")}
#let label_note(l,n) = [#text(weight:700,l) #n]
#let badge_inner(m,c,t) = box(radius:3pt,outset:3pt,stroke:.3pt+c,[#text(8pt,fill:c,m)#box(align(center,par(leading:0.2em,text(8pt,fill:c,t))))])
#let badge(m,c,t) = box(inset:(x:2pt,bottom:1pt),badge_inner(m,c,t))
#let alert_badge(p) = {
  let alert = drug_alert(p.displaycolor)
  ((alert == "HAD",badge(none,red,"HAD")),
   (alert == "LASA",badge(none,blue,"LASA")),
   (true, none)).find(t => t.at(0)).at(1)
}
#let parse_order(oi) = {
  let otype = if oi.order_type == "oneday" {badge(none,black,[ONE DAY])} else if oi.order_type == "continuous" {badge(none,black,[CONTINUOUS])} else []
  if oi.order_id == none [รายการที่ไม่ผูกกับ order] else [
    #otype#h(3pt)#alert_badge(oi) #if oi.order_item_type == "off" [OFF: #strong(oi.off_med_name) #oi.off_order_item_detail
    ] else [#strong(oi.med_name) #oi.order_item_detail] #if oi.stat == "Y" [\(stat\)]
  ]
}
#let parse_action(ac,parsed_oi) = [
  #badge(none,red,[ACTION])#h(1pt)#parsed_oi
  #if ac.action_result != none and ac.action_result != "" [#strong[ผลลัพธ์:] #ac.action_result]
  #if ac.action_remark != none and ac.action_remark != "" [#strong[หมายเหตุ:] #ac.action_remark]
]
#let parse_note(nt) = [
  #badge(none,black,[NOTE])
  #if nt.progress_note_item_types.len() > 0 {
    nt.progress_note_item_types.map(ts => {
      let items = if ts.progress_note_items.len() > 0 {
        ts.progress_note_items.map(item => [#item.progress_note_item_detail #if item.progress_note_item_detail_2 != none [ \/ #item.progress_note_item_detail_2]]).join()
      } else []
      text(weight:700,[#note_type(ts.progress_note_item_type): ])+items
    }).join()
  } else []
]
#let parse_consult_c(cs) = [
  #badge(none,black,[CONSULT])
  #if cs.spcltyname != none [#strong(cs.spcltyname)]
  #if cs.consult_doctorcode_mention_name != none [#cs.consult_doctorcode_mention_name]
  #if cs.consult_data != none [#strong[Detail:] #cs.consult_data]
]
#let parse_consult_r(cs) = [
  #badge(none,black,[RECOMMEND])
  #if cs.consult_finding != none [#strong[Finding:] #cs.consult_finding]
  #if cs.consult_diagnosis != none [#strong[Dx:] #cs.consult_diagnosis]
  #if cs.consult_recommendation != none [#strong[Recommend:] #cs.consult_recommendation]
]
#let parse_vs(vs,birthday) = [
  #badge(none,black,[VS]) #vital_sign.parse_vs(vs,birthday)
]
#let parse_io(i) = [
  #badge(none,black,[IO])
  #if i.io_parenteral_type != none [#strong(square_bracket_to_span(i.io_parenteral_name))]
	#if i.io_parenteral_amount != none [#strong[เพิ่ม:] #i.io_parenteral_amount ml]
	#if i.io_parenteral_absorb != none [#strong[ได้รับ:]  #i.io_parenteral_absorb ml]
	#if i.io_parenteral_carry_forward != none [#strong[ยกไป:] #i.io_parenteral_carry_forward ml]
	#if i.io_parenteral_remark != none [\(#i.io_parenteral_remark\)]
	#if i.io_oral_name != none [#strong(i.io_oral_name)]
	#if i.io_oral_amount != none [#strong[เพิ่ม:] #i.io_oral_amount ml]
	#if i.io_oral_absorb != none [#strong[ได้รับ:]  #i.io_oral_absorb ml]
	#if i.io_oral_carry_forward != none [#strong[ยกไป:] #i.io_oral_carry_forward ml]
	#if i.io_oral_remark != none [\(#i.io_oral_remark\)]
	#if i.io_output_type != none [#strong[#i.io_output_type:]]
	#if i.io_output_amount != none [#i.io_output_amount ml]
	#if i.io_output_remark != none [\(#i.io_output_remark\)]
]
#let parse_lab(lb) = [
  #badge(none,black,[LAB])
  #if lb.form_name != none [#strong(lb.form_name)]
  #for li in lb.lab_items_group.map(gr => gr.lab_items).flatten() [
    #strong[#li.lab_items_name_ref:] #li.lab_order_result #li.lab_items_unit #if li.lab_items_normal_value_ref != none and li.lab_items_normal_value_ref != "" [\[#li.lab_items_normal_value_ref\]]
  ]
]
#let parse_all(i) = ([#dt2th(i.dt)], [#i.txt], [#i.owner])
// PREPARED VARIABLES
#let order_item = order.map(od => {
  let owner = if od.nurse_order_as_name != none [#if od.nurse_order_as_is_intern == true [\(Intern\) ] #od.nurse_order_as_name] else [#if od.order_doctor_is_intern == true [\(Intern\) ] #od.order_doctor_name]
  od.order_item_types.map(ot => ot.order_items).flatten().map(oi => {
    oi.owner = owner;oi.parsed = parse_order(oi);oi
  })
}).flatten()
#let order_d = order_item.map(oi => {
  let dt = parse_d_t(oi.order_date, oi.order_time)
  (dt: dt, ds: dt2d(dt), txt: oi.parsed, owner: oi.owner)
})
#let action_d = order_item.map(oi => {
  oi.index_plans.map(pn => pn.actions).flatten().filter(ac => ac.action_date != none and ac.action_time != none).map(ac => {
    let dt = parse_d_t(ac.action_date, ac.action_time)
    (dt: dt, ds: dt2d(dt), txt: parse_action(ac,oi.parsed), owner: text[#if ac.action_person_1_name != none [#ac.action_person_1_name] #if ac.action_person_2_name != none [ \/ #ac.action_person_2_name]])
  })
}).flatten()
#let note_d = note.filter(nt => nt.progress_note_date != none and nt.progress_note_time != none).map(nt => {
  let dt = parse_d_t(nt.progress_note_date, nt.progress_note_time)
  (dt: dt, ds: dt2d(dt), txt: parse_note(nt), owner: [#if nt.order_doctor_is_intern == true [\(Intern\) ] #nt.order_doctor_name])
})
#let cons_c_d = cons.filter(cs => cs.consult_date != none and cs.consult_time != none).map(cs => {
  let dt = parse_d_t(cs.consult_date, cs.consult_time)
  (dt: dt, ds: dt2d(dt), txt: parse_consult_c(cs), owner: cs.string_consult_request_name)
})
#let cons_r_d = cons.filter(cs => cs.consult_datetime_create_reply != none).map(cs => {
  let dt = parse_dt(cs.consult_datetime_create_reply)
  (dt: dt, ds: dt2d(dt), txt: parse_consult_r(cs), owner: cs.string_consult_reply_name)
})
#let vs_d = vs.filter(v => v.vs_datetime != none).map(v => {
  let dt = parse_dt(v.vs_datetime)
  (dt: dt, ds: dt2d(dt), txt: parse_vs(v,pt.birthday), owner: v.update_opduser_name)
})
#let io_d = io.filter(i => i.io_date != none and i.io_time != none).map(i => {
  let dt = parse_d_t(i.io_date, i.io_time)
  (dt: dt, ds: dt2d(dt), txt: parse_io(i), owner: i.user_name)
})
#let lab_d = lab.filter(lb => lb.report_date != none and lb.report_time != none).map(lb => {
  let dt = parse_d_t(lb.report_date, lb.report_time)
  (dt: dt, ds: dt2d(dt), txt: parse_lab(lb), owner: lb.reporter_name)
})
#let all_ds = (order_d.map(oi => oi.ds).dedup() +
  action_d.map(ac => ac.ds).dedup() +
  note_d.map(nt => nt.ds).dedup() +
  cons_c_d.map(cs => cs.ds).dedup() +
  cons_r_d.map(cs => cs.ds).dedup() +
  vs_d.map(vs => vs.ds).dedup() +
  io_d.map(io => io.ds).dedup() +
  lab_d.map(lb => lb.ds).dedup()).dedup().sorted()
#let all = (order_d + action_d + note_d + cons_c_d + cons_r_d + vs_d + io_d + lab_d).sorted(key:i => i.dt)
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[รายละเอียดเหตุการณ์])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an)],
)
#for all_d in all_ds [
  #let data = all.filter(i => i.ds == all_d).map(i => parse_all(i))
  #strong[วันที่ #date_th(all_d)]#v(-10pt)
  #table(columns:(88pt,1fr,120pt),stroke:.5pt,
  table.header(strong[วัน-เวลา],strong[รายละเอียด],strong[เจ้าหน้าที่]),
  ..data.flatten())
]
