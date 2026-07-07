#import "customs/config.typ": drug_alert
#import "templates/utils.typ": api, get_patient_main, date_th, month_th, time_th, parse_d_t, parse_d
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let order = data.at("order_item",default: none)
#if order == none {order = json(api + "ipd/order/item?an=" + data.id + "&view_by=doctor")}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:18pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let table_h(c) = [#align(center,strong(c))]
#let bullet(act,d,t) = [#block(above:12pt,below:8pt,strong[- #act : #date_th(d) #time_th(t)])]
#let parse_dt2d(dt) = if dt == none {none} else {
  let (d,t) = dt.split()
  parse_d(d)
}
#let dt2dth(dt) = if dt == none {none} else {
  let m = dt.month()
  let ms = if m < 10 {"0" + str(m)} else {str(m)}
  str(dt.day()) + " " + month_th(ms) + str(dt.year() + 543)
}
#let small_txt(c) = text(10pt,c)
#let doctors(p) = [
  #if p.check_person_name != none [#p.check_person_name \/ ]
  #if p.action_person_1_name != none [#p.action_person_1_name]
  #if p.action_person_2_name != none [ \/ #p.action_person_2_name]
]
#let sortby_dt(oi) = if oi.order_date != none {parse_d_t(oi.order_date,oi.order_time)} else {parse_d_t(pt.regdate,pt.regtime)}
#let badge_inner(m,c,t) = box(radius:3pt,outset:3pt,stroke:.3pt+c,[#text(8pt,fill:c,m)#box(align(center,par(leading:0.2em,text(8pt,fill:c,t))))])
#let badge(m,t) = box(inset:(x:2pt,bottom:1pt),badge_inner(m,black,t))
#let alert_badge(p) = {
  let alert = drug_alert(p.displaycolor)
  ((alert == "HAD",box(inset:(left:8pt,bottom:1pt),badge_inner(none,red,"HAD"))),
   (alert == "LASA",box(inset:(left:8pt,bottom:1pt),badge_inner(none,blue,"LASA"))),
   (true, none)).find(t => t.at(0)).at(1)
}
#let order_name(oi) = {
  let otype = if oi.order_type == "oneday" {badge(none,[ONE DAY])} else if oi.order_type == "continuous" {badge(none,[CONTINUOUS])} else []
  if oi.order_id == none [รายการที่ไม่ผูกกับ order] else [
    #strong[#date_th(oi.order_date) #time_th(oi.order_time)]#linebreak()#otype#alert_badge(oi)#linebreak()
    #if oi.order_item_type == "off" [OFF: #strong(oi.off_med_name) #oi.off_order_item_detail] else [#strong(oi.med_name) #oi.order_item_detail] #if oi.stat == "Y" [\(stat\)]
  ]
}
// PREPARED VARIABLES
#let plan = order.map(oi => oi.index_plans).flatten().map(pn => {
  pn.display = if pn.plan_sch_type == "stat" {"STAT"}
    else if pn.plan_sch_type == "date" {"PRN"}
    else {time_th(pn.plan_time)}
  pn
})
#let action = plan.map(pn => pn.actions).flatten().filter(ac => ac.action_date != none)
#let details = order.sorted(key:oi => sortby_dt(oi)).map(oi => {
  let pns = plan.filter(pn => pn.order_item_id == oi.order_item_id)
  let pnsd = pns.map(pn => pn.display).dedup().sorted()
  let pnsd_len = pnsd.len()
  let tails = if pnsd_len > 0 {
    pnsd.map(display => {
      let pn_ids = pns.filter(pn => pn.display == display).map(pn => pn.plan_id).dedup()
      let acs = action.filter(ac => pn_ids.contains(ac.plan_id)).map(ac => [
        #if ac.action_date != none [#badge(sym.checkmark,[#date_th(ac.action_date) #time_th(ac.action_time)])]
        #small_txt[ #doctors(ac) #if ac.action_result != none and ac.action_result != "" [#strong[ผลลัพธ์:] #ac.action_result]
        #if ac.action_remark != none and ac.action_remark != "" [#strong[หมายเหตุ:] #ac.action_remark]]
      ]).join(linebreak())
      (table_h[#display],acs)
    })
  } else {([],[])}
  (table.cell(rowspan:if pnsd_len > 0 {pnsd_len} else {1},order_name(oi)),..tails)
})
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[Index Plan and Action])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an)],
)
#table(columns:(120pt,45pt,1fr),stroke:.5pt,
  table.header(table_h[Order],table_h[Plan],table_h[Details]),
  ..details.flatten()
)
