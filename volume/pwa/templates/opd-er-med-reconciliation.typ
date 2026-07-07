#import "templates/utils.typ": api, get_patient_main, date_th, datetime_th, one_space, if_empty
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none {pt = get_patient_main(data.id)}
#let recons = data.at("recon",default: none)
#if recons == none {recons = if pt.opd_er_order_master_id == none {()} else {json(api + "opd-er/med-reconcile?opd_er_order_master_id=" + str(pt.opd_er_order_master_id))}}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let table_h(c) = [#align(center,strong(c))]
#let used_txt(s) = strong(
  ((s == "Y","สั่งใช้"),
   (s == "N","ไม่สั่งใช้"),
   (s == "H","Hold"),
   (true, "")).find(t => t.at(0)).at(1)
)
#let item_row((i,item)) = (align(center,[#i]),if_empty(item.custom_med_name,item.med_name,item.custom_med_name),one_space(item.old_drugusage),align(center,[#item.receive_qty]),align(center,date_th(item.receive_date)),[#item.receive_from],[#used_txt(item.used) #if item.used == "Y" {if item.changed_drugusage == none [: วิธีใช้เดิม] else [: #one_space(item.changed_drugusage)]}],align(center,datetime_th(item.last_dose_taken_time)),[#item.last_dose_taken_remark])
#let render_rc(rc) = box[#v(5pt)
  #label_note([วันที่ : ],datetime_th(rc.med_reconciliation_datetime))#h(10pt)
  #label_note([ผู้บันทึกรายการ : ],[#rc.pharmacist_name #if rc.phamacist_confirm_datetime != none [\(#datetime_th(rc.phamacist_confirm_datetime)\)]])#h(10pt)
  #label_note([แพทย์ผู้พิจารณา : ],[#rc.doctor_name #if rc.doctor_confirm_datetime != none [\(#datetime_th(rc.doctor_confirm_datetime)\)]])#h(10pt)
  #if rc.note != none [#label_note([หมายเหตุ : ],[#rc.note])]#linebreak()#v(-8pt)
  #table(columns:(20pt,3fr,3fr,30pt,55pt,70pt,3fr,90pt,95pt),stroke:.5pt,
    table.header(table_h[\#],table_h[ชื่อยา],table_h[วิธีใช้],table_h[ได้รับ],table_h[วันที่ได้รับยา],table_h[สถานพยาบาล],table_h[คำสั่งแพทย์],table_h[Lase Dose],table_h[จำนวนยาเหลือ/หมายเหตุ]),..rc.med_reconciliation_items.enumerate(start:1).map(tuple => item_row(tuple)).flatten())
]
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[ใบ Medical Reconciliation Sheet])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([VN : ],pt.vn)],
)
#recons.map(rc => render_rc(rc)).join()
