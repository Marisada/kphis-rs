#import "customs/config.typ": drug_alert, is_inj
#import "templates/utils.typ": api, get_patient_main, date_th, datetime_th, month_th, time_th, parse_d_t, parse_d
// PRELUDE
#let eachpage = 4
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let order = data.at("order_item",default: none)
#if order == none {order = json(api + "ipd/order/item?an=" + data.id + "&view_by=doctor")}
#let medpay = data.at("med_pay",default: none)
#if medpay == none {medpay = json(api + "ipd/index-med-pay-an/" + data.id)}
// PREPARED FUNCTIONS
#let label_note(l,n) = [#text(weight:700,l) #n]
#let table_h(c) = [#align(center,strong(c))]
#let parse_dt2d(dt) = if dt == none {none} else {
  let (d,t) = dt.split()
  parse_d(d)
}
#let dt2dth(dt) = if dt == none {none} else {
  let m = dt.month()
  let ms = if m < 10 {"0" + str(m)} else {str(m)}
  str(dt.day()) + " " + month_th(ms) + str(dt.year() + 543)
}
#let datetime_to_date_th(dt) = if dt == none {none} else {
  let (d,t) = dt.split()
  time_th(t)
}
#let naming(c) = text(10pt,c)
#let doctors(p) = naming([
  #if p.check_person_name != none [#p.check_person_name \/ ]
  #if p.action_person_1_name != none [#p.action_person_1_name]
  #if p.action_person_2_name != none [ \/ #p.action_person_2_name]
])
#let sortby_dt(oi) = if oi.order_date != none {parse_d_t(oi.order_date,oi.order_time)} else {parse_d_t(pt.regdate,pt.regtime)}
#let badge_inner(m,c,t) = box(radius:3pt,outset:3pt,stroke:.5pt+c,[#text(8pt,fill:c,m)#box(align(center,par(leading:0.2em,text(8pt,fill:c,weight:700,t))))])
#let badge(m,t) = box(inset:(x:2pt,bottom:1pt),badge_inner(m,black,t))
#let badge_red(m,t) = box(inset:(x:2pt,bottom:1pt),badge_inner(m,red,t))
#let drug_alert_badge(c) = {
  let alert = drug_alert(c)
  ((alert == "HAD",box(inset:(left:8pt,bottom:1pt),badge_inner(none,red,"HAD"))),
   (alert == "LASA",box(inset:(left:8pt,bottom:1pt),badge_inner(none,blue,"LASA"))),
   (true, none)).find(t => t.at(0)).at(1)
}
#let drug_alert_num(c) = {
  let alert = drug_alert(c)
  ((alert == "HAD",0),
   (alert == "LASA",1),
   (true, 2)).find(t => t.at(0)).at(1)
}
#let order_name(oi) = {
  let otype = if oi.order_type == "oneday" {badge(none,[ONE DAY])} else if oi.order_type == "continuous" {badge(none,[CONTINUOUS])} else []
  if oi.order_id == none [รายการที่ไม่ผูกกับ order] else [
    #let is_offed = oi.off_by_datetime != none
    \u{2022} #strong[#date_th(oi.order_date) #time_th(oi.order_time)]\
    #if is_offed [#text(red,weight:700,[OFF #datetime_th(oi.off_by_datetime)])#linebreak()]
    #otype#drug_alert_badge(oi.displaycolor)#if oi.stat == "Y" [#h(5pt)#badge_red(none,[STAT])]#if oi.allergy_agent_symptom != none [#h(5pt)#badge_red(none,[แพ้ยา/เฝ้าระวัง])]\
    #if is_offed {strike(oi.order_item_detail)} else {oi.order_item_detail}
  ]
}
// PREPARED VARIABLES
#let oi_icodes = order.filter(oi => oi.icode != none).map(oi => (oi.icode,oi.med_name,oi.displaycolor,is_inj(oi.dosageform)))
#let py_icodes = medpay.map(py => (py.icode,py.med_name,py.displaycolor,is_inj(py.dosageform)))
#let icodes = (..oi_icodes,..py_icodes).dedup(key:((i,_,_,_)) => i).sorted(key: ((i,n,c,j)) => (not j,drug_alert_num(c),n))
#let medpay = medpay.filter(p => {
  if p.med_pay_qty != none {p.med_pay_qty > 0} else {p.med_order_qty != none and p.med_order_qty > 0} and (p.pay_date_time != none or p.order_date_time != none)
}).map(p => {
  p.d = parse_dt2d(if p.pay_date_time != none {p.pay_date_time} else {p.order_date_time});p
})
#let render_icode(icode,title) = {
  let med = order.filter(oi => oi.icode == icode).map(oi => {
    oi.ds = (parse_d(oi.order_date),oi.index_plans.map(pn => pn.actions.map(ac => parse_d(ac.action_date)))).flatten().dedup().filter(d => d != none);oi
  })
  let pay = medpay.filter(p => p.icode == icode and p.d != none)
  let plan = med.map(oi => oi.index_plans).flatten().map(pn => {
    pn.ds = pn.actions.map(ac => parse_d(ac.action_date)).flatten().dedup().filter(d => d != none)
    pn.display = if pn.plan_sch_type == "stat" {"STAT"}
      else if pn.plan_sch_type == "date" {"PRN"}
      else {time_th(pn.plan_time)};pn
  })
  let action = plan.map(pn => pn.actions).flatten().map(ac => {
    ac.d = parse_d(ac.action_date);ac
  })
  let heads = (action.filter(ac => ac.d != none).map(ac => ac.d) + pay.map(p => p.d)).dedup().sorted().chunks(eachpage)
  if heads.len() == 0 [#block(breakable:false,[#title#v(-7pt)
    #table(columns:(120pt,45pt,..(1fr,)*eachpage),stroke:.5pt,
      table.header(table_h[Order],table_h[Plan],..([],)*eachpage),
      table.cell(breakable:false,med.map(mp => order_name(mp)).join(linebreak())),[],..([],)*eachpage)
    ])] else {
    let last_len = heads.last().len()
    if last_len < eachpage {
      heads.last() = heads.last() + (none,)*(eachpage - last_len)
    }
    let details = heads.map(col => {
      let medp = med.filter(oi => oi.ds.any(d => col.contains(d))).sorted(key:mp => sortby_dt(mp))
      let payp = pay.filter(p => col.contains(p.d))
      let col1 = if medp.len() > 0 {medp.map(mp => order_name(mp)).join(linebreak())} else [#title#linebreak()ใช้ร่วมกับคำสั่งแพทย์อื่นๆ]
      let ps = col.map(d => {
        payp.filter(p => p.d == d).map(p => [
          #if p.med_pay_qty != none {badge("+",str(p.med_pay_qty))} else {badge("+",str(p.med_order_qty))} #h(1pt)#badge(none,datetime_to_date_th(p.order_date_time)) #naming(p.entryuser)
        ]).join(linebreak())
      })
      let pns = plan.filter(pn => pn.ds.any(d => col.contains(d)))
      let pnsd = plan.map(pn => pn.display).dedup().sorted()
      let pnsd_len = pnsd.len()
      let tails = pnsd.map(display => {
        let pn_ids = pns.filter(pn => pn.display == display).map(pn => pn.plan_id).dedup()
        let acs = col.map(d => {
          action.filter(ac => ac.d == d and pn_ids.contains(ac.plan_id)).map(ac => [#badge(sym.checkmark,time_th(ac.action_time)) #doctors(ac)]).join(linebreak())
        })
        (table_h[#display],..acs)
      })
      (table.cell(rowspan:pnsd_len+1,col1),table_h[Rx],..ps,..tails)
    })
    for (head,detail) in heads.zip(details).filter(((head,detail)) => detail.len() > 0) [
      #v(1pt)#title#v(-7pt)
      #table(columns:(120pt,45pt,..(1fr,)*eachpage),stroke:.5pt,
        table.header(table_h[Order],table_h[Plan],..head.map(p => table_h[#dt2dth(p)])),
        ..detail.flatten()
      )
    ]
  }
}
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[Medication Administration Record (eMAR)])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an)],
)
#for ((icode,med_name,displaycolor,_)) in icodes {render_icode(icode,[\u{2022} #strong[#med_name]#h(0pt)#drug_alert_badge(displaycolor)])}