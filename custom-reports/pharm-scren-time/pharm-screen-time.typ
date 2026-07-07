#import "templates/utils.typ": date_th, datetime_th, time_th
// PRELUDE
#let data = json("data.json")
#let rows = data.at("data",default: ())
// PREPARED FUNCTIONS
#let two_digits(f) = calc.round(digits:2,f)
#let text_c(c) = align(center,c)
#let table_h(c) = [#align(center+horizon,strong(c))]
// PREPARED VARIABLES
#let rows_len = rows.len()
#let checks_sum = if rows_len > 0 {two_digits(rows.map(row => row.check_secs).sum(default:0) / (rows_len * 60))} else []
#let done_sum = if rows_len > 0 {two_digits(rows.map(row => row.done_secs).sum(default:0) / (rows_len * 60))} else []
#let all_sum = if rows_len > 0 {two_digits(rows.map(row => row.all_secs).sum(default:0) / (rows_len * 60))} else []
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",margin:(x:1cm,y:1.5cm),header-ascent:5pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
#h(1fr) #text(size:20pt,weight:700,align(center,[รายงาน ระยะเวลารอคอยรับยา (Waiting time)#linebreak()#data.start_time น. - #data.end_time น. ระหว่างวันที่ #date_th(data.start_date) ถึงวันที่ #date_th(data.end_date)]))#h(1fr)
#v(-30pt)
#table(columns:(30pt,70pt,80pt,60pt,65pt,50pt,65pt,50pt,1fr),stroke:.5pt,
  table.header(table.cell(rowspan:2,table_h[ลำดับ]),table.cell(rowspan:2,table_h[วันที่]),table.cell(rowspan:2,table_h[VN]),table.cell(rowspan:2,table_h[รับใบสั่งยา#linebreak()เวลา]),
    table.cell(colspan:2,table_h[ตรวจสอบยา]),table.cell(colspan:2,table_h[จ่ายยา]),table.cell(rowspan:2,table_h[ระยะเวลา#linebreak()รอรับยา#linebreak()\(นาที\)]),
    table_h[เวลาที่#linebreak()ตรวจสอบยา],table_h[รวมเวลา#linebreak()\(นาที\)],table_h[เวลาที่#linebreak()จ่ายยา],table_h[รวมเวลา#linebreak()\(นาที\)]),
  ..rows.enumerate(start:1).map(((i,row)) => {
    (text_c[#i],text_c(date_th(row.accept_date)),text_c[#row.vn],text_c(time_th(row.accept_time)),text_c(time_th(row.check_time)),text_c[#two_digits(row.check_secs / 60)],
    text_c(time_th(row.done_time)),text_c[#two_digits(row.done_secs / 60)],text_c[#two_digits(row.all_secs / 60)])
  }).flatten(),
  table.cell(colspan:4,table_h[ระยะเวลารอคอยเฉลี่ย (#rows_len รายการ)]),table.cell(colspan:2,table_h[#checks_sum นาที]),table.cell(colspan:2,table_h[#done_sum นาที]),table_h[#all_sum นาที]
)