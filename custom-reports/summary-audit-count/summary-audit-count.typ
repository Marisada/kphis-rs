#import "@preview/oxifmt:1.0.0": strfmt
#import "customs/config.typ": hospital-name
#import "templates/utils.typ": date_th, thousands
// PRELUDE
#let data = json("data.json")
#let ipt = data.at("data",default: ())
#let audit_types = data.at("audit_type",default: ())
#let wards = data.at("ward",default: ())
// PREPARED FUNCTIONS
#let text_c(c) = align(center,c)
#let table_h(c) = [#align(center,strong(c))]
#let percent(v,m) = if m == 0 [--] else [#strfmt("{:.2}",v * 100 / m)]
#let drg(n) = if n == none [] else [#strfmt("{:.4}",n,fmt-thousands-separator:",")]
// PREPARED VARIABLES
#let audit_type = audit_types.join(" และ ")
#let ward = wards.join(", ")
#let audit = ipt.filter(i => i.com_adjrw != none and i.rev_adjrw != none).map(i => {
  let sas = i.sas.split(",")
  sas.push(i.sa)
  i.sai = sas.map(s => s.trim()).dedup().filter(s => s != none and s != "")
  let cas = i.cas.split(",")
  cas.push(i.ca)
  i.cai = cas.map(s => s.trim()).dedup().filter(s => s != none and s != "")
  i
})
#let adjrw_ch = audit.filter(i => i.com_adjrw != i.rev_adjrw)
#let adjrw_ch_sa = adjrw_ch.filter(i => i.sai.contains("0") and i.sai.len() > 1).len()
#let adjrw_ch_ca = adjrw_ch.filter(i => i.cai.contains("0") and i.cai.len() > 1).len()
#let adjrw_ch_na = adjrw_ch.filter(i => i.sai == ("0") and i.cai == ("0")).len()
#let adjrw_ch_up = adjrw_ch.filter(i => i.rev_adjrw > i.com_adjrw)
#let adjrw_ch_dw = adjrw_ch.filter(i => i.rev_adjrw < i.com_adjrw)
#let sa_len(ty) = audit.filter(i => i.sai.contains(ty)).len()
#let ca_len(ty) = audit.filter(i => i.cai.contains(ty)).len()
#let sa0 = sa_len("0")
#let sa1a = sa_len("1a")
#let sa1b = sa_len("1b")
#let sa1c = sa_len("1c")
#let sa1d = sa_len("1d")
#let sa2a = sa_len("2a")
#let sa2b = sa_len("2b")
#let sa2c = sa_len("2c")
#let sa2d = sa_len("2d")
#let sa3a = sa_len("3a")
#let sa3b = sa_len("3b")
#let sa3c = sa_len("3c")
#let sa3d = sa_len("3d")
#let sa5 = sa_len("5")
#let sa6 = sa_len("6")
#let ca0 = ca_len("0")
#let ca1a = ca_len("1a")
#let ca1b = ca_len("1b")
#let ca1c = ca_len("1c")
#let ca2a = ca_len("2a")
#let ca2b = ca_len("2b")
#let ca2c = ca_len("2c")
#let ca2d = ca_len("2d")
#let ca3a = ca_len("3a")
#let ca3b = ca_len("3b")
#let ca3c = ca_len("3c")
#let ca3d = ca_len("3d")
#let ca6 = ca_len("6")
// RENDER
#set text(font:"TH Sarabun New",size:16pt)
#set page(paper:"a4",margin:(x:1cm,y:2cm),header-ascent:5pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
#h(1fr) #text(size:20pt,weight:700,align(center,[สรุปผลการตรวจสอบเวชระเบียน#hospital-name#linebreak()#audit_type Audit #ward#linebreak()ระหว่างวันที่ #date_th(data.start) ถึงวันที่ #date_th(data.end)]))#h(1fr)
#v(-25pt)
#table(columns:(1fr,50pt,100pt),stroke:.5pt,
  table.header(table_h[สรุปผลการตรวจสอบเวชระเบียน],table_h[จำนวน],table_h[ร้อยละ]),
  [เวชระบียนทั้งหมด],text_c[#ipt.len()],text_c[100],
  [เวชระเบียนที่ได้รับการตรวจสอบ],text_c[#audit.len()],text_c(percent(audit.len(),ipt.len())),
  [ตรวจสอบพบความผิดพลาดที่มีผลต่อการเปลี่ยนกลุ่ม DRG หรือค่า AdjRW],text_c[#adjrw_ch.len()],text_c(percent(adjrw_ch.len(),audit.len())),
  [- จากการสรุปโรคและหัตถการ],text_c[#adjrw_ch_sa],text_c(percent(adjrw_ch_sa,adjrw_ch.len())),
  [- จากการให้รหัสโรคและหัตถการ],text_c[#adjrw_ch_ca],text_c(percent(adjrw_ch_ca,adjrw_ch.len())),
  [- จากการบันทึกข้อมูล],text_c[#adjrw_ch_na],text_c(percent(adjrw_ch_na,adjrw_ch.len())),
  [ตรวจไม่พบความผิดพลาด หรือพบความผิดพลาดที่ไม่มีผลต่อการชดเชย],text_c[#(audit.len()-adjrw_ch.len())],text_c(percent(audit.len()-adjrw_ch.len(),audit.len())),
)
#table(columns:(1fr,50pt,100pt),stroke:.5pt,
  table.header(table_h[การเปลี่ยนแปลงค่าน้ำหนักสัมพัทธ์ในภาพรวม],table_h[จำนวน],table_h[AdjRW]),
  [ค่า AdjRW รวมก่อนการตรวจสอบเวชระเบียน],text_c[#audit.len()],text_c(drg(audit.map(i => i.com_adjrw).sum(default:0))),
  [ค่า AdjRW รวมหลังการตรวจสอบเวชระเบียน],text_c[#audit.len()],text_c(drg(audit.map(i => i.rev_adjrw).sum(default:0))),
  [ค่า AdjRW หลังการตรวจสอบเวชระเบียน เพิ่มขึ้น],text_c[#adjrw_ch_up.len()],text_c(drg(adjrw_ch_up.map(i => i.rev_adjrw - i.com_adjrw).sum(default:0))),
  [ค่า AdjRW หลังการตรวจสอบเวชระเบียน ลดลง],text_c[#adjrw_ch_dw.len()],text_c(drg(adjrw_ch_dw.map(i => i.com_adjrw - i.rev_adjrw).sum(default:0))),
  [ค่า AdjRW รวมการเปลี่ยนแปลงหลังการตรวจสอบเวชระเบียน],text_c[#audit.len()],text_c(drg(audit.map(i => i.rev_adjrw - i.com_adjrw).sum(default:0))),
)
#table(columns:(1fr,50pt,100pt),stroke:.5pt,
  table.header(table_h[ความผิดพลาดจากการสรุปโรคและหัตถการ (Summary Assessment - SA)],table_h[จำนวน],table_h[ร้อยละ]),
  [SA0 : ความเห็นเกี่ยวกับการสรุป สอดคล้องกัน],text_c[#sa0],text_c(percent(sa0,audit.len())),
  table.cell(colspan:3,[SA1 : Principle diagnosis (PDx)]),
  [SA1a : ไม่สรุป PDx],text_c[#sa1a],text_c(percent(sa1a,audit.len())),
  [SA1b : สรุป PDx ไม่ถูกต้อง],text_c[#sa1b],text_c(percent(sa1b,audit.len())),
  [SA1c : สรุป PDx ไม่เฉพาะเจาะจง],text_c[#sa1c],text_c(percent(sa1c,audit.len())),
  [SA1d : สรุป PDx โดยไม่มีหลักฐานในเวชระเบียน],text_c[#sa1d],text_c(percent(sa1d,audit.len())),
  table.cell(colspan:3,[SA2 : Additional diagnosis (Co-morbidity and Complication : CC)]),
  [SA2a : ไม่สรุป CC],text_c[#sa2a],text_c(percent(sa2a,audit.len())),
  [SA2b : สรุป CC ไม่ถูกต้อง],text_c[#sa2b],text_c(percent(sa2b,audit.len())),
  [SA2c : สรุป CC ไม่เฉพาะเจาะจง],text_c[#sa2c],text_c(percent(sa2c,audit.len())),
  [SA2d : สรุป CC โดยไม่มีหลักฐานในเวชระเบียน],text_c[#sa2d],text_c(percent(sa2d,audit.len())),
  table.cell(colspan:3,[SA3 : Operation (OP)]),
  [SA3a : ไม่สรุป Op],text_c[#sa3a],text_c(percent(sa3a,audit.len())),
  [SA3b : สรุป Op ไม่ถูกต้อง],text_c[#sa3b],text_c(percent(sa3b,audit.len())),
  [SA3c : สรุป Op ไม่เฉพาะเจาะจง],text_c[#sa3c],text_c(percent(sa3c,audit.len())),
  [SA3d : สรุป Op โดยไม่มีหลักฐานในเวชระเบียน],text_c[#sa3d],text_c(percent(sa3d,audit.len())),
  [SA5 : ไม่มีการสรุปเวชระเบียน],text_c[#sa5],text_c(percent(sa5,audit.len())),
  [SA6 : ปัญหาอื่น ใช้คำย่อ คำกำกวม อ่านไม่ออก],text_c[#sa6],text_c(percent(sa6,audit.len())),
)
#table(columns:(1fr,50pt,100pt),stroke:.5pt,
  table.header(table_h[ความผิดพลาดจากการให้รหัสโรคและหัตถการ (Coding Assessment - CA)],table_h[จำนวน],table_h[ร้อยละ]),
  [CA0 : ความเห็นเกี่ยวกับการให้รหัส สอดคล้องกัน],text_c[#ca0],text_c(percent(ca0,audit.len())),
  table.cell(colspan:3,[CA1 : Principle diagnosis (PDx)]),
  [CA1a : ไม่ให้รหัส PDx],text_c[#ca1a],text_c(percent(ca1a,audit.len())),
  [CA1b : ให้รหัส PDx ไม่ถูกต้องตามมาตรฐานการให้รหัส],text_c[#ca1b],text_c(percent(ca1b,audit.len())),
  [CA1c : ให้รหัส PDx ไม่เฉพาะเจาะจงตามมาตรฐานการให้รหัส],text_c[#ca1c],text_c(percent(ca1c,audit.len())),
  table.cell(colspan:3,[CA2 : Additional diagnosis (Co-morbidity and Complication : CC)]),
  [CA2a : ไม่ให้รหัส CC],text_c[#ca2a],text_c(percent(ca2a,audit.len())),
  [CA2b : ให้รหัส CC ไม่ถูกต้องตามมาตรฐานการให้รหัส],text_c[#ca2b],text_c(percent(ca2b,audit.len())),
  [CA2c : ให้รหัส CC ไม่เฉพาะเจาะจงตามมาตรฐานการให้รหัส],text_c[#ca2c],text_c(percent(ca2c,audit.len())),
  [CA2d : เพิ่มรหัส CC ไม่ตรงตามมาตรฐานการให้รหัส],text_c[#ca2d],text_c(percent(ca2d,audit.len())),
  table.cell(colspan:3,[CA3 : Operation (OP)]),
  [CA3a : ไม่ให้รหัส Op],text_c[#ca3a],text_c(percent(ca3a,audit.len())),
  [CA3b : ให้รหัส Op ไม่ถูกต้องตามมาตรฐานการให้รหัส],text_c[#ca3b],text_c(percent(ca3b,audit.len())),
  [CA3c : ให้รหัส Op ไม่เฉพาะเจาะจงตามมาตรฐานการให้รหัส],text_c[#ca3c],text_c(percent(ca3c,audit.len())),
  [CA3d : เพิ่มรหัส Op ไม่ตรงตามมาตรฐานการให้รหัส],text_c[#ca3d],text_c(percent(ca3d,audit.len())),
  [CA6 : ปัญหาอื่น ซึ่งอาจทำให้การวินิจฉัยหรือหัตถการของ Doctor และ Coder ต่างกัน],text_c[#ca6],text_c(percent(ca6,audit.len())),
)