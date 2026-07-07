#import "customs/config.typ": hospital-name, hospital-info
#import "templates/utils.typ": api, cid, vnan_is_ipd, get_patient_main, date_th, date_th_full, time_th
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let is_ipd = vnan_is_ipd(data.id)
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let referouts = data.at("referout", default: none)
#if referouts == none {referouts = json(api + "his/refer-out-vnan/" + data.id)}
// PREPARED FUNCTIONS
#let label_note(l,n) = [#text(weight:700,l) #n]
#let checkbox_eq(v,ev,t) = [#if v == ev {sym.ballot.cross} else {sym.ballot} #t]
#let render(r, with_break) = [
  #let refer = r.referout
  #let vs = r.vital_signs.first(default:none)
  #align(center,text(22pt, weight:700, [แบบสำหรับส่งผู้ป่วยไปรับการตรวจหรือรักษาต่อ]))
  #grid(columns:(1fr,1fr),label_note("เลขที่ ", [#refer.refer_number]),label_note("วันที่ ", date_th_full(refer.refer_date)))
  #v(0pt)#label_note("จาก ", [#hospital-name #hospital-info])\
  #v(0pt)#label_note("ถึง ", [#refer.refer_hospcode_name])\
  #v(5pt)#h(50pt)พร้อมหนังสือนี้ ขอส่งผู้ป่วยชื่อ #strong[#pt.pname#pt.fname #pt.lname] เพศ #strong[#pt.sex_name] อายุ #strong[#pt.age_y] ปี อยู่บ้านเลขที่ #strong[#pt.homeaddr] #if pt.cid != none [เลขประจำตัวประชาชน #strong[#cid(pt.cid)]] เลขประจำตัวผู้ป่วย HN: #strong[#refer.hn] #if is_ipd [AN] else [VN]: #strong[#refer.vn] สิทธิ์การรักษา #strong[#pt.pttype_name] มาเพื่อโปรด\
  #v(-5pt)#h(25pt)#checkbox_eq(refer.refer_cause,1,"รับไว้รักษาต่อ")#h(70pt)#checkbox_eq(refer.refer_cause,2,"ตรวจชันสูตร")#h(70pt)#checkbox_eq(refer.refer_cause,3,"คุมไว้สังเกต")#h(70pt)#checkbox_eq(refer.refer_cause,4,"ขอทราบผล")\
  #v(0pt)#label_note("๑. ประวัติการป่วยในอดีต : ", [#refer.pmh])\
  #v(0pt)#label_note("๒. ประวัติการป่วยปัจจุบัน : ", [#if vs != none [CC : #vs.cc#linebreak()PE : #vs.pe#linebreak()]#refer.hpi])\
  #v(0pt)#label_note("๓. ผลการตรวจชันสูตรทางห้องทดลองที่สำคัญ : ", [#linebreak()#refer.lab_text])\
  #v(0pt)#label_note("๔. การวินิจฉัยโรคขั้นต้น : ", [#refer.pre_diagnosis])\
  #v(0pt)#label_note("๕. การรักษาที่ให้ไว้แล้ว : ", [#linebreak()#refer.treatment_text])\
  #v(0pt)#label_note("๖. สาเหตุที่ส่ง : ", [#refer.request_text])\
  #v(0pt)#label_note("๗. รายละเอียดอื่นๆ : ", [#refer.other_text])\
  #v(0pt)#label_note("๘. ใบส่งตัวนี้ใช้ได้ถึงวันที่ : ", date_th(refer.expire_date))\
  #v(5pt)#h(250pt)#box(width: 280pt, [#align(center, "ขอแสดงความนับถือ")
  #align(left, [#h(50pt)\(ลงชื่อ)])
  #align(center, [(#refer.doctor_name)])
  #v(-5pt)#align(center, [#date_th(refer.refer_date) #time_th(refer.refer_time)])])
  #if with_break [#pagebreak()]
]
// RENDER
#set text(font:"TH Sarabun New",size:16pt)
#set page(paper:"a4",margin: (top: 2cm, rest: 1cm),
  header: context[#h(1fr)#counter(page).display("หน้า 1/1",both:true)],
)
#let len = referouts.len()
#if len > 0 {
  for (i,referout) in referouts.enumerate(start:1) {render(referout, i < len)}
}