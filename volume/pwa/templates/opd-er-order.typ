#import "customs/config.typ": hospital-name
#import "templates/utils.typ": api, get_patient_main
#import "templates/utils-order.typ": get_rows
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient", default: none)
#if pt == none {pt = get_patient_main(data.id)}
#let oneday = data.at("oneday", default: none)
#if oneday == none {oneday = if pt.opd_er_order_master_id == none {()} else {json(api + "opd-er/order/order?opd_er_order_master_id=" + str(pt.opd_er_order_master_id) + "&order_type=oneday&view_by=doctor")}}
#let cont = data.at("cont", default: none)
#if cont == none {cont = if pt.opd_er_order_master_id == none {()} else {json(api + "opd-er/order/order?opd_er_order_master_id=" + str(pt.opd_er_order_master_id) + "&order_type=continuous&view_by=doctor")}}
#let note = data.at("note", default: none)
#if note == none {note = if pt.opd_er_order_master_id == none {()} else {json(api + "opd-er/order/progress-note?opd_er_order_master_id=" + str(pt.opd_er_order_master_id))}}
// PREPARED FUNCTIONS
#let thead(c) = table.cell(align:center+horizon,text(size:16pt,weight:700,c))
#let vline(x) = place(top+left,line(stroke:.5pt,start:(x,120pt),end:(x,787pt)))
#let label_note(l,n) = [#text(weight:700,l) #n]
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set block(below:10pt)
#set table(stroke:.5pt)
#set grid(inset:(x:3pt,top:10pt,bottom:5pt))
#set list(marker: [--])
#set page(paper:"a4",
  margin:(x:0.5cm,top:120pt,bottom:55pt),
  header-ascent:0pt,footer-descent:0pt,
  header:context[
    #grid(columns:(1fr,3fr,1fr),rows:2,
      grid.cell(align:center,[#image("/statics/picture/MOPH.svg",width:70pt)]),
      grid.cell(rowspan:2,align:center+horizon,text(size:24pt,weight:700,[*บันทึกคำสั่งแพทย์#linebreak()\(Doctor's Order Sheet Form\)*])),
      grid.cell(rowspan:2,align:right,[FM-ER-01/18 page #counter(page).display("1/1",both:true)]),
      grid.cell(inset:3pt,align:center,text(size:14pt,weight:700,fill:rgb(5,104,57),baseline:-5pt,[#hospital-name])),
    )#v(-10pt)
    #table(columns:(4fr,5fr,5fr),
      thead([Progress Note]),thead([Orders For One Day]),thead([Orders For Continuation]),
    )],
  background:[#vline(14.2pt)#vline(176.1pt)#vline(378.6pt)#vline(581.1pt)],
  footer:[#table(columns:(5fr,2fr,2fr,2fr),
    table.cell(stroke:(right:none),label_note([ชื่อ],[#pt.pname #pt.fname #pt.lname])),
    table.cell(stroke:(x:none),label_note([อายุ],[#pt.age_y ปี])),
    table.cell(stroke:(x:none),label_note([HN],pt.hn)),
    table.cell(stroke:(left:none),label_note([VN],pt.vn)),
  )
  #text(size:18pt,weight:700,align(center,[หมายเหตุ : กรุณาระบุคำสั่ง OFF โดยไม่ต้องย้อนไปยกเลิกคำสั่งเดิม]))]
)
// BODY
#let rows = get_rows(oneday.filter(o => o.order_confirm == "Y"),cont.filter(o => o.order_confirm == "Y"),note).flatten()
#grid(columns:(4fr,5fr,5fr),stroke:(bottom:0.5pt+gray),..rows)
