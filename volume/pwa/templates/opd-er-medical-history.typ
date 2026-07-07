#import "@preview/oxifmt:1.0.0": strfmt
#import "templates/utils.typ": api, get_patient_main, date_th, time_th, explode_imgs
#import "templates/scores.typ"
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let hline = [#v(-15pt)#box(line(length:100%,stroke:.5pt))]
#let border_box(c) = rect(width:100%,outset:(x:1pt,y:4pt),stroke:.5pt,radius:9pt,c)
#let card_box(h,c) = [#border_box(align(center,text(size:20pt,weight:700,h)))#c]
#let label_note(l,n) = [#text(weight:700,l) #n]
#let pe_note(c,t) = [#if c == "Y" [#sym.ballot ปกติ #sym.ballot.check ผิดปกติ] else [#sym.ballot.check ปกติ #sym.ballot ผิดปกติ] : #t]
#let ind_box(c) = box(inset:(left:10pt),c)
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none {pt = get_patient_main(data.id)}
#let visit_datetime = if pt.vstdate != none and pt.vsttime != none {pt.vstdate + "T" + pt.vsttime.split(".").at(0)}
#let med_data = data.at("med",default: none)
#let trauma_data = data.at("trauma",default: none)
#let allergy_data = data.at("allergy",default: none)
#let screen_data = data.at("screen",default: none)
#let consult_data = data.at("consult",default: none)
#let ft_data = data.at("ft",default: none)
#if pt.opd_er_order_master_id != none {
  let params = "?opd_er_order_master_id=" + str(pt.opd_er_order_master_id) + "&hn=" + pt.hn + "&vn=" + data.id + "&visit_datetime=" + visit_datetime
  if med_data == none {med_data = json(api + "opd-er/medical-history" + params)}
  if trauma_data == none {trauma_data = json(api + "opd-er/medical-history-trauma" + params)}
  if allergy_data == none {allergy_data = json(api + "opd-er/medical-history-allergy" + params)}
  if screen_data == none {screen_data = json(api + "opd-er/medical-history-screen" + params)}
  if consult_data == none {consult_data = json(api + "opd-er/medical-history-consult" + params)}
  if ft_data == none {ft_data = json(api + "opd-er/medical-history-ft" + params)}
}
#if med_data == none {none} else {
  let opdscreen = med_data.opdscreen
  // PREPARED VARIABLES
  let trauma_section = if trauma_data != none [#card_box([Triage],[
    #strong[A: Airway & restriction c-spine]\
    #ind_box[#if trauma_data.arc == "1" [#sym.ballot.check] else [#sym.ballot] Patent\
      #if trauma_data.arc == "2" [#sym.ballot.check] else [#sym.ballot] C-spine protection\
      #if trauma_data.arc == "3" [#sym.ballot.check] else [#sym.ballot] Non-patent : Clinical #trauma_data.arc_npc_text]\
    #strong[B: Breathing]\
    #ind_box[- Chest wall : #trauma_data.breathing_chest_wall
    - Lung : #trauma_data.breathing_lung]\
    #strong[C: Circulation]\
    #ind_box[#if trauma_data.circulation_shock == "2" [#sym.ballot Stable #sym.ballot.check Shock  : #trauma_data.circulation_shock_text] else [#sym.ballot.check Stable #sym.ballot Shock]
      #if trauma_data.circulation_other == "Y" [#sym.ballot.check อื่นๆ : #trauma_data.circulation_other_text] else [#sym.ballot อื่นๆ]\
      #if trauma_data.circulation_efast_date == none [#sym.ballot eFAST] else [#sym.ballot.check eFAST : #date_th(trauma_data.circulation_efast_date) #time_th(trauma_data.circulation_efast_time)]\
      #ind_box[โดย #trauma_data.circulation_doctor_name]\
      #ind_box[#if trauma_data.circulation == "P" [#sym.ballot Negative #sym.ballot.check Positive at #trauma_data.circulation_positive_text] else [#sym.ballot.check Negative #sym.ballot Positive]]]\
    #strong[D: Disability]\
    #ind_box[- E #trauma_data.disability_e V #trauma_data.disability_v M #trauma_data.disability_m Pupil Rt #trauma_data.disability_pupil_rt mm Lt #trauma_data.disability_pupil_lt mm\
      - อื่นๆ : #trauma_data.disability_other]\
    #strong[E: Exposure] #trauma_data.exposure\
    #label_note([แพทย์ผู้ตรวจ : ],trauma_data.doctor_name)
  ])]
  let hx_section = [#card_box([History],[
    #label_note([CC : ],if opdscreen != none [#opdscreen.cc] else [])\
    #label_note([PI : ],if opdscreen != none [#opdscreen.hpi] else [])\
    #label_note([ประวัติการใช้ยา : ],med_data.hosxp_drug_history.join(linebreak()))\
    #label_note([ประวัติการผ่าตัด : ],med_data.hosxp_operation_history.join(linebreak()))\
    #label_note([ประวัติการแพ้ยา : ],[#med_data.hosxp_drugallergy.join(", ")])\
    #allergy_data.map(ah => [#h(10pt)- #ah.er_allergy_history_agent: #ah.er_allergy_history_symptom]).join()
  ])]
  let pe_section = [#card_box([Physical Examination],[
    #if med_data.vs_kphis != none [
      #let vs = med_data.vs_kphis
      #let ews_s = scores.score("ews",vs.ews_concat,pt.birthday)
      #let qsofa_s = scores.score("qsofa",vs.ews_concat,pt.birthday)
      #let sirs_s = scores.score("sirs",vs.ews_concat,pt.birthday)
      #label_note([น้ำหนัก : ],strfmt("{:.2} kg.",vs.bw)) #label_note([ส่วนสูง : ],[#vs.height cm.]) #if vs.pain != none [#label_note([Pain scales : ],[#vs.pain คะแนน])]\
      #label_note([BP : ],[#vs.sbp/#vs.dbp mmHg]) #label_note([P : ],[#vs.pr /min]) #label_note([R : ],[#vs.rr /min]) #label_note([T : ],[#vs.bt \u{0e4d}C])\
      #label_note([SpO#sub("2") : ],[#vs.sat %])\
      #label_note([Coma scale : ],[E#sub[#vs.eye]V#sub[#vs.verbal]M#sub[#vs.movement]]) #label_note([SpO#sub("2") : ],[#vs.sat %])
      #label_note([Pupil : ],[Rt #vs.right_pupil Lt #vs.left_pupil])\
      #if ews_s.score != none [#label_note([#ews_s.label : ],ews_s.score)]
      #if qsofa_s.score != none [#label_note([#qsofa_s.label : ],qsofa_s.score)]
      #if sirs_s.score != none [#label_note([#sirs_s.label : ],sirs_s.score)]\
    ]
    #if opdscreen != none [#label_note([GA : ],opdscreen.pe_ga_text)\
      #grid(columns:(40pt,1fr),gutter:15pt,
        strong[HEENT],pe_note(opdscreen.pe_heent,opdscreen.pe_heent_text),
        strong[HEART],pe_note(opdscreen.pe_heart,opdscreen.pe_heart_text),
        strong[LUNG],pe_note(opdscreen.pe_lung,opdscreen.pe_lung_text),
        strong[ABDOMEN],pe_note(opdscreen.pe_ab,opdscreen.pe_ab_text),
        strong[EXT],pe_note(opdscreen.pe_ext,opdscreen.pe_ext_text),
        strong[NEURO],pe_note(opdscreen.pe_neuro,opdscreen.pe_neuro_text),
      )
      #if opdscreen.pe != none [#label_note([PE TEXT : ],opdscreen.pe)]]
  ])]
  let diag_section = if med_data.hosxp_diagnosis.len() > 0 [#card_box([Diagnosis],[
    #med_data.hosxp_diagnosis.map(dx => [- #dx]).join()
  ])]
  let consult_section = if consult_data.len() > 0 [#card_box([Consults],[
    #box(list(..consult_data.map(c => [
      #label_note([แผนก : ],c.er_consult_ward_name) #label_note([เวลา : ],[#date_th(c.er_consult_date) #time_th(c.er_consult_time)])\
      #label_note([แพทย์เวรที่มาดู : ],c.doctor_name) #label_note([เวลา : ],[#date_th(c.er_consult_date_reply) #time_th(c.er_consult_time_reply)])
    ])))
  ])]
  let set_ft_section = if ft_data != none [#card_box([Set FT],[
    #label_note([ประสานห้องผ่าตัด Set FT : ],[#date_th(ft_data.set_ft_date) #time_th(ft_data.set_ft_time)])
  ])]
  let times = if screen_data != none {(
    label_note([เวลาที่มาถึง ER : ],[#date_th(screen_data.screening_arrive_date) #time_th(screen_data.screening_arrive_time)]),
    label_note([เวลาที่คัดกรอง : ],[#date_th(screen_data.screening_date) #time_th(screen_data.screening_time)]),
    label_note([เวลาที่รายงาน : ],[#date_th(screen_data.screening_report_date) #time_th(screen_data.screening_report_time)]),
    label_note([เวลาที่พบแพทย์ : ],[#date_th(screen_data.screening_see_doctor_date) #time_th(screen_data.screening_see_doctor_time)]),
    label_note([ผู้คัดกรอง],screen_data.nurse_name)
  )}
  // RENDER
  set text(font:"TH Sarabun New",size:14pt)
  set page(paper:"a4",margin:1cm)
  [#section[ประวัติผู้ป่วย Emergency Room]
  #border_box[
    #label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([VN : ],pt.vn)\
    #label_note([สิทธิการรักษา : ],pt.pttype_name)
    #grid(columns:(1fr,1fr),gutter:10pt,
      label_note([ประเภทผู้ป่วย : ],if opdscreen != none [#opdscreen.er_pt_type_name] else []),
      label_note([ความเร่งด่วน : ],if opdscreen != none [#opdscreen.er_emergency_type_name] else []),
      label_note([ระดับความฉุกเฉิน : ],if opdscreen != none [#opdscreen.er_emergency_level_name] else []),
      label_note([ประเภททางคลินิก : ],if opdscreen != none [#opdscreen.er_spclty_name] else []),
    ..times)
  ]
  #columns(2,gutter:15pt,[
    #trauma_section #hx_section #pe_section #diag_section #consult_section #set_ft_section,
    #if trauma_data != none and trauma_data.imgs != none {card_box([Images],explode_imgs(3,false,trauma_data.imgs))}
  ])]
}