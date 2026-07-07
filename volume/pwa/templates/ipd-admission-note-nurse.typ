#import "customs/config.typ": code-name
#import "templates/utils.typ": api, get_patient_main, date_th, time_th
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let note = data.at("note", default: none)
#if note == none { note = json(api + "ipd/admission-note-nurse-an/" + data.id) }
// PREPARED FUNCTIONS and VARIABLES
#let checkbox(v,t) = [#if v == t {sym.ballot.cross} else {sym.ballot} #t]
#let checkbox_eq(v,ev,t) = [#if v == ev {sym.ballot.cross} else {sym.ballot} #t]
#let checkbox_if(v,ev,t,f) = if v == ev [#sym.ballot.cross #t] else [#sym.ballot #f]
#let checkbox_toggle(v,ev,t,tf,f) = if v == ev [#sym.ballot #f #sym.ballot.cross #ev #t] else [#sym.ballot.cross #f #sym.ballot #tf]
#let label_note(l,n) = [#text(weight:700,l) #n]
// SET PAGE ATTRIBUTES
#set page(
  margin: (y: 1.5cm, x: 1cm),
  header-ascent: 9pt,
  footer-descent: 5pt,
  header: context[#h(3fr) #text(size:16pt)[*การ​ประเมิน​สภาพ​ผู้ป่วย​แรก​รับ​และ​แบบแผน​สุข​ภาพ​ (ยกเว้น​ผู้ป่วย​เด็ก​อา​ยุ​ < 1 ปี )*] #h(1fr) #code-name\-IPD-NS-ADM #counter(page).display("1/1",both:true)],
  footer: [#label_note([HN: ],pt.hn) #label_note([AN: ], pt.an) #label_note([ชื่อ-สกุล: ], [#pt.pname#pt.fname #pt.lname]) #label_note([อายุ: ], [#pt.age_y ปี]) #label_note([ตึก: ], pt.at("ward_name",default: none)) #label_note([เตียง: ], pt.at("bedno",default: none)) #label_note([แผนก: ], pt.at("spclty_name",default: none)) #label_note([สิทธิ: ], [(#pt.pttype) #pt.pttype_name])]
)
#set text(font: "TH Sarabun New",size: 14pt)
// RENDER
#if note == none {none} else {
  let is_female = {pt.sex == "2"}
  table(
    columns: (100pt,auto,1fr),stroke:.5pt,
    table.cell(colspan:3,fill:luma(222),[#align(center)[*ประวัติผู้ป่วยแรกรับ*]]),
    [*วันที่-เวลาแรกรับ*],table.cell(colspan:2,[#date_th(note.receiver_medication_date) #time_th(note.receiver_medication_time)]),
    [*ผู้ให้ข้อมูล*],table.cell(colspan:2,
    [#checkbox_eq(note.info_patient,"Y","ผู้ป่วย")
    #checkbox_eq(note.info_parent,"Y","บิดา/มารดา")
    #checkbox_eq(note.info_spouse,"Y","สามี/ภรรยา")
    #checkbox_eq(note.info_child,"Y","บุตร")
    #checkbox_eq(note.info_relatives,"Y","ญาติ/ผู้ดูแล")
    #checkbox_eq(note.info_sender,"Y","ผู้นำส่ง")]),
    [*อาการสำคัญ*],table.cell(colspan:2,[#if note.chief_complaints != none {note.chief_complaints.replace(regex("\s")," ")}]),
    [*ประวัติการเจ็บป่วย*],table.cell(colspan:2,[#if note.medical_history != none {note.medical_history.replace(regex("\s")," ")}]),
    [*สัญญาณชีพแรกรับ*],table.cell(colspan:2,[#note.vs_admit]),
    table.cell(colspan:3,fill:luma(222),[#align(center)[*สภาพร่างกายแรกรับ*]]),
    [*ความรู้สึกตัว*],table.cell(colspan:2,
    [#checkbox(note.concious,"รู้สึกตัวดี")
    #checkbox(note.concious,"สับสน")
    #checkbox(note.concious,"ง่วงซึม")
    #checkbox(note.concious,"ไม่รู้สึกตัว")]),
    [*ลักษณะการหายใจ*],table.cell(colspan:2,
    [#checkbox_eq(note.normal_breath,"Y","ปกติ")
    #checkbox_eq(note.kussmaul,"Y","หอบลึก")
    #checkbox_eq(note.tachypnea,"Y","เร็วตื้น")
    #checkbox_eq(note.dyspnea,"Y","ลำบาก")
    #checkbox_eq(note.apnea,"Y","ไม่หายใจ")
    #checkbox_eq(note.tube,"Y","ใส่ท่อช่วยหายใจ")]),
    [*ระบบหัวใจ*],table.cell(colspan:2,
    [#checkbox_eq(note.normal_hr,"Y","ปกติ")
    #checkbox_eq(note.arregular,"Y","อัตราการเต้นไม่สม่ำเสมอ")
    #checkbox_eq(note.weakness,"Y","ชีพจรเบา")
    #checkbox_eq(note.arrhythmia,"Y","ใจสั่น")
    #checkbox_eq(note.chestpain,"Y","เจ็บหน้าอก")
    #checkbox_eq(note.pacemaker,"Y","ใส่เครื่องกระตุ้นหัวใจ")\
    #checkbox_if(note.cardio_other,"Y",[อื่นๆ #note.cardio_other_text],"อื่นๆ")]),
    [*การไหลเวียนโลหิต*],table.cell(colspan:2,
    [#checkbox_eq(note.normal_cir,"Y","ปกติ")
    #checkbox_eq(note.pale,"Y","ซีด")
    #checkbox_eq(note.cyanosis,"Y","เขียวปลายมือ-เท้า")
    #checkbox_eq(note.generalized_edema,"Y","บวมทั่วตัว")
    #checkbox_eq(note.localized_edema,"Y",[บวมเฉพาะที่#note.localized_edema_text])
    #checkbox_eq(note.pitting_edema,"Y","บวมกดบุ๋ม")\
    #checkbox_if(note.circulation_orther,"Y",[อื่นๆ #note.circulation_orther_text],"อื่นๆ")]),
    [*สภาพผิวหนัง*],table.cell(colspan:2,
    [#checkbox_eq(note.normal_skin,"Y","ปกติ")
    #checkbox_eq(note.dry,"Y","แห้งแตก")
    #checkbox_eq(note.bruise,"Y","บาง ช้ำ หลุดลอกง่าย")
    #checkbox_eq(note.erythema,"Y","ผื่นแดง")
    #checkbox_eq(note.abscess,"Y","แผล ฝี")
    #checkbox_eq(note.joudice,"Y","เหลือง")\
    #checkbox_if(note.skin_other,"Y",[อื่นๆ #note.skin_other_text],"อื่นๆ")]),
    [*ความเจ็บปวด*],table.cell(colspan:2, [#checkbox(note.pain,"ไม่มี") #checkbox_if(note.pain,"มี",
    [มี *Pain Score* #note.pain_score คะแนน *บริเวณ* #note.location#linebreak()*ลักษณะการเจ็บปวด*
    #checkbox(note.pain_charac,"ครั้งคราว")
    #checkbox(note.pain_charac,"ตลอดเวลา")
    #checkbox_if(note.pain_charac,"อื่นๆ",[อื่นๆ #note.pain_charac_text],"อื่นๆ")],"มี")]),
    table.cell(colspan:3,fill:luma(222),[#align(center)[*สภาพจิตใจแรกรับ*]]),
    [*การประเมินภาวะจิตใจ*],table.cell(colspan:2,
    if note.no_mental_state == "Y" [#sym.ballot ประเมินได้ #sym.ballot.cross ประเมินไม่ได้เนื่องจาก #note.no_mental_state_text] else [#sym.ballot.cross ประเมินได้ #sym.ballot ประเมินไม่ได้]),
    [*ด้านพฤติกรรม*],table.cell(colspan:2,
    [#checkbox_eq(note.normal_behav,"Y","ร่วมมือดี")
    #checkbox_eq(note.agitate,"Y","กระวนกระวาย")
    #checkbox_eq(note.aggressive,"Y","ก้าวร้าว")
    #checkbox_eq(note.depression,"Y","ซึมเศร้า")
    #checkbox_eq(note.madness,"Y","เอะอะโวยวาย")\
    #checkbox_if(note.behaviour_other,"Y",[อื่นๆ #note.behaviour_other_text],"อื่นๆ")]),
    [*ด้านอารมณ์*],table.cell(colspan:2,
    [#checkbox_eq(note.normal_emotional,"Y","สงบ")
    #checkbox_eq(note.angry,"Y","โกรธ")
    #checkbox_eq(note.moody,"Y","หงุดหงิด")
    #checkbox_eq(note.anxiety,"Y","กังวลใจ")
    #checkbox_eq(note.fear,"Y","หวาดกลัว")\
    #checkbox_if(note.emotional_other,"Y",[อื่นๆ #note.emotional_other_text],"อื่นๆ")]),
    [*ความกังวลใจ*],table.cell(colspan:2,
    [#checkbox_eq(note.no_anxiety,"Y","ปฎิเสธ")\
    #checkbox_eq(note.study,"Y","การเรียน")
    #checkbox_eq(note.family,"Y","ครอบครัว")
    #checkbox_eq(note.economy,"Y","ค่าใช้จ่าย")
    #checkbox_eq(note.habitation,"Y","ที่อยู่อาศัย")
    #checkbox_eq(note.illness,"Y","ความเจ็บป่วย")]),
    [*ความต้องการด้านจิตวิญญาณ*],table.cell(colspan:2,
    [#checkbox_eq(note.spiritual_no,"Y","ไม่ต้องการ")
    #checkbox_eq(note.spiritual_back_home,"Y","บ่นอยากกลับบ้านมาก")
    #checkbox_eq(note.spiritual_need_family,"Y","ถามถึงบุคคลในครอบครัว")
    #checkbox_eq(note.spiritual_other,"Y",[อื่นๆ #note.spiritual_other_text])\
    #checkbox_if(note.spiritual_cant_rated,"Y",[ประเมินไม่ได้ #note.spiritual_cant_rated_text],"ประเมินไม่ได้")]),
    table.cell(colspan:3,fill:luma(222),[#align(center)[*สภาพสังคมและเศรษฐานะ*]]),
    [*การศึกษา*],table.cell(colspan:2,[#checkbox(note.education,"ไม่ได้รับ") #checkbox_eq(note.education,"ได้รับ",[ได้รับ (ระบุ) #note.education_result])]),
    [*อาชีพ*],table.cell(colspan:2,[#note.occupation]),
    [*รายได้*],table.cell(colspan:2,[#checkbox(note.income,"เพียงพอ") #checkbox(note.income,"ไม่เพียงพอ")]),       
    [*ผู้ให้การดูแล\ และช่วยเหลือ*],table.cell(colspan:2,[
    #checkbox_eq(note.self_value,"Y","ตนเอง")
    #checkbox(note.person_family,"บุคคลในครอบครัว")
    #checkbox(note.neighbor,"เพื่อนบ้าน")\
    #checkbox_if(note.assistant_other,"Y",[อื่นๆ #note.assistant_other_text],"อื่นๆ")]),
    [*อาชีพผู้ดูแล*], table.cell(colspan:2,[#note.assistant_occupation]),
    table.cell(colspan:3,fill:luma(222),[#align(center)[*แบบแผนสุขภาพ*]]),
    table.cell(rowspan:2,[*การรับรู้สุขภาพ#linebreak()และการดูแลสุขภาพ*]),
    [*การดูแลตนเอง*],
    [#checkbox_eq(note.clinic,"Y","ไป รพ./คลินิก")
    #checkbox_eq(note.buy_medicine,"Y","ซื้อยารับประทานเอง")],
    [*พฤติกรรมเสี่ยง*],
    [#checkbox_eq(note.no_risk,"Y","ปฏิเสธ")\
    #checkbox_if(note.smoking,"Y",[สูบบุหรี่ #note.smoke_year ปี ปริมาณ #note.smoke_frequency /วัน เลิกเมื่อ #note.smoke_stopped],"สูบบุหรี่")\
    #checkbox_if(note.alcohol,"Y",[ดื่มสุรา #note.alc_year ปี ปริมาณ #note.alc_frequency /วัน เลิกเมื่อ #note.alc_stopped],"ดื่มสุรา")\
    #checkbox_if(note.medication_used,"Y",[ยา (ระบุ) #note.med_name ระยะเวลาที่ใช้ #note.med_year ปี ปริมาณ #note.med_frequency /วัน เลิกเมื่อ #note.med_stopped],"ยา (ระบุ)")],
    table.cell(rowspan:2,[*อาหารและ\ การเผาผลาญอาหาร*]),
    [*ประเภทอาหาร*],
    [#checkbox(note.diet_regular,"อาหารทั่วไป")
    #checkbox_if(note.diet_regular,"อาหารเฉพาะโรค",[อาหารเฉพาะโรค (ระบุ) #note.diet_spec],"อาหารเฉพาะโรค")],
    [*ปัญหาการ\ รับประทานอาหาร*],
    [#checkbox_eq(note.nutrition_risk,"Y","ไม่มี")
    #checkbox_eq(note.loss_appetite,"Y","เบื่ออาหาร")
    #checkbox_eq(note.dysphagia,"Y","เคี้ยว/กลืนลำบาก")
    #checkbox_eq(note.loss_gustation,"Y","ไม่รู้รสกลิ่น")
    #checkbox_eq(note.denture,"Y","ใส่ฟันปลอม")\
    #checkbox_if(note.nutrition_risk_other,"Y",[อื่นๆ (ระบุ) #note.nutrition_risk_other_text],"อื่นๆ")],
    table.cell(rowspan:2,[*การขับถ่าย*]),
    [*ปัสสาวะ*],
    [#checkbox_eq(note.normal_urine,"Y","ปกติ")
    #checkbox_eq(note.dysuria,"Y","แสบขัด")
    #checkbox_eq(note.incontinence,"Y","กลั้นไม่ได้")
    #checkbox_eq(note.staining,"Y","ลำบาก")
    #checkbox_eq(note.hematuria,"Y","เป็นเลือด")
    #checkbox_eq(note.catheter,"Y","สายสวนปัสสาวะ")],
    [*อุจจาระ*],
    [#checkbox_eq(note.normal_feces,"Y","ปกติ")
    #checkbox_eq(note.constipation,"Y","ท้องผูก")
    #checkbox_eq(note.diarrhea,"Y","ท้องเสีย")
    #checkbox_eq(note.bowel_incontinence,"Y","กลั้นไม่ได้")
    #checkbox_eq(note.hemorrhoid,"Y","ริดสีดวงทวาร")
    #checkbox_eq(note.colostomy,"Y","ถ่ายทางหน้าท้อง")],
    [*กิจกรรมและ\ ออกกำลังกาย*],
    table.cell(colspan:2,
    [#checkbox_eq(note.activity1,"Y","ทำได้เอง")
    #checkbox_eq(note.activity2,"Y","ต้องมีคนช่วย")
    #checkbox_eq(note.activity3,"Y","ทำเองไม่ได้")
    #checkbox_if(note.activity4,"Y",[ใช้กายอุปกรณ์ #note.o_p_use],"ใช้กายอุปกรณ์")]),
    table.cell(rowspan:2,[*การพักผ่อนนอนหลับ*]),
    [*การนอนปกติ*],
    [#checkbox_if(note.sleep_per_day,"Y",[วันละ #note.sleep_hour ชั่วโมง],"วันละ  ชั่วโมง")\
    #checkbox_if(note.sleep_problems,"Y",[ปัญหาการนอน (ระบุ) #note.sleep_problems_detail],"ปัญหาการนอน")],
    [*การใช้ยานอนหลับ*],
    [#checkbox(note.sleep_med_name,"ไม่เคย")
    #checkbox(note.sleep_med_name,"เป็นครั้งคราว")
    #checkbox_if(note.sleep_med_name,"เป็นประจำ",[เป็นประจำ ยา (ระบุ) #note.sleep_med_name_detail],"เป็นประจำ")],
    table.cell(rowspan:5,[*สติปัญญาและการรับรู้*]),
    [*การรับรู้*],
    [#checkbox(note.cognitive,"ตรง")
    #checkbox(note.cognitive,"ไม่ตรง")],
    [*ความจำ*],
    [#checkbox(note.memory,"ปกติ")
    #checkbox_if(note.memory,"ผิดปกติ",[ผิดปกติ (ระบุ) #note.memory_detail],"ผิดปกติ")],
    [*การได้ยิน*],
    [#checkbox(note.hearing,"ปกติ")
    #checkbox_if(note.hearing,"ผิดปกติ",[ผิดปกติ (ระบุ) #note.hearing_detail],"ผิดปกติ")\
    #checkbox_eq(note.eartone,"Y","ใช้เครื่องช่วยฟัง")],
    [*การมองเห็น*],
    [#checkbox(note.vision,"ปกติ")
    #checkbox_if(note.vision,"ผิดปกติ",[ผิดปกติ (ระบุ) #note.vision_detail],"ผิดปกติ")\
    #checkbox_eq(note.vision_eyeglasses,"Y","แว่นตา")
    #checkbox_eq(note.vision_contactlens,"Y","คอนแทคเลนส์")],
    [*การพูด*],
    [#checkbox(note.speech,"ปกติ")
    #checkbox_if(note.speech,"ผิดปกติ",[ผิดปกติ (ระบุ) #note.speech_detail],"ผิดปกติ")],
    table.cell(rowspan:2,[*การรับรู้ตนเอง\ และอัตมโนทัศน์*]),
    [*กระทบต่อภาพลักษณ์*],
    checkbox_toggle(note.self_image,"มี",[#note.self_image_detail],"มี","ไม่มี"),
    [*กระทบต่อความสามารถ*],
    checkbox_toggle(note.self_activity,"มี",[#note.self_activity_detail],"มี","ไม่มี"),
    [*บทบาทและสัมพันธภาพ*],
    [*ผลกระทบจาก\ ความเจ็บป่วย*],
    [#checkbox_toggle(note.sickness_effect,"มีผลกระทบต่อ",
      [#checkbox_eq(note.sickness_family,"Y","ครอบครัว")
      #checkbox_eq(note.sickness_occupation,"Y","อาชีพ")
      #checkbox_eq(note.sickness_education,"Y","การศึกษา")\
      #checkbox_if(note.sickness_other,"Y",[อื่นๆ #note.sickness_other_text],"อื่นๆ")],"มีผลกระทบ","ไม่มีผลกระทบ")],
    table.cell(rowspan:2,[*เพศและการเจริญพันธุ์*]),
    [*ประจำเดือน*],
    [#if is_female [
      #checkbox(note.period,"ยังไม่มี") #checkbox_if(note.period,"มี",[มี
      #checkbox(note.period_normal,"ปกติ")
      #checkbox_eq(note.period_normal,"ผิดปกติ",[ผิดปกติ(ระบุ) #note.period_disorders])
      *LMP* #note.period_lmp],"มี")\
      #checkbox_if(note.period,"หมดประจำเดือน",[หมดประจำเดือน เมื่ออายุ #note.period_menopause ปี],"หมดประจำเดือน")]],
    [*เต้านม*],
    [#checkbox_toggle(note.breast,"ผิดปกติ",[(ระบุ) #note.breast_disorders],"ผิดปกติ","ปกติ")],
    [*การปรับตัวต่อ\ ความเครียด*],
    [*วิธีการแก้ไขปัญหา*],
    [#checkbox_eq(note.consult,"Y","ปรึกษา")
    #checkbox_eq(note.seclude,"Y","แยกตัว")
    #checkbox_eq(note.medication,"Y",[ใช้ยา#note.medication_detail])
    #checkbox_eq(note.religion,"Y","ศาสนา")\
    #checkbox_if(note.coping_stress_other,"Y",[อื่นๆ #note.coping_stress_other_detail],"อื่นๆ")],
    table.cell(rowspan:3,[*คุณค่าและความเชื่อ*]),
    [*เชื่อว่าการเจ็บป่วยครั้งนี้\ มีสาเหตุจาก*],
    [#checkbox_eq(note.belief_sickness_behave,"Y","ปฏิบัติตัวไม่ถูกต้อง")
    #checkbox_eq(note.belief_sickness_age,"Y","ตามวัย")
    #checkbox_eq(note.belief_sickness_destiny,"Y","เคราะห์กรรม")\
    #checkbox_if(note.belief_sickness_other,"Y",[อื่นๆ #note.belief_sickness_other_text],"อื่นๆ")],
    [*สิ่งยึดเหนี่ยวด้านจิตใจ*], checkbox_toggle(note.belief_believe,"มี","","มี","ไม่มี"),
    [*ต้องการปฏิบัติกิจกรรม\ ทางศาสนา*], checkbox_toggle(note.religious_activity,"ต้องการ",[#note.religious_activity_text],"ต้องการ","ไม่ต้องการ"),
    table.cell(fill:luma(222),[*ข้อมูลที่ให้ขณะแรกรับ*]),
    table.cell(colspan:2,fill:luma(222),
    [โรค​และ​อาการ​ปัจจุบัน​, แพทย์ผู้ดูแล, แนวทาง​การ​รักษา​พยาบาล​, สิทธิ​การ​รักษา​, การ​ลง​นาม​ยิน​ยอม, อาคาร​สถาน​ที่​ \
    การ​ปฏิบัติ​ตัว​ขณะ​เข้า​รับ​การ​รักษา​, กฎ​ระเบียบ​การ​เยี่ยม, การ​ติดต่อ​สอบ​ถาม]),
    [*ผู้ประเมิน*],table.cell(colspan:2,[#note.nurse_name #note.nurse_pos #note.nurse_licenseno]),
  )
}