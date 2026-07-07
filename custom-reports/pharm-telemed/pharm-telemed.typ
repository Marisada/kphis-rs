#import "templates/utils.typ": date_th, datetime_th, time_th
// PRELUDE
#let data = json("data.json")
#let rows = data.at("data",default: ())
// PREPARED FUNCTIONS
#let two_digits(f) = calc.round(digits:2,f)
#let text_c(c) = align(center,c)
#let table_h(c) = [#align(center+horizon,strong(c))]
#let render(rs) = table(columns:(30pt,110pt,50pt,180pt,1fr),stroke:.5pt,
  table.header(table_h[ลำดับ],table_h[วันที่-เวลา],table_h[HN],table_h[ชื่อ-สกุล],table_h[รายละเอียด]),
  ..rs.enumerate(start:1).map(((i,row)) => {
    (text_c[#i],text_c(datetime_th(row.telemed_time)),text_c[#row.hn],[#row.pname #row.fname #row.lname],[#row.detail])
  }).flatten(),
)
// PREPARED VARIABLES
#let r_add = rows.filter(r => r.telemed_add != none).map(r => {r.detail = r.telemed_add;r})
#let r_up = rows.filter(r => r.telemed_dose_up != none).map(r => {r.detail = r.telemed_dose_up;r})
#let r_down = rows.filter(r => r.telemed_dose_down != none).map(r => {r.detail = r.telemed_dose_down;r})
#let r_off = rows.filter(r => r.telemed_off != none).map(r => {r.detail = r.telemed_off;r})
#let r_other = rows.filter(r => r.telemed_other != none).map(r => {r.detail = r.telemed_other;r})
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1cm),header-ascent:0pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
#h(1fr) #text(size:20pt,weight:700,align(center,[รายงาน การให้บริการจ่ายยาผ่าน Telemed#linebreak()ระหว่างวันที่ #date_th(data.start) ถึงวันที่ #date_th(data.end) (ทั้งหมด #rows.len() ราย)]))#h(1fr)
#v(-30pt)
#if r_add.len() > 0 [#text(18pt,weight:700,[ยาใหม่])#v(-15pt)#render(r_add)#v(-10pt)]
#if r_up.len() > 0 [#text(18pt,weight:700,[เพิ่มขนาดยา])#v(-15pt)#render(r_up)#v(-10pt)]
#if r_down.len() > 0 [#text(18pt,weight:700,[ลดขนาดยา])#v(-15pt)#render(r_down)#v(-10pt)]
#if r_off.len() > 0 [#text(18pt,weight:700,[หยุดยา])#v(-15pt)#render(r_off)#v(-10pt)]
#if r_other.len() > 0 [#text(18pt,weight:700,[อื่น ๆ])#v(-15pt)#render(r_other)]