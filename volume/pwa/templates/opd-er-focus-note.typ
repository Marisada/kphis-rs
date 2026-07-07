#import "templates/utils.typ": api, get_patient_main, date_th, time_th, explode_imgs, square_bracket_to_span
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none {pt = get_patient_main(data.id)}
#let note = data.at("note",default: none)
#if note == none {note = if pt.opd_er_order_master_id == none {()} else {json(api + "opd-er/focus-note-id/" + str(pt.opd_er_order_master_id))}}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let table_h(c) = [#align(center,strong(c))]
#let cap_pipe(c,b) = if c == none {none} else {
  c.split("|").map(r => {
    let rs = r.split("^")
    if rs.len() == 2 and rs.at(0) != "9999" {[- #if b {square_bracket_to_span(rs.at(1))} else {rs.at(1)}]} else {none}
  }).flatten().filter(s => s != none).join()
}
#let split_line(c,b) = if c == none {none} else {
  list(..c.replace("\n","").split("- ").filter(r => r != "").map(r => if b {square_bracket_to_span(r)} else [#r]))
}
// PREPARED VARIABLES
#let rows = note.map(n => {
  let aie = [#if n.general_symptoms != none [#square_bracket_to_span(n.general_symptoms)#linebreak()]
    A : #explode_imgs(5,true,n.a_imgs)#split_line(n.assessment, true)
    I : #cap_pipe(n.intvts, true)#if n.intvt_text != none [#v(-7pt)#split_line(n.intvt_text, true)]
    #cap_pipe(n.dlcs, false)#if n.dlc_text != none [#v(-7pt)#split_line(n.dlc_text, false)]
    E : #explode_imgs(5,true,n.e_imgs)#square_bracket_to_span(n.evalution)
    #if n.other != none [#linebreak()#square_bracket_to_span(n.other)]]
  (align(center,[#date_th(n.fcnote_date)#linebreak()#time_th(n.fcnote_time)]),
  align(center,[#n.fcnote_patient_type]),[#n.focus_name #n.focus_text],aie,
  align(center,[#n.user_name#linebreak()#n.entryposition #n.licenseno]))
}).flatten()
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[บันทึกความก้าวหน้าทางการพยาบาล])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([VN : ],pt.vn)],
)
#table(columns:(60pt,42pt,3fr,5fr,120pt),stroke:.5pt,
  table.header(table_h[วัน เดือน ปี#linebreak()เวลา],table_h[ประเภท#linebreak()ผู้ป่วย],table_h[Focus],table_h[บันทึกความก้าวหน้าทางการพยาบาล#linebreak()A:Assessment, I:Intervention, E:Evaluation],table_h[ผู้บันทึก#linebreak()ตำแหน่ง]),
  ..rows
)
