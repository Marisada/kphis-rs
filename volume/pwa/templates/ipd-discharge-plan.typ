#import "templates/utils.typ": api, get_patient_main, date_th, time_th, datetime_th
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let dc_plans = data.at("dc_plans",default: none)
#if dc_plans == none {dc_plans = json(api + "ipd/dc-plan-an/" + data.id)}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
#let checkbox_eq(v,ev,t) = [#if v == ev {sym.ballot.cross} else {sym.ballot} #t]
#let checkbox_with_text(v,l) = if v != none [#sym.ballot.cross #l #v] else [#sym.ballot #l]
#let table_h(c) = [#align(center,strong(c))]
#let assess_checks(p,r,o) = [
  #checkbox_eq(p,"Y","ผู้ป่วยเข้าใจ")\
  #checkbox_eq(r,"Y","ญาติเข้าใจ")\
  #checkbox_with_text(o,"อื่นๆ")]
#let cap_pipe(c) = if type(c) != str [] else {
  c.split("|").map(r => {
    let rs = r.split("^")
    if rs.len() == 2 [- #rs.at(1)] else {none}
  }).flatten().filter(s => s != none).join()
}
#let list_text(s) = if s == none [] else if type(s) == str and s.contains("\n- ") [#s.trim("- ").split("\n- ").map(r => [- #r]).join()] else [- #s.trim("- ")]
// RENDER
#set page(paper:"a4",margin:1cm)
#set text(font:"TH Sarabun New",size:14pt)
#let render(dcp) = [
  #section[บันทึกการพยาบาลก่อนจำหน่าย (Discharge Plan)]
  #label_note([ชื่อ-สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน])
    #label_note([HN : ],pt.hn) #label_note([AN : ],pt.an) #label_note([หอผู้ป่วย : ],pt.at("ward_name",default: none))\
  #label_note([วันที่จำหน่าย : ],datetime_th(dcp.dc_datetime)) โดย
    #checkbox_eq(dcp.dc_type_ok,"Y","แพทย์อนุญาต")
    #checkbox_eq(dcp.dc_type_refer,"Y","Refer")
    #checkbox_with_text(dcp.dc_type_other,"อื่นๆ")\
  #label_note([อาการและอาการแสดงก่อนจำหน่าย : ],dcp.dc_symptom)\
  #label_note([อุปกรณ์ที่ติดตัวผู้ป่วย : ],[
    #checkbox_eq(dcp.inst_none,"Y","ไม่มี")
    #checkbox_eq(dcp.inst_foley,"Y","สายสวนปัสสาวะ")
    #checkbox_eq(dcp.inst_ett,"Y","ET-Tube")
    #checkbox_eq(dcp.inst_tt,"Y","TT-Tube")
    #checkbox_eq(dcp.inst_ng,"Y","NG-Tube")
    #checkbox_with_text(dcp.inst_other,"อื่นๆ")])\
  #label_note([สิ่งที่ผู้ป่วยได้รับก่อนจำหน่าย : ],[
    #checkbox_eq(dcp.with_drug,"Y","ยา")
    #checkbox_eq(dcp.with_appoint,"Y","ใบนัด")
    #checkbox_eq(dcp.with_cert,"Y","ใบรับรองแพทย์")
    #checkbox_with_text(dcp.with_other,"อื่นๆ")])
  #table(columns:(1fr,80pt,120pt),stroke:.5pt,
    table.header(table_h[คำแนะนำก่อนจำหน่าย],table_h[ประเมินผล],table_h[ผู้ให้คำแนะนำ]),
    [#text(weight:700,[Diagnosis :])\
        - #label_note([ชื่อโรคที่เจ็บป่วย : ], dcp.dx_name)
        - #label_note([ความรู้ที่แนะนำ : ], dcp.dx_knowledge)
        #list_text(dcp.dx_text)],
      [#assess_checks(dcp.dx_patient_ok,dcp.dx_relatives_ok,dcp.dx_other)],
      [#dcp.dx_user_name #dcp.dx_entryposition #dcp.dx_licenseno#linebreak()#datetime_th(dcp.dx_datetime)],
    [#label_note([Medication : ],[ยาและวิธีใช้ยาที่ได้รับกลับบ้าน]) #cap_pipe(dcp.meds)
      #list_text(dcp.med_text)],
      [#assess_checks(dcp.med_patient_ok,dcp.med_relatives_ok,dcp.med_other)],
      [#dcp.med_user_name #dcp.med_entryposition #dcp.med_licenseno#linebreak()#datetime_th(dcp.med_datetime)],
    [#label_note([Environment : ],[สภาพแวดล้อมที่ควรจัดเมื่อจำหน่าย]) #cap_pipe(dcp.envs)
      #list_text(dcp.env_text)],
      [#assess_checks(dcp.env_patient_ok,dcp.env_relatives_ok,dcp.env_other)],
      [#dcp.env_user_name #dcp.env_entryposition #dcp.env_licenseno#linebreak()#datetime_th(dcp.env_datetime)],
    [#label_note([Treatment : ],[ข้อควรปฏิบัติ]) #cap_pipe(dcp.txs) 
        - #label_note([อาการเร่งด่วนที่ควรมา รพ. : ],dcp.dx_revisit)
        #list_text(dcp.tx_text)],
      [#assess_checks(dcp.tx_patient_ok,dcp.tx_relatives_ok,dcp.tx_other)],
      [#dcp.tx_user_name #dcp.tx_entryposition #dcp.tx_licenseno#linebreak()#datetime_th(dcp.tx_datetime)],
    [#label_note([Health : ],[การฟื้นฟูสภาพร่างกาย และการป้องกันภาวะแทรกซ้อน])\
        #dcp.dx_prevention
        #list_text(dcp.health_text)],
      [#assess_checks(dcp.health_patient_ok,dcp.health_relatives_ok,dcp.health_other)],
      [#dcp.health_user_name #dcp.health_entryposition #dcp.health_licenseno#linebreak()#datetime_th(dcp.health_datetime)],
    [#text(weight:700,[Out patient Referral :])\
        - #label_note([นัดครั้งต่อไป : ], [#date_th(dcp.appoint_date) #time_th(dcp.appoint_time)])
          #label_note([สถานที่ : ],dcp.appoint_place) #label_note([เพื่อตรวจ : ],dcp.appoint_for)
        - #label_note([ส่งต่อผู้ป่วยไปที่ รพ. : ], dcp.refer_to)
        #list_text(dcp.out_text)],
      [#assess_checks(dcp.out_patient_ok,dcp.out_relatives_ok,dcp.out_other)],
      [#dcp.out_user_name #dcp.out_entryposition #dcp.out_licenseno#linebreak()#datetime_th(dcp.out_datetime)],
    [#label_note([Diet : ],[อาหารที่ควรงดหรือควรรับประทาน]) #cap_pipe(dcp.diets)
      #list_text(dcp.diet_text)],
      [#assess_checks(dcp.diet_patient_ok,dcp.diet_relatives_ok,dcp.diet_other)],
      [#dcp.diet_user_name #dcp.diet_entryposition #dcp.diet_licenseno#linebreak()#datetime_th(dcp.diet_datetime)],
  )
]
#if dc_plans.len() > 0 {
  dc_plans.map(dcp => render(dcp)).join(pagebreak())
}