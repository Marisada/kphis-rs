#import "templates/utils.typ": date_th, time_th
// PRELUDE
#let data = json("data.json")
#let users = data.at("user",default: ())
#let notes = data.at("data",default: ())
// PREPARED FUNCTIONS
#let mid(c) = align(center,c)
#let table_h(c) = [#align(center,strong(c))]
#let empty_zero(s) = if s == none {0} else if s.len() > 0 {1} else {0}
// PREPARED VARIABLES
#let user = users.join(", ")
#let rows = notes.enumerate(start:1).map(((i,n)) => {
  let nt = empty_zero(n.notes)
  let pl = empty_zero(n.problem_lists)
  let sj = empty_zero(n.subjectives)
  let oj = empty_zero(n.objectives)
  let am = empty_zero(n.assessments)
  let pn = empty_zero(n.plans)
  let rs = nt + pl + sj + oj + am + pn
  let tuples = (table.cell(rowspan:rs,mid[#i]),table.cell(rowspan:rs,mid([#date_th(n.progress_note_date)#linebreak()#time_th(n.progress_note_time)])),table.cell(rowspan:rs,mid[#n.an]))
  if rs == 0 {
     tuples.push([])  
  } else {
    if nt > 0 {tuples.push([*Note*: #n.notes])}
    if pl > 0 {tuples.push([*Problem-list*: #n.problem_lists])}
    if sj > 0 {tuples.push([*Subjective*: #n.subjectives])}
    if oj > 0 {tuples.push([*Objective*: #n.objectives])}
    if am > 0 {tuples.push([*Assessment*: #n.assessments])}
    if pn > 0 {tuples.push([*Plan*: #n.plans])}
  }
  tuples.insert(4,table.cell(rowspan:rs,[#n.name]))
  tuples
}).flatten()
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),header-ascent:5pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
#h(1fr) #text(size:20pt,weight:700,align(center,[รายงานรายละเอียดการบันทึก Progreee Note#linebreak()ของ #user#linebreak()ระหว่างวันที่ #date_th(data.start) ถึงวันที่ #date_th(data.end)]))#h(1fr)
#v(-25pt)
#table(columns:(35pt,70pt,70pt,1fr,120pt),stroke:.5pt,
  table.header(table_h[ลำดับ],table_h[วันที่-เวลา],table_h[AN],table_h[รายละเอียด],table_h[เจ้าหน้าที่]),
  ..rows,
)