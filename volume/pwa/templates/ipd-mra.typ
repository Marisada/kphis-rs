#import "@preview/oxifmt:1.0.0": strfmt
#import "customs/config.typ": hospcode, hospital-name
#import "templates/utils.typ": api, get_patient_main, month_th, date_th, time_th, parse_d
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let mra_data = data.at("mra",default: none)
#if mra_data == none {mra_data = json(api + "ipd/mra?an=" + data.id)}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let table_h(c) = table.cell(fill:luma(100),align(center+horizon,text(fill:white,weight:700,c)))
#let cen(c) = align(center,c)
#let percent(v,m) = if m == 0 [--] else [#strfmt("{:.2}",v * 100 / m)]
#let notna(v) = if v == none {0} else {1}
#let mayna(na,v) = cen(if na [NA] else [#v])
#let some1(v) = if v != none and v {1} else {0}
#let exct(v) = align(center,if v == none [NA] else if v [1] else [0])
#let chk(v) = align(center,if v [\u{2717}] else [])
#let noinput = table.cell(fill:luma(200),[])
#let sd = ("sd_1","sd_2","sd_3","sd_4","sd_5","sd_6","sd_7","sd_8","sd_9")
#let so = ("so_1","so_2","so_3","so_4","so_5","so_6","so_7")
#let ic = ("ic_1","ic_2","ic_3","ic_4","ic_5","ic_6","ic_7","ic_8","ic_9")
#let hx = ("hx_1","hx_2","hx_3","hx_4","hx_5","hx_6","hx_7","hx_8","hx_9")
#let pe = ("pe_1","pe_2","pe_3","pe_4","pe_5","pe_6","pe_7","pe_8","pe_9")
#let pn = ("pn_1","pn_2","pn_3","pn_4","pn_5","pn_6","pn_7","pn_8","pn_9")
#let cr = ("cr_1","cr_2","cr_3","cr_4","cr_5","cr_6","cr_7","cr_8","cr_9")
#let ar = ("ar_1","ar_2","ar_3","ar_4","ar_5","ar_6","ar_7","ar_8","ar_9")
#let on = ("on_1","on_2","on_3","on_4","on_5","on_6","on_7","on_8","on_9")
#let lr = ("lr_1","lr_2","lr_3","lr_4","lr_5","lr_6","lr_7","lr_8","lr_9")
#let rr = ("rr_1","rr_2","rr_3","rr_4","rr_5","rr_6","rr_7","rr_8","rr_9")
#let nn = ("nn_1","nn_2","nn_3","nn_4","nn_5","nn_6","nn_7","nn_8","nn_9")
#let all = (sd,so,ic,hx,pe,pn,cr,ar,on,lr,rr,nn).flatten()
#let sd_v(p) = sd.map(k => some1(p.at(k))).sum()
#let so_v(p) = so.map(k => some1(p.at(k))).sum()
#let ic_v(p) = ic.map(k => some1(p.at(k))).sum()
#let hx_v(p) = hx.map(k => some1(p.at(k))).sum()
#let pe_v(p) = pe.map(k => some1(p.at(k))).sum()
#let pn_v(p) = pn.map(k => some1(p.at(k))).sum()
#let cr_v(p) = cr.map(k => some1(p.at(k))).sum()
#let ar_v(p) = ar.map(k => some1(p.at(k))).sum()
#let on_v(p) = on.map(k => some1(p.at(k))).sum()
#let lr_v(p) = lr.map(k => some1(p.at(k))).sum()
#let rr_v(p) = rr.map(k => some1(p.at(k))).sum()
#let nn_v(p) = {nn.map(k => some1(p.at(k))).sum() - some1(p.at("nn_sub"))}
#let total_max(p) = all.map(k => notna(p.at(k))).sum()
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,top:1.5cm,bottom:1cm),header-ascent:9pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
// Page template
#let body(p) = box[
  #let sd_t = sd_v(p)
  #let so_t = so_v(p)
  #let ic_t = ic_v(p)
  #let hx_t = hx_v(p)
  #let pe_t = pe_v(p)
  #let pn_t = pn_v(p)
  #let cr_t = cr_v(p)
  #let ar_t = ar_v(p)
  #let on_t = on_v(p)
  #let lr_t = lr_v(p)
  #let rr_t = rr_v(p)
  #let nn_t = nn_v(p)
  #let tm = total_max(p)
  #let tv = sd_t+so_t+ic_t+hx_t+pe_t+pn_t+cr_t+ar_t+on_t+lr_t+rr_t+nn_t 
  #section[แบบตรวจประเมินคุณภาพการบันทึกเวชระเบียน#if p.is_psychiatry [ผู้ป่วยจิตเวช กรณี]ผู้ป่วยใน Medical Record Audit Form (IPD)]
  #v(-5pt)#label_note([Hcode],[#hospcode]) #label_note([Hname],[#hospital-name]) #label_note([HN],[#p.hn]) #label_note([AN],[#p.an]) #label_note([Date admitted],date_th(p.adm_date)) #label_note([Date discharged],date_th(p.dch_date))\
  #label_note([การบันทึกช่อง NA: ],[กรณีไม่จำเป็นต้องมีเอกสารใน Content ลำดับที่ 7, 8, 9, 10, 11 เนื่องจากไม่มีการให้บริการ ให้กากบาท ในช่อง NA])\
  #label_note([การบันทึกช่อง Missing: ],[กรณีไม่มีเอกสารให้ตรวจสอบ เวชระเบียนไม่ครบ หรือหายไป ให้ กากบาทในช่อง Missing])\
  #label_note([การบันทึกช่อง No: ],[กรณีมีเอกสารแต่ไม่มีการบันทึกในเอกสารนั้น ให้กากบาทในช่อง No])\
  #label_note([การบันทึกคะแนน: ],[(1) กรณีที่ผ่านเกณฑ์ในแต่ละข้อ ให้ 1 คะแนน  (2) กรณีที่ไม่ผ่านเกณฑ์ในแต่ละข้อ ให้ 0 คะแนน  (3) กรณีไม่จำเป็นต้องมีการบันทึก/ไม่มีข้อมูล #underline[ในเกณฑ์ข้อที่ระบุให้มี NA] ได้ ให้ NA])
  #v(-10pt)#table(
    columns:(150pt,20pt,38pt,20pt,30pt,30pt,30pt,30pt,30pt,30pt,30pt,30pt,30pt,35pt,35pt,1fr),
    table.header(table_h[Content],table_h[NA],table_h[Missing],table_h[No],
      table_h[เกณฑ์ข้อ 1],table_h[เกณฑ์ข้อ 2],table_h[เกณฑ์ข้อ 3],table_h[เกณฑ์ข้อ 4],table_h[เกณฑ์ข้อ 5],table_h[เกณฑ์ข้อ 6],table_h[เกณฑ์ข้อ 7],table_h[เกณฑ์ข้อ 8],table_h[เกณฑ์ข้อ 9],
      table_h[หักคะแนน],table_h[รวมคะแนน],table_h[หมายเหตุ]),
    strong[1. Discharge summary: Dx., OP],noinput,chk(p.sd_m),chk(p.sd_n),..sd.map(k => exct(p.at(k))),noinput,cen[#sd_t],[#p.sd_text],
    strong[2. Discharge summary: Other],noinput,chk(p.so_m),chk(p.so_n),..so.map(k => exct(p.at(k))),noinput,noinput,noinput,cen[#so_t],[#p.so_text],
    strong[3. Informed consent],noinput,chk(p.ic_m),chk(p.ic_n),..ic.map(k => exct(p.at(k))),noinput,cen[#ic_t],[#p.ic_text],
    strong[4. History],noinput,chk(p.hx_m),chk(p.hx_n),..hx.map(k => exct(p.at(k))),noinput,cen[#hx_t],[#p.hx_text],
    strong[5. Physical exam],noinput,chk(p.pe_m),chk(p.pe_n),..pe.map(k => exct(p.at(k))),noinput,cen[#pe_t],[#p.pe_text],
    strong[6. Progress note],noinput,chk(p.pn_m),chk(p.pn_n),..pn.map(k => exct(p.at(k))),noinput,cen[#pn_t],[#p.pn_text],
    strong[7. Consultation record],chk(p.cr_na),chk(p.cr_m),chk(p.cr_n),..cr.map(k => exct(p.at(k))),noinput,mayna(p.cr_na,cr_t),[#p.cr_text],
    strong[8. Anesthetic record],chk(p.ar_na),chk(p.ar_m),chk(p.ar_n),..ar.map(k => exct(p.at(k))),noinput,mayna(p.ar_na,ar_t),[#p.ar_text],
    strong[9. Operative note#if p.is_psychiatry [\*]],chk(p.on_na),chk(p.on_m),chk(p.on_n),..on.map(k => exct(p.at(k))),noinput,mayna(p.on_na,on_t),[#p.on_text],
    strong[10. Labour record],chk(p.lr_na),chk(p.lr_m),chk(p.lr_n),..lr.map(k => exct(p.at(k))),noinput,mayna(p.lr_na,lr_t),[#p.lr_text],
    strong[11. Rehabilitation record],chk(p.rr_na),chk(p.rr_m),chk(p.rr_n),..rr.map(k => exct(p.at(k))),noinput,mayna(p.rr_na,rr_t),[#p.rr_text],
    strong[12. Nurses' note],noinput,chk(p.nn_m),chk(p.nn_n),..nn.map(k => exct(p.at(k))),exct(p.nn_sub),cen[#nn_t],[#p.nn_text],
  )
  #v(-5pt)#align(center,strong[คะแนนเต็ม (Full score) รวม *#tm* คะแนน (ต้องไม่น้อยกว่า #if p.is_psychiatry [57] else [56] คะแนน  คะแนนที่ได้ (Sum score) *#tv* ร้อยละ  #percent(tv,tm)])
  #v(-5pt)#underline[*ประเมินคุณภาพการบันทึกเวชระเบียนในภาพรวม*]\
  *Overall finding* #h(10pt) (#if p.is_not_sorted [\u{2713}] else [\u{2003}]) การจัดเรียงเวชระเบียนไม่เป็นไปตามที่มาตรฐานกำหนด\
  #h(79pt) (#if p.is_unknown [\u{2713}] else [\u{2003}]) เอกสารบางแผ่น ไม่มีชื่อผู้รับบริการ HN AN ทำให้ไม่สามารถระบุได้ว่า เอกสารแผ่นนี้เป็นของใคร จึงไม่สามารถทบทวนเอกสารแผ่นนั้นได้\
  #h(120pt) *(เลือกเพียง 1 ข้อ)* #h(10pt) (#if p.overall == "I" [\u{2713}] else [\u{2003}]) Documentation inadequate for meaningful review #h(5pt) (ข้อมูลไม่เพียงพอสำหรับการทบทวน)\
  #h(202pt) (#if p.overall == "N" [\u{2713}] else [\u{2003}]) No significant medical record issue identified #h(28pt) (ไม่มีปัญหาสำคัญจากการทบทวน)\
  #h(202pt) (#if p.overall == "P" [\u{2713}] else [\u{2003}]) Certain issues in question specify #h(77pt) (มีปัญหาจากการทบทวนที่ต้องค้นต่อ ระบุ : #if p.overall_text == none [#{"."*20}] else [#p.overall_text])\
  #if p.is_psychiatry [*หมายเหตุ* Operative note\* กรณีผู้ป่วยจิตเวช ใช้ประเมินคุณภาพการบันทึก ECT/Psychosocial intervention #linebreak()]
  #align(end,[#label_note([Audit by],[#p.auditor]) #label_note([Audit Date],date_th(p.audit_date))])
]
// render all pages
#for p in mra_data {
  body(p)
}