#import "customs/config.typ": hospital-name
#import "templates/utils.typ": api, vnan_is_ipd, get_patient_main, date_th, time_th, datetime_th, cid
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let is_ipd = vnan_is_ipd(data.id)
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#assert(pt != none, message:"no 'patient' in data")
#let summary = data.at("summary", default: none)
#let summ = {}
#if summary == none {
  summ = json(api + "ipd/summary?an=" + data.id)
  summary = summ.summary
}
#let dx = data.at("dx", default: none)
#if dx == none { dx = summ.dx_data }
#let doctor = data.at("doctor", default: none)
#if doctor == none { doctor = summ.doctor_data }
// PREPARED FUNCTIONS
#let bold(x, is_black) = {
  let f = if is_black {black} else {rgb("#3d9970")}
  let s = if x != none and type(x) == int {str(x)} else {x}
  text(fill:f, size:14pt, weight:700, top-edge:0.4em, s)
}
#let input(x) = text(fill:black, weight:700, x)
#let check(v) = if v != none and v == "Y" [#text(fill:black, [#sym.checkmark])] else [#sym.space.quad]
#let check_eq(v,s) = if v != none and v == s [#text(fill:black, [#sym.checkmark])] else [#sym.space.quad]
#let check_not_none(v) = if v != none [#text(fill:black, [#sym.checkmark])] else [#sym.space.quad]
#let diags(dx, ty) = {
  let items = dx.filter(d => d.ty == ty)
  if items.len() > 0 {
    items.enumerate(start:1).map(((i,d)) => {
      input([#str(i). #d.detail])
    }).join(linebreak())
  } else {linebreak()}
}
#let doctors(dr, ty) = {
  v(5pt) + text(fill:black, dr.filter(d => d.ty == ty).map(d => [
    #linebreak()#d.doctor_name #d.licenseno
  ]).join(linebreak()))
}
#let signatures(dr, ty) = {
  v(12pt) + dr.filter(d => d.ty == ty).map(d => text(size: 12pt, top-edge:0.2em, repeat[.])).join()
}
//RENDER
#set page(margin: 1cm)
#set text(fill: olive, top-edge: 0.2em)
#place(top + center, image(width: 60pt, "statics/picture/krut-green.svg"))
#grid(
  columns: (1fr,)*2,
  rows: (63pt),
  align: (left + bottom, right + bottom),
  bold([MINISTRY OF PUBLIC HEALTH#linebreak()THAILAND],false),
  bold([ฉบับปรับปรุง มิถุนายน 2556\
  #hospital-name\
  IN PATIENT SUMMARY],false)
)
#v(5pt, weak: true)
#if summary == none {none} else {
  table(
    columns: (1fr,)*6 + (2fr,)*2 + (3fr,)*3,
    stroke: .5pt + olive,
    inset: (top:7pt, rest:4pt),
    // line 1
    table.cell(colspan:11, [สิทธิการรักษา : #input(pt.pttype_name)]),
    // line 2
    table.cell(colspan:6, [1.\u{00A0}ADMISSION NUMBER\
    #bold(summary.an,true)]),
    table.cell(colspan:4, [2.\u{00A0}PERSONAL IDENTIFICATION NUMBER OR PASSPORT NUMBER\
    #bold([#cid(pt.cid) #pt.passport_no],true)]),
    [3.\u{00A0}HOSPITAL NUMBER\
    #bold(pt.hn,true)],
    // line 3
    table.cell(colspan:7, [4.\u{00A0}PATIENT NAME\
    #bold([#{pt.pname + pt.fname} #pt.lname],true)]),
    table.cell(colspan:4, [5.\u{00A0}PATIENT ADDRESS #if pt.hometel != none and pt.hometel.len() > 0 [, TEL: #input(pt.hometel)]\
    #bold(pt.homeaddr,true)]),
    // line 4
    table.cell(colspan:3, [6.\u{00A0}SEX\
    #bold(pt.sex_name,true)]),
    table.cell(colspan:3, [7.\u{00A0}MARITAL STATUS\
    #bold(pt.marrystatus_name,true)]),
    table.cell(colspan:2, [8.\u{00A0}ETHNIC GROUP\
    #bold(pt.citizenship_name,true)]),
    table.cell(colspan:3, [9.\u{00A0}OCCUPATION\
    #bold(pt.occupation_name,true)]),
    // line 5
    table.cell(colspan:3, [10.\u{00A0}DATE OF BIRTH\
    #bold(date_th(pt.birthday),true)]),
    table.cell(colspan:4, [11.\u{00A0}AGE AT ADMISSION\
    #bold(pt.age_y,true) YEARS #bold(pt.age_m,true) MONTHS #bold(pt.age_d,true) DAYS]),
    table.cell(colspan:2, [12.\u{00A0}BIRTHWEIGHT (INFANT ONLY)\
    #if pt.at("birth_weight",default: none) != none [#bold(pt.birth_weight,true) GRAMS]]),
    [13.\u{00A0}LENGTH OF STAY\
    #if pt.at("admdate",default: none) != none [#bold(pt.admdate,true) DAYS]],
    [14.\u{00A0}TOTAL LEAVE DAYS\
    #if pt.at("leave_home_day",default: none) != none [#bold(pt.leave_home_day,true) DAYS]],
    // line 6
    table.cell(colspan:4, rowspan:3, [15.\u{00A0}WARD\
    #bold(pt.at("ward_name",default: none),true)]),
    table.cell(colspan:3, rowspan:3, [16.\u{00A0}DEPARTMENT\
    #bold(pt.at("spclty_name",default: none),true)]),
    [17.\u{00A0}DATE OF],[DAY-MONTH-YEAR],[TIME],table.cell(rowspan:3, [ICD CODING BY CODER\
    #input(summary.coder_name)]),
    [ADMISSION],if is_ipd [#input(date_th(pt.regdate))] else [],if is_ipd [#input(time_th(pt.regtime))] else [],
    [DISCHARGE],[#input(date_th(pt.at("dchdate",default: none)))],[#input(time_th(pt.at("dchtime",default: none)))],
    // diagnosis (1)-(5)
    table.cell(rowspan:5, align:center + horizon, rotate(270deg, reflow:true, [18.\u{00A0}DIAGNOSIS])),
    table.cell(colspan:9,[(1) PRINCIPLE DIAGNOSIS บันทึกได้เพียงโรคเดียวเท่านั้น\
    #if summary.principal_diagnosis != none {input(summary.principal_diagnosis)} else {linebreak()}]),[MAIN (มีได้รหัสเดียว)\
    #input(summary.principal_diagnosis_code)],
    table.cell(colspan:9,[(2) PRE ADMISSION COMORBIDITY (S)\
    #diags(dx, 2)]),[COMORBIDITY (S)\
    #input(summary.pre_admission_comorbidity_codes)],
    table.cell(colspan:9,[(3) COMPLICATION (S) (POST ADMISSION COMORBIDITY)\
    #diags(dx, 3)]),[COMPLICATION (S)\
    #input(summary.post_admission_comorbidity_codes)],
    table.cell(colspan:9,[(4) OTHER DIAGNOSIS\
    #diags(dx, 4)]),[OTHER (S)\
    #input(summary.other_diagnosis_codes)],
    table.cell(colspan:9,[(5) EXTERNAL CAUSE (S) OF INJURY\
    #diags(dx, 5)]),[EXTERNAL CAUSE (S)\
    #input(summary.external_cause_codes)],
    // operation
    table.cell(align:center + horizon, [19]),
    table.cell(colspan:9,[OPERATION ROOM PROCEDURES\
    #if summary.operating_room != none {input(summary.operating_room)} else {linebreak()}]),
    [PROCEDURES CODING#linebreak()MAIN (มีได้รหัสเดียว)\
    #input(summary.main_procedure_code)],
    // non operation
    table.cell(align:center + horizon, [20]),
    table.cell(colspan:9,[IMPORTANT NON OPERATING ROOM PROCEDURES\
    1.\u{00A0}(#check(summary.tracheostomy)) TRACHEOSTOMY\
    2.\u{00A0}(#check_not_none(summary.mechanical_ventilation)) MECHANICAL VENTILATION \u{00A0} (#check_eq(summary.mechanical_ventilation, "1")) INVASIVE มากกว่า 96 ชม. \u{00A0} (#check_eq(summary.mechanical_ventilation, "2")) INVASIVE น้อยกว่า 96 ชม. \u{00A0} (#check_eq(summary.mechanical_ventilation, "3")) NON-INVASIVE\
    3.\u{00A0}(#check(summary.packed_redcells)) PACKED RED CELLS \u{00A0} (#check(summary.fresh_frozen_plasma)) FRESH FROZEN PLASMA \u{00A0} (#check(summary.platelets)) PLATELETS  \u{00A0} (#check(summary.cryoprecipitate)) CRYOPRECIPITATE \u{00A0} (#check(summary.whole_blood)) WHOLE BLOOD\
    4.\u{00A0}(#check(summary.chemotherapy)) CHEMOTHERAPY\
    5.\u{00A0}(#check(summary.hemodialysis)) HEMODIALYSIS\
    6.\u{00A0}(#check(summary.non_or_other)) OTHERS #input(summary.non_or_other_text)]),
    [OTHER (S)#linebreak()#input(summary.other_procedure_codes)],
    // special investigation
    table.cell(align:center + horizon, [21]),
    table.cell(colspan:10,[SPECIAL INVESTIGATIONS\
    1.\u{00A0}(#check(summary.computer_tomography)) COMPUTERIZED TOMOGRAPHY #input(summary.computer_tomography_text)\
    2.\u{00A0}(#check(summary.mri)) MRI #input(summary.mri_text)\
    3.\u{00A0}(#check(summary.special_other)) OTHERS #input(summary.special_other_text)]),
    // discharge
    table.cell(colspan:8, [22.\u{00A0}DISCHARGE STATUS]),
    table.cell(colspan:3, [23.\u{00A0}DISCHARGE TYPE]),
    table.cell(colspan:8, grid(columns: (2fr,3fr),[
    1.\u{00A0}(#check_eq(summary.discharge_status, "01")) COMPLETE RECOVERED\
    2.\u{00A0}(#check_eq(summary.discharge_status, "02")) IMPROVED\
    3.\u{00A0}(#check_eq(summary.discharge_status, "03")) NOT IMPROVED\
    4.\u{00A0}(#check_eq(summary.discharge_status, "04")) DELIVERED\
    5.\u{00A0}(#check_eq(summary.discharge_status, "05")) UNDELIVERED],[
    6.\u{00A0}(#check_eq(summary.discharge_status, "06")) NORMAL CHILD DISCHARGE WITH MOTHER\
    7.\u{00A0}(#check_eq(summary.discharge_status, "07")) NORMAL CHILD DISCHARGE SEPERATELY\
    8.\u{00A0}(#check_eq(summary.discharge_status, "08")) DEAD])),
    table.cell(colspan:3, grid(columns: (1fr,1fr),[
    1.\u{00A0}(#check_eq(summary.discharge_type, "01")) WITH APPROVAL\
    2.\u{00A0}(#check_eq(summary.discharge_type, "02")) AGAINST ADVICE\
    3.\u{00A0}(#check_eq(summary.discharge_type, "03")) ESCAPE\
    4.\u{00A0}(#check_eq(summary.discharge_type, "04")) BY TRANSFER],[
    5.\u{00A0}(#check_eq(summary.discharge_type, "05")) OTHER\
    6.\u{00A0}(#check_eq(summary.discharge_type, "06")) DEAD, AUTOPSY\
    7.\u{00A0}(#check_eq(summary.discharge_type, "07")) DEAD, NO AUTOPSY],
    grid.cell(colspan:2, inset: (top:10pt, left:5pt), [ชื่อสถานพยาบาลที่ส่งต่อ #input(summary.hospname)]))),
    table.cell(colspan:11, align:center, [IN CASE OF STILLBIRTHS OR INFANT DYING WITHIN 1 WEEK FROM BIRTH. PLEASE COMPLETE THE PERINATAL DEATH CERTIFICATE ON OTHER SIDE OF FORM])
  )
}
// signature
#if summary == none {none} else {
  grid(
    columns: (1fr,90pt,1fr,90pt),
    gutter: 3pt,
    inset: (x:20pt),
    [ATTENDING PHYSICIAN],[SIGNATURE],[APPROVED BY],[SIGNATURE],
    doctors(doctor, 1),signatures(doctor, 1),
    doctors(doctor, 2),signatures(doctor, 2)
  )
}