#import "templates/utils.typ": date_th, time_th, parse_d_t, parse_dt, watermarks
// PRELUDE
#let data = json("data.json")
#let adm = data.at("data",default: ())
// PREPARED FUNCTIONS
#let mid(c) = align(center,c)
#let table_h(c) = [#align(center,strong(c))]
#let label_note(l,n) = [#text(weight:700,l) #n]
#let ptype(ty) = {
  ((ty == "01","แพทย์"),
  (ty == "02","ทันตแพทย์"),
  (ty == "03","พยาบาล"),
  (ty == "11","เภสัชกร"),
  (true, "อื่นๆ")).find(t => t.at(0)).at(1)
}
#let parse_minutes(f) = {
  let i = int(f)
  let h = int(i / 60)
  let m = calc.rem(i, 60)
  [#if h == 0 [] else [#h ชั่วโมง ]#m นาที]
}
// PREPARED VARIABLES
#let adm = adm.map(a => {
  let rdt = parse_d_t(a.regdate,a.regtime)
  let mdt = parse_dt(a.min_medrec_datetime)
  let pdt = parse_dt(a.phamacist_confirm_datetime)
  let ddt = parse_dt(a.doctor_confirm_datetime)
  a.mm = (mdt - rdt).minutes()
  a.pm = (pdt - mdt).minutes()
  a.dm = (ddt - pdt).minutes()
  a
})
#let rows = adm.enumerate(start:1).map(((i,a)) => {(
  mid[#i],mid[#a.an],mid[#date_th(a.regdate) #time_th(a.regtime)],
  mid(ptype(a.confirm_provider_type)),mid(if a.is_self_confirm==1 [#sym.checkmark] else []),
  mid(parse_minutes(a.mm)),mid(parse_minutes(a.pm)),mid(parse_minutes(a.dm))
)}).flatten()
#let len = adm.len()
#let ave_m = adm.map(a => a.mm).sum(default:0) / len
#let ave_p = adm.map(a => a.pm).sum(default:0) / len
#let ave_d = adm.map(a => a.dm).sum(default:0) / len
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),header-ascent:5pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
  foreground: watermarks(2,55pt,33%),
)
#h(1fr) #text(size:20pt,weight:700,align(center,[รายงานระยะเวลา Med-Reconciliation ผู้ป่วยใน#linebreak()ระหว่างวันที่ #date_th(data.start) ถึงวันที่ #date_th(data.end)]))#h(1fr)
#v(-25pt)
#table(columns:(35pt,80pt,120pt,100pt,65pt,1fr,1fr,1fr),stroke:.5pt,
  table.header(table_h[ลำดับ],table_h[AN],table_h[วัน/เวลา Admit],table_h([วิชาชีพผู้บันทึก]),table_h([เป็นผู้สั่งใช้]),table_h[ระยะเวลาบันทึก],table_h[ระยะเวลายืนยัน],table_h([ระยะเวลาแพทย์ยืนยัน])),
  ..rows,
  table.cell(colspan:5,table_h[ระยะเวลาเฉลี่ย]),mid(parse_minutes(ave_m)),mid(parse_minutes(ave_p)),mid(parse_minutes(ave_d))
)