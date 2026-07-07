#import "customs/config.typ": hospital-name, hospital-info
#import "templates/utils.typ": api, cid, vnan_is_ipd, get_patient_main, date_th, date_th_full, time_th
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let is_ipd = vnan_is_ipd(data.id)
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let refernotes = data.at("refernote", default: none)
#if refernotes == none {refernotes = json(api + "refer-note-vnan/" + data.id)}
// PREPARED FUNCTIONS
#let label_note(l,n) = [#text(weight:700,l) #n]
#let checkbox_eq(v,ev,t) = [#if v == ev {sym.ballot.cross} else {sym.ballot} #t]
#let render(r, with_break) = [
  #place(top,dy:-35pt,image(width: 50pt, "statics/picture/krut.svg"))
  #align(center,text(24pt, weight:700, [บันทึกข้อความ]))
  #v(0pt)#label_note("ส่วนราชการ ", [#hospital-name #hospital-info])\
  #grid(columns:(1fr,1fr),label_note("ที่ ", [#r.docno]),label_note("วันที่ ", date_th_full(r.refer_date)))
  #v(0pt)#label_note("เรื่อง ", "ส่งตัวผู้ป่วยเพื่อรักษาต่อ")\
  #v(0pt)#label_note("เรียน ", [แพทย์ผู้เกี่ยวข้อง #if r.refer_hospcode_name != none [#r.refer_hospcode_name]])\
  #v(5pt)#h(50pt)พร้อมหนังสือนี้ ขอส่งผู้ป่วยชื่อ #strong[#pt.pname#pt.fname #pt.lname] เพศ #strong[#pt.sex_name] อายุ #strong[#pt.age_y] ปี อยู่บ้านเลขที่ #strong[#pt.homeaddr] #if pt.cid != none [เลขประจำตัวประชาชน #strong[#cid(pt.cid)]] เลขประจำตัวผู้ป่วย HN: #strong[#r.hn] #if is_ipd [AN] else [VN]: #strong[#r.vn] สิทธิ์การรักษา #strong[#pt.pttype_name] มาเพื่อโปรดรับไว้รักษาต่อ โดยมีประวัติดังต่อไปนี้\
  #v(0pt)#label_note("๑. ประวัติการป่วยในอดีต : ", [#r.pmh])\
  #v(0pt)#label_note("๒. ประวัติการป่วยปัจจุบัน : ", [#if r.cc != none [CC : #r.cc#linebreak()]#if r.pe != none [PE : #r.pe#linebreak()]#r.hpi])\
  #v(0pt)#label_note("๓. ผลการตรวจชันสูตรทางห้องทดลองที่สำคัญ : ", [#linebreak()#r.lab_text])\
  #v(0pt)#label_note("๔. การวินิจฉัยโรคขั้นต้น : ", [#r.diagnosis_text])\
  #v(0pt)#label_note("๕. การรักษาที่ให้ไว้แล้ว : ", [#linebreak()#r.treatment_text])\
  #v(0pt)#label_note("๖. สาเหตุที่ส่ง : ", [#r.request_text])\
  #v(0pt)#label_note("๗. รายละเอียดอื่นๆ : ", [#r.other_text])\
  #v(5pt)#h(250pt)#box(width: 280pt, [#align(center, "ขอแสดงความนับถือ")
  #align(left, [#h(50pt)\(ลงชื่อ)])
  #align(center, [(#r.doctor_name)])
  #v(-5pt)#align(center, [#date_th(r.refer_date) #time_th(r.refer_time)])])
  #if with_break [#pagebreak()]
]
// RENDER
#set text(font:"TH Sarabun New",size:16pt)
#set page(paper:"a4",margin: (top: 2cm, rest: 1cm),
  header: context[#h(1fr)#counter(page).display("หน้า 1/1",both:true)],
)
#let len = refernotes.len()
#if len > 0 {
  for (i,refernote) in refernotes.enumerate(start:1) {render(refernote, i < len)}
}