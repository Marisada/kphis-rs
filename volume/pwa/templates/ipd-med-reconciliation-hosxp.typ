#import "customs/config.typ": hospital-name
#import "templates/utils.typ": api, get_patient_main, one_space
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let recons = data.at("recon",default: none)
#if recons == none {recons = json(api + "ipd/med-reconcile-hosxp-an/" + data.id)}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let thead(c) = table.cell(align:center+horizon,text(size:16pt,weight:700,c))
#let vline(x) = place(top+left,line(stroke:.5pt,start:(x,120pt),end:(x,677pt)))
#let bbrace = [\[#h(8pt)\]]
#let dots(len) = box(width:len, repeat[.])
// PREPARED VARIABLES
#let drugs = recons.map(rc => [+ #bbrace #rc.medication_name#linebreak()#one_space(rc.usage_name)]).join()
// RENDER
#set text(font:"TH Sarabun New",size:14pt)
#set table(stroke:.5pt)
#set grid(inset:(x:3pt,top:10pt,bottom:5pt))
#set page(paper:"a4",
  margin:(x:0.5cm,top:120pt,bottom:165pt),
  header-ascent:0pt,footer-descent:0pt,
  header:[
    #grid(columns:(1fr,3fr,1fr),rows:2,
      grid.cell(align:center,[#image("/statics/picture/MOPH.svg",width:70pt)]),
      grid.cell(rowspan:2,align:center+horizon,text(size:24pt,weight:700,[*บันทึกคำสั่งแพทย์#linebreak()\(Doctor's Order Sheet Form\)*])),
      grid.cell(rowspan:2,align:right,[FM-IPD-01/18]),
      grid.cell(inset:3pt,align:center,text(size:18pt,weight:700,fill:rgb(5,104,57),baseline:-5pt,[#hospital-name])),
    )#v(-15pt)
    #table(columns:(4fr,3fr,7fr),
      thead([Progress Note]),thead([Orders For One Day]),thead([Orders For Continuation]),
    )],
  background:[#vline(14.2pt)#vline(176.1pt)#vline(297.7pt)#vline(581.1pt)],
  footer:[#table(columns:(4fr,3fr,2fr,2fr,2fr),
    table.cell(colspan:5,align(center,[\*\*\*เภสัชกร จะไม่จ่ายยา จนกว่าแพทย์ จะทบทวนรายการยาเดิมที่สั่งใช้และมีลายเซ็นกำกับ\*\*\*])),
    table.cell(colspan:5,[#strong[การจัดการยาเดิมผู้ป่วย]\
      #bbrace ไม่ได้นำยาเดิมมาด้วย#h(5pt)>>> วันที่ D/C #dots(65pt) #bbrace จัดยาเพิ่มเติมให้ผู้ป่วย #bbrace จัดยาเพิ่มเติมให้ผู้ป่วยพร้อมทบทวนรายการยาครบถ้วนแล้ว\
      #bbrace นำยาเดิมมาด้วย#h(23pt)>>> วันที่ D/C #dots(65pt) #bbrace คืนยาเดิมให้ผู้ป่วยแล้ว\
      #v(10pt)#h(150pt)เภสัชกร#dots(120pt)วันที่ทบทวนรายการยาเดิม/รับออเดอร์#dots(80pt)
    ]),
    label_note([Department],pt.at("spclty_name",default: none)),
    label_note([Ward],pt.at("ward_name",default: none)),
    table.cell(colspan:3,label_note([Attending Physician],[])),
    table.cell(stroke:(right:none),label_note([ชื่อ],[#pt.pname #pt.fname #pt.lname])),
    table.cell(stroke:(x:none),label_note([อายุ],[#pt.age_y ปี])),
    table.cell(stroke:(x:none),label_note([เตียง],pt.at("bedno",default: none))),
    table.cell(stroke:(x:none),label_note([HN],pt.hn)),
    table.cell(stroke:(left:none),label_note([AN],pt.an)),
  )#v(-12pt)
  #text(size:18pt,weight:700,align(center,[หมายเหตุ : กรุณาระบุคำสั่ง OFF โดยไม่ต้องย้อนไปยกเลิกคำสั่งเดิม]))]
)
#grid(columns:(4fr,3fr,7fr),
  [#strong[แหล่งที่มาข้อมูล]\
  #h(10pt)#bbrace สมุดประจำตัวผู้ป่วย\
  #h(10pt)#bbrace HOSxP\
  #h(10pt)#bbrace ซักประวัติผู้ป่วยหรือญาติ\
  #h(10pt)#bbrace สอบถามจากสถานพยาบาลอื่น\
  #h(26pt)ที่จ่ายยาให้ผู้ป่วย\
  #h(10pt)#bbrace ยาเดิมที่ผู้ป่วยนำมาด้วย\
  #strong[ปัญหาเกี่ยวเนื่องกับยา]\
  #h(10pt)#bbrace ADR\
  #h(10pt)#bbrace D/I\
  #h(10pt)#bbrace Non compliance\
  #h(10pt)#bbrace อื่น ๆ\
  #strong[Dose ยาล่าสุดที่ผุ้ป่วยใช้]\
  #h(10pt)#bbrace ก่อนอาหาร#h(20pt)#bbrace หลังอาหาร\
  #h(10pt)#bbrace เช้า#h(50pt)#bbrace กลางวัน\
  #h(10pt)#bbrace เย็น#h(48pt)#bbrace ก่อนนอน\
  #h(10pt)#bbrace ยังไม่กินยา],[],
  [#strong[Medical Reconciliation]#linebreak()#drugs]
)
