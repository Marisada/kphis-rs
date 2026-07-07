#import "templates/utils.typ": api, get_patient_main, date_th, time_th
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let focus = data.at("focus",default: none)
#if focus == none {focus = json(api + "ipd/focus-list-an/" + data.id)}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let table_h(c) = [#align(center,strong(c))]
#let cap_pipe(c,t) = if type(c) != str [] else {
  c.split("|").map(r => {
    let rs = r.split("^")
    if rs.len() == 2 {
      let txt = if rs.at(0) == "999" [ : #t] else []
      [- #rs.at(1) #txt]
    } else {none}
  }).flatten().filter(s => s != none).join()
}
// PREPARED VARIABLES
#let rows = focus.enumerate(start:1).map(((i,f)) => {
  (align(center,[#i]),[#f.focus_name #f.focus_text],cap_pipe(f.goals,f.goal_text),
  align(center,[#date_th(f.fclist_stdate) #time_th(f.fclist_sttime)#linebreak()#f.create_user_name#linebreak()#f.create_user_entryposition #f.create_user_licenseno]),
  align(center,[#if f.fclist_status != "2" and f.dchtype_name != none [#f.dchtype_name#linebreak()]#date_th(f.fclist_enddate) #time_th(f.fclist_endtime) #if f.fclist_enddate != none [#linebreak()#f.update_user_name#linebreak()#f.update_user_entryposition #f.update_user_licenseno]]))
}).flatten()
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[Focus List])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an)],
)
#table(columns:(35pt,3fr,5fr,160pt,160pt),stroke:.5pt,
  table.header(table_h[ลำดับ],table_h[Focus],table_h[เป้าหมาย / ผลลัพธ์ที่ต้องการ],table_h[วันที่เริ่มต้นปัญหา],table_h[วันที่สิ้นสุดปัญหา]),
  ..rows
)