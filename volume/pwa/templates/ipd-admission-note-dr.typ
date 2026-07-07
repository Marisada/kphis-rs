#import "customs/config.typ": code-name, hospital-name
#import "templates/utils.typ": api, get_patient_main, date_th, time_th, datetime_th_checked, timestamp_th, cid, explode_imgs
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#assert(pt.hn != none, message:"no 'hn' in patient")
#let patient_image = data.at("patient_image", default: none)
#if patient_image == none { patient_image = "/img/patient/" + pt.hn }
#let raw = data.at("raw", default: none)
#if raw == none { raw = json(api + "ipd/admission-note-dr-an/" + data.id) }
// PREPARED FUNCTIONS
#let cap_pipe_eq(t) = if t == none {none} else {
  t.replace("^","=").replace("|",", ")
}
#let cap_dd(t) = if t == none {none} else {
  t.split("|").map(s => {
    let ar = s.split("^");
    if ar.len() == 3 [โรค #ar.at(0) เป็นมา #ar.at(1) ปี รักษาที่ #ar.at(2)] else []
  }).join(", ")
}
#let cap_fm(t) = if t == none {none} else {
  t.split("|").map(s => {
    let ar = s.split("^");
    if ar.len() == 2 [#ar.at(1)เป็นโรค #ar.at(0)] else []
  }).join(", ")
}
#let cap_ad(t) = if t == none {none} else {
  t.split("|").map(s => {
    let ar = s.split("^");
    if ar.len() == 2 [#ar.at(0) ประเมินผลกระทบได้ #ar.at(1).split(",").first() คะแนน] else []
  }).join(", ")
}
// SET PAGE ATTRIBUTES
#set page(margin: (y:2cm, x:1cm),header-ascent: 15pt,footer-descent: 0pt,
  header: context[#h(3fr) #text(size:18pt)[*แบบบันทึกการรับใหม่ผู้ป่วยใน #hospital-name*] #h(2fr) #code-name\-IPD-DR-ADM #counter(page).display("1/1",both:true)],
  footer: [
    #table(stroke:none,columns:(1fr,auto,auto,auto), [*ชื่อ - สกุล* : #pt.pname#pt.fname #pt.lname],[*อายุ* : #pt.age_y ปี #pt.age_m เดือน #pt.age_d วัน],[*HN* : #pt.hn],[*AN* : #pt.an],)#v(-15pt)
    #table(stroke:none,columns: 4,[*ตึก* : #pt.at("ward_name",default: none)],[*เตียง* : #pt.at("bedno",default:none)],[*แผนก* : #pt.at("spclty_name",default: none)],[*สิทธิ* : (#pt.pttype) #pt.pttype_name])]
)
#set table(stroke:.5pt)
#set text(font:"TH Sarabun New",size:14pt,baseline:2pt,top-edge:"x-height")
// RENDER
#if raw.admission_note == none {none} else {
  let note = raw.admission_note
  let docs = raw.admission_note_doctors
  let vs = raw.vs
  let pr = raw.period
  let pe = raw.opdscreen_pe
  let pe_bw = if pe == none {none} else {pe.bw}
  let pe_ht = if pe == none {none} else {pe.height}
  let is_female = pt.sex == "2"
  let is_puberty = {is_female and pt.age_y > 8}
  table(
    columns: (1fr,) * 6,
    rows: (auto,16pt,auto,16pt,auto,16pt,auto,auto),
    if patient_image != none {image(width:80pt, patient_image)} else {image(width:80pt, "statics/picture/user.svg")},
    table.cell(colspan:5,[#v(5pt)
      *วันที่รับไว้รักษา* : #date_th(note.receiver_medication_date) เวลา : #time_th(note.receiver_medication_time) *เข้ารับการรักษาโดย* : #note.take_medication_by *มาถึงหอผู้ป่วยโดย* : #note.arrive_by \
      *นำส่งผู้ป่วยโดย* :
        #if note.taken_by_relative == "Y" [ญาติ]
        #if note.taken_by_crib == "Y" [พนักงานเปล]
        #if note.taken_by_nurse == "Y" [พยาบาล]
        #if note.taken_by_etc == "Y" [#note.taken_by]
      *ผู้ให้ข้อมูล* : #note.informant_patient #note.informant_relatives #note.informant_deliverer #note.informant_etc \
      *อาการสำคัญ* : #note.chief_complaints \
      *สัญญาณชีพแรกรับ* : *T* #note.t \u{0e4d}C *PR* #note.pr /min *RR* #note.rr /min *BP* #note.bp mmHg *GCS* #note.gcs คะแนน (E#note.e;V#note.v;M#note.m) \
      #if pe_bw != none [*น้ำหนัก* : #pe_bw Kg. ] else if pt.latest_bw != none [*น้ำหนัก* : #if decimal(pt.latest_bw) > decimal(5) {calc.round(decimal(pt.latest_bw),digits:1)} else {pt.latest_bw} Kg. ]
      #if pe_ht != none [*ส่วนสูง* : #pe_ht cm. ] else if pt.latest_height != none [*ส่วนสูง* : #pt.latest_height cm. ]
      #if note.braden_scale != none [*Braden scale* #note.braden_scale.split(",").first() คะแนน]
      #if pt.latest_bw != none or pt.latest_height != none or note.braden_scale != none {linebreak()}
      #if pt.drugallergy != none or pt.er_drugallergy_history != none or note.allergy_drug_history != none or note.allergy_food_history != none or note.allergy_etc_history != none [
        #text(red)[*ประวัติการแพ้* :
          *#pt.drugallergy*
          *#pt.er_drugallergy_history*
          *#cap_pipe_eq(note.allergy_drug_history)*
          *#cap_pipe_eq(note.allergy_food_history)*
          *#cap_pipe_eq(note.allergy_etc_history)*
        ]
      ] else [*ไม่มีประวัติการแพ้*]
    ]),
    table.cell(colspan:6,[#align(center)[*ประวัติการเจ็บป่วยปัจจุบัน*]]),
    table.cell(colspan:6,[
      #note.medical_history
      #if note.condition_pregnant != "ปกติ" [-- #note.condition_pregnant]
    ]),
    table.cell(colspan:6,[#align(center)[*ประวัติการเจ็บป่วยในอดีต*]]),
    table.cell(colspan:6,[
      *โรคประจำตัว* #if note.disease != "มี" [
        #sym.ballot.cross ไม่มี #sym.ballot มี
      ] else [
        #sym.ballot ไม่มี #sym.ballot.cross มี #cap_dd(note.disease_detail)
      ] \
      *การผ่าตัด* #if note.operation_history == none [
        #sym.ballot.cross ไม่มี #sym.ballot มี
      ] else [
        #sym.ballot ไม่มี #sym.ballot.cross มี #note.operation_history
      ] \
      *ประวัติครอบครัว* #if note.family_medical_history != "มี" [
        #sym.ballot.cross ไม่มี #sym.ballot มี
      ] else [
        #sym.ballot ไม่มี #sym.ballot.cross มี #cap_fm(note.family_medical_history_detail)
      ] \
      #if pr != none and pt.age_y > 0 [
        *พฤติกรรมเสี่ยง* #if pr.no_risk == "Y" [
          #sym.ballot.cross ไม่มี #sym.ballot มี \
        ] else [
          #sym.ballot ไม่มี #sym.ballot.cross มี \
          #if pr.smoking == "Y" [
            -- *สูบบุหรี่* #pr.smoke_frequency /วัน เป็นเวลา #pr.smoke_year ปี #if pr.smoke_stopped != none [หยุดเมื่อ #pr.smoke_stopped] \
          ]
          #if pr.alcohol == "Y" [
            -- *ดื่มสุรา* #pr.alc_frequency /วัน เป็นเวลา #pr.alc_year ปี #if pr.alc_stopped != none [หยุดเมื่อ #pr.alc_stopped] \
          ]
          #if pr.medication_used == "Y" [
            -- *ยา* #pr.med_name #pr.med_frequency /วัน เป็นเวลา #pr.med_year ปี #if pr.med_stopped != none [หยุดเมื่อ #pr.alc_stopped] \
          ]
        ]
      ]
      #if is_puberty [
        *ประวัติสูตินรีเวช* :
        #if pr != none {
          if pr.period == "ยังไม่มี" [ยังไม่มีประจำเดือน]
          else if pr.period == "หมดประจำเดือน" [
            หมดประจำเดือน#if pr.period_menopause != none [เมื่ออายุ #pr.period_menopause ปี]]
          else [#if pr.period_normal == "ปกติ" [ประจำเดือนปกติ] else [ประจำเดือน#pr.period_disorders]]
          if note.lmp != none [*LMP* #date_th(note.lmp)] else if pr.period_lmp != none [ *LMP* #pr.period_lmp]
        }
        G#note.g;P#note.p GA #note.gestational_age;+#note.gestational_day wks
        #if note.lmp != none [*LMP* #date_th(note.lmp)]
        #if note.edc != none [*EDC* #date_th(note.edc)]
        ANC #note.anc TT #note.tt เข็ม
        #if note.last_child != none [*Last child* อายุ #note.last_child ปี]
        #if note.last_abort != none [*Last abort* เมื่อ #note.last_abort]
        #if note.curette == "Y" [เคย] else [ไม่เคย]ขูดมดลูก
        #if note.pb_no != "Y" [
          #if note.giant_baby == "Y" [เคยคลอดบุตร นน.>4000 กรัม]
          #if note.distocia == "Y" [มีประวัติคลอดยาก]
          #if note.pph == "Y" [มีประวัติตกเลือดหลังคลอด]
          #note.extraction
          #note.pb_etc
        ]
        #if note.hiv != none [*HIV* #note.hiv/#note.hiv2]
        #if note.vdrl != none [*VDRL* #note.vdrl/#note.vdrl2]
        #if note.hbs_ag != none [*HBsAg* #note.hbs_ag/#note.hbs_ag2]
        #if note.hct != none [*Hct* #note.hct/#note.hct2]
        #if note.gr != none [*Bl.gr* #note.gr]
        #if note.thalassemia != none [*Hb-typing* #note.thalassemia]
        #if note.husband != none [*Hb-typing สามี* #note.husband] \
      ]
      #if pt.age_y < 9 [
        *วัคซีน* #if note.receives_immunisation_history_kid == "ครบตามวัย" [
          #sym.ballot.cross ครบตามวัย #sym.ballot ไม่ครบ
        ] else [
          #sym.ballot ครบตามวัย #sym.ballot.cross #note.receives_immunisation_history_kid
        ] \
        *พัฒนาการ* #if note.developmentally_kid == "ปกติ" [
          #sym.ballot.cross ปกติ #sym.ballot ผิดปกติ
        ] else [
          #sym.ballot ปกติ #sym.ballot.cross #note.developmentally_kid
        ] \
        *มารดา* : G#note.g;P#note.p GA #note.gestational_age;+#note.gestational_day wks ANC #note.anc TT #note.tt เข็ม \
        *การคลอด* : #if note.deliver_anomalies != "ปกติ" [#note.deliver_anomalies เนื่องจาก#note.deliver_anomalies_means] else [ปกติ] ที่ #note.deliver_location นน.แรกคลอด #note.deliver_first_weight กรัม #note.deliver_first_health \
        *การเลี้ยงทารก* : #if note.fant_breast_feeding_end_age_month != none [กินนมแม่ถึงอายุ #note.fant_breast_feeding_end_age_month เดือน]
        #if note.fant_artificial_feeding_start_age_month != none [เริ่มนมผสมเมื่ออายุ #note.fant_artificial_feeding_start_age_month เดือน]
        #note.fant_feeding_etc
        #if note.supplementary_feeding == "ได้รับ" [ได้รับอาหารเสริมตั้งแต่อายุ #note.supplementary_feeding_start_age_month เดือน] else  [ยังไม่ได้รับอาหารเสริม] \
      ]
      #if note.addict != none [
        *การใช้ยาและสารเสพติดใน 3 เดือนที่ผ่านมา* #if note.addict != "มี" [
          #sym.ballot.cross ไม่มี #sym.ballot มี
        ] else [
          #sym.ballot ไม่มี #sym.ballot.cross มีการใช้ #cap_ad(note.addict_assist)
        ] \
      ]
      #if note.addict_inj != none [
        *การใช้สารเสพติดชนิดฉีด* #if note.addict_inj != "Y" [
          #sym.ballot.cross ไม่เคย #sym.ballot เคย
        ] else [
          #sym.ballot ไม่เคย #sym.ballot.cross เคย และภายใน 3 เดือนที่ผ่านมา ใช้#if note.addict_inj_often == "Y" [มากกว่า 1 ครั้งต่อสัปดาห์ หรือมากกว่า 3 วันติดต่อกัน] else [1 ครั้งต่อสัปดาห์ หรือน้อยกว่า 3 วันติดต่อกัน]
        ] \
      ]
      #let (awq, awq_h, awq_a, awq_r) = if note.amphetamine_awq == none {(none,none,none,none)} else {
        let split = note.amphetamine_awq.split(",");
        (split.at(0),split.at(1,default: none),split.at(2,default: none),split.at(3,default: none))
      }
      #if awq != none [-- ประเมินอาการถอนพิษแอมเฟตามีน (AWQv2) ได้ #awq คะแนน #if awq_h != none and awq_a != none and awq_r != none [(H: #awq_h, A: #awq_a, R: #awq_r)]#linebreak()]
      #if note.aggression_oas != none [-- ประเมินพฤติกรรมก้าวร้าวรุนแรง (OAS) ได้ #note.aggression_oas.split(",").first() คะแนน#linebreak()]
      #if note.motivation_scale != none [-- ประเมิน Motivation scale ได้ #note.motivation_scale คะแนน#linebreak()]
      #if note.craving_scale != none [-- ประเมิน Craving scale ได้ #note.craving_scale คะแนน#linebreak()]
      #if note.stage_of_change_name != none [-- ประเมิน Stage of change: #note.stage_of_change_name#linebreak()]
      #if note.alcohol_audit != none [-- ประเมินปัญหาการดื่มสุรา (AUDIT) ได้ #note.alcohol_audit.split(",").first() คะแนน#linebreak()]
      #if note.alcohol_ciwa != none [-- ประเมินอาการถอนพิษสุรา (CIWA-Ar) ได้ #note.alcohol_ciwa.split(",").first() คะแนน#linebreak()]
      #if note.alcohol_aws != none [-- ประเมินอาการขาดสุรา (AWS) ได้ #note.alcohol_aws.split(",").first() คะแนน#linebreak()]
      #if note.nicotin_ftnd != none [-- ประเมินระดับการติดนิโคติน (FTND) ได้ #note.nicotin_ftnd.split(",").first() คะแนน#linebreak()]
      #if note.depress_2q != none [-- ประเมินภาวะซึมเศร้า 2 คำถาม (2Q) ได้ #note.depress_2q.split(",").first() ข้อ#linebreak()]
      #if note.depress_9q != none [-- ประเมินภาวะซึมเศร้า 9 คำถาม (9Q) ได้ #note.depress_9q.split(",").first() คะแนน#linebreak()]
      #if note.depress_cdi != none [-- คัดกรองภาวะซึมเศร้าในเด็ก (CDI) ได้ #note.depress_cdi.split(",").first() คะแนน#linebreak()]
      #if note.depress_cesd != none [-- ประเมินภาวะซึมเศร้าในวัยรุ่น (CES-D) ได้ #note.depress_cesd.split(",").first() คะแนน#linebreak()]
      #if note.depress_phqa != none [-- ประเมินภาวะซึมเศร้าในวัยรุ่น (PHQ-A) ได้ #note.depress_phqa.split(",").first() คะแนน#linebreak()]
      #if note.suicide_8q != none [-- ประเมินแนวโน้มฆ่าตัวตาย 8 คำถาม (8Q) ได้ #note.suicide_8q.split(",").first() คะแนน#linebreak()]
      #if note.stress_st5 != none [-- ประเมินภาวะเครียด (ST-5) ได้ #note.stress_st5.split(",").first() คะแนน#linebreak()]
      #if note.ptsd_screen != none [-- ประเมิน PTSD screening test ได้ #note.ptsd_screen.split(",").first() คะแนน#linebreak()]
      #if note.ptsd_pisces != none [-- ประเมินผลกระทบทางจิตใจหลังเกิดเหตุการณ์สะเทือนขวัญ (PISCES-10) ได้ #note.ptsd_pisces.split(",").first() คะแนน#linebreak()]
      #if note.ptsd_cries != none [-- ประเมินผลกระทบจากเหตุการณ์ภัยพิบัติสำหรับเด็ก (CRIES-13) ได้ #note.ptsd_cries.split(",").first() คะแนน#linebreak()]
      *นอน รพ.ครั้งล่าสุด* : #if note.inpatient_history == "เคย" [#datetime_th_checked(note.inpatient_last_date) #note.inpatient_location #note.inpatient_because] else [ไม่เคย]
    ]),
    table.cell(colspan:6,[#align(center)[*Review of Systems*]]),
    table.cell(colspan:6,grid(columns:(1fr,1fr,),gutter:10pt,[
      *โรคเกี่ยวกับ ตา หู คอ จมูก* : #note.ros_eent\
      *โรคระบบประสาท แขนขาอ่อนแรง* : #note.ros_neuro\
      *โรคปอด ถุงลมโป่งพอง หอบหืด*	:	#note.ros_lung\
      *วัณโรค* : #note.ros_tb\
      *ความดันโลหิตสูง* : #note.ros_ht\
      *หัวใจผิดปกติ* : #note.ros_heart\
      *โรคตับและทางเดินน้ำดี* : #note.ros_liver],[
      *โรคกระเพาะอาหาร ลำไส้* : #note.ros_gi\
      *คอพอก เบาหวาน* : #note.ros_endocrine\
      *โรคไต นิ่วไต ไตวาย*	: #note.ros_kidney\
      *เนื้องอก มะเร็ง* : #note.ros_tumour\
      *ความผิดปกติของเม็ดเลือด* : #note.ros_hemato\
      *โรคข้อ เกาต์* : #note.ros_rheumato\
      *โรคทางจิตเวช* : #note.ros_psychia
      ],grid.cell(colspan:2,[*โรคอื่นๆ* : #note.ros_other]),
    )),
    table.cell(colspan:6,[#align(center)[*Physical Examination*]]),
    table.cell(colspan:6,[
      *General* : #note.pe_general\
      *Skin* : #note.pe_skin\
      *HEENT* : #note.pe_heent\
      *Neck* : #note.pe_neck\
      *Breast and Thorax* : #note.pe_breastthorax\
      *Heart* : #note.pe_heart\
      *Lungs* : #note.pe_lungs\
      *Abdomen* : #note.pe_abdomen
        #if note.hf != none [, Height of fundus #note.hf cm]
        #if note.hf_position != none [#note.hf_position]\
      *Rectum and Genitalia* : #note.pe_rectalgenitalia\
      *Extremities* : #note.pe_extremities\
      *Neuro* : #note.pe_neurological\
      #if is_female [*OB/Gyn* : #if note.lr_presentation == none [#note.pe_ob_gynexam] else [
        - *Height of fundus*: #note.hf cm, *Back of fetus*: #note.lr_back_fetus, *Postition*: #note.hf_position, *Presentation*: #note.lr_presentation, *Engagement*: #if note.lr_engagement == "Y" [Yes] else [No], *Cephalic prominence*: #note.lr_prominence, *Attitude*: #note.lr_attitude,
        - *FHR*: #note.lr_fhr/min #if note.lr_fhr_irrigular == "Y" [irregular] else [regular], *EFW*: #note.lr_efw gms, *Interval*: #note.lr_interval, *Duration*: #note.lr_duration", *Intensity*: #note.lr_intensity,
        - *Diagonal conjugate*: #note.lr_pelvic_diagonal cm, *Interspinous diameter*: #note.lr_pelvic_interspinous cm, *Pelvic sidewall*: #note.lr_pelvic_sidewall, *Ischeal spine*: #note.lr_ischeal_spine, *Sacral curve*: #note.lr_sacral_curve, *Pubic angle*: #note.lr_pubic_angle degree, *Assessment*: #if note.lr_pelvic_ok == "Y" [Adequate] else [Contract],
        - *Dilatation*: #note.lr_cx_dilate cm, *Effacement*: #note.lr_cx_efface%, *Station*: #if note.lr_cx_station > 0 [\+]#note.lr_cx_station, *Position*: #note.lr_cx_position, *Consistency*: #note.lr_cx_consistency, *Bishop score*: #note.lr_cx_bishop, *Assessment*: #if note.lr_cx_ok == "Y" [Favorable] else [Unfavorable]
        - *Membrane*: #note.lr_membrane, #if note.lr_amniotic_color != none [*Amniotic fluid*: #note.lr_amniotic_color, *Amniotic fluid odor*: #note.lr_amniotic_smell]]]
      *Other* : #note.pe_other\
      *PE Text* : #note.pe_text
    ]),
    ..if note.imgs != none {(
      table.cell(colspan:6,[#align(center)[*รูปภาพ*]]),
      table.cell(colspan:6,inset:(top:5pt,bottom:9pt),explode_imgs(5,false,note.imgs))
    )} else {(table.cell(colspan:6,inset:0pt,[]),)},
    table.cell(colspan:6,inset:0pt,if note.svg_tag != none and note.svg_tag.contains("path") [
      #image("statics/picture/allbody.svg") #if type(note.svg_tag) == str [#place(top + left,dx:2pt,dy:2pt,image(bytes(note.svg_tag)))]
    ]),
    table.cell(colspan:3,[
      *Impression* : #note.impression\
      *Diff Dx* : #note.diff_dx\
      *Plan Management* : #note.plan_management
    ]),
    table.cell(colspan:3,[
      *แพทย์ผู้บันทึก* : #docs.map(d => [#d.admission_note_doctorname #d.entryposition #d.licenseno]).join(", ")\
      *พยาบาลผู้บันทึก* : #note.nurse_name #note.nurse_pos #note.nurse_licenseno
    ])
  )
}
