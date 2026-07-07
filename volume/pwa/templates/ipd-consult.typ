#import "customs/config.typ": hospital-name
#import "templates/utils.typ": api, get_patient_main, date_th, time_th, datetime_th, parse_d_t, explode_imgs
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let cons = data.at("consults",default: none)
#if cons == none {cons = json(api + "ipd/consult-an/" + data.id)}
// PREPARED FUNCTIONS
#let section(c) = align(center,text(size:20pt,weight:700,c))
#let label_note(l,n) = [#text(weight:700,l) #n]
// RENDER
#set page(paper:"a4",margin:1cm)
#set text(font:"TH Sarabun New",size:14pt)
#set grid(gutter:10pt)
#let render(con) = [
  #section[#hospital-name]
  #section[CONSULTATION]
  #grid(columns:(1fr,1fr),
    label_note([NAME : ],[#pt.pname #pt.fname #pt.lname]),label_note([AGE : ],[#pt.age_y ปี #pt.age_m เดือน]),
    label_note([HN : ],pt.hn),[#label_note([WARD : ],pt.at("ward_name",default: none))#h(10pt)#label_note([BED : ],pt.at("bedno",default: none))],
    label_note([DATE : ],[#date_th(con.consult_date) #time_th(con.consult_time)]),label_note([EMERGENCY : ],con.consult_emergency_name)
  )
  #line(stroke:.5pt,length:100%)
  #label_note([CONSULT TO DEPARTMENT : ],con.spcltyname)\
  #strong[HISTORY AND PHYSICAL EXAMINATION AND LAB FINDING :]\
  #pad(left:15pt,con.consult_data)
  #explode_imgs(5,false,con.d_imgs)\ \
  #align(right,con.string_consult_request_name)
  #align(right,[PHYSICIAN ATTENDING])
  #line(stroke:.5pt,length:100%)
  #section[CONSULTATION REPORT]
  #if con.consult_datetime_create_reply != none [#label_note([DATE : ],[#datetime_th(con.consult_datetime_create_reply) #if con.consult_datetime_update_reply != none and con.consult_datetime_update_reply != con.consult_datetime_create_reply [#label_note([LAST UPDATE : ],datetime_th(con.consult_datetime_create_reply))]])]\
  #v(5pt)#strong[FINDING]\
  #pad(left:15pt,con.consult_finding)
  #explode_imgs(5,false,con.f_imgs)\ \
  #strong[DIAGNOSIS]\
  #pad(left:15pt,con.consult_diagnosis)
  #strong[RECOMMENDATION]\
  #pad(left:15pt,con.consult_recommendation)
  #align(right,con.string_consult_reply_name)
  #align(right,[CONSULTANT])
]
#if cons.len() > 0 {
  cons.sorted(key:cn => parse_d_t(cn.consult_date,cn.consult_time)).map(cn => render(cn)).join(pagebreak())
}
