#import "templates/utils.typ": date_th
// PRELUDE
#let data = json("data.json")
#let wards = data.at("ward",default: ())
#let focus = data.at("data",default: ())
// PREPARED FUNCTIONS
#let mid(c) = align(center,c)
#let table_h(c) = [#align(center,strong(c))]
// PREPARED VARIABLES
#let ward = wards.join(", ")
#let rows = focus.enumerate(start:1).map(((i,f)) => {
  (mid[#i],[#f.focus_name],mid[#f.total],mid[#f.ongoing],mid[#f.closed])
}).flatten()
#let sum_total = focus.map(f => f.total).sum(default:0)
#let sum_ongoing = focus.map(f => f.ongoing).sum(default:0)
#let sum_closed = focus.map(f => f.closed).sum(default:0)
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",margin:(x:1cm,y:1.5cm),header-ascent:5pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
#h(1fr) #text(size:20pt,weight:700,align(center,[รายงานจำนวนการใช้ Focus #ward#linebreak()ระหว่างวันที่ #date_th(data.start) ถึงวันที่ #date_th(data.end)]))#h(1fr)
#v(-25pt)
#table(columns:(35pt,3fr,50pt,50pt,50pt),stroke:.5pt,
  table.header(table_h[ลำดับ],table_h[Focus],table_h[ทั้งหมด],table_h[ยังไม่ยุติ],table_h[ยุติแล้ว]),
  ..rows,
  table.cell(colspan:2,table_h[รวมทั้งสิ้น]),mid[#sum_total],mid[#sum_ongoing],mid[#sum_closed]
)