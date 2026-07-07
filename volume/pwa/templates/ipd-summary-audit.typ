#import "@preview/oxifmt:1.0.0": strfmt
#import "customs/config.typ": hospcode, hospital-name
#import "templates/utils.typ": api, get_patient_main, month_th, date_th, datetime_th, time_th, parse_d, parse_sex
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let audits = data.at("audit",default: none)
#if audits == none {audits = json(api + "ipd/summary-audit?an=" + data.id)}
// PREPARED FUNCTIONS
#let label_note(l,n) = [#text(weight:700,l) #n]
#let is_yn(a,b) = if a == none or b == none {none} else if a == b [Y] else [N]
#let parse_auth(s) = if s == none {none} else {
  ((s == "N","No"),
    (s == "T","Signed (text signature)"),
    (s == "C","Signed (cursive signature)"),
    (s == "D","Signed (digital signature)"),
    (true, "")).find(t => t.at(0)).at(1)
}
#let parse_audit(s) = if s == none {none} else {
  ((s == "I","Internal"),
    (s == "E","External"),
    (true, "")).find(t => t.at(0)).at(1)
}
#let table_h(c) = align(center,strong(c))
#let t_center(c) = align(center,c)
#let audit_row(i,a) = (t_center[#a.ty#i],t_center[#a.com_icd],[#a.sum_dx],t_center[#a.sum_icd],[#a.rev_dx],t_center[#a.rev_icd],t_center[#a.sa],t_center[#a.ca],[#a.remark])
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[แบบบันทึกผลการตรวจสอบเวชระเบียน (Summary / Coding Audit) #hospital-name])#h(1fr)#counter(page).display("1/1",both:true)],
)
// Page template
#let body(b) = {
  let pdx = b.summary_audit_items.filter(i => i.ty == "PDx")
  let sdx = b.summary_audit_items.filter(i => i.ty == "SDx")
  let odx = b.summary_audit_items.filter(i => i.ty == "ODx")
  let op = b.summary_audit_items.filter(i => i.ty == "Op")
  let sas = (b.sa,..pdx.map(i => i.sa),..sdx.map(i => i.sa),..odx.map(i => i.sa),..op.map(i => i.sa)).dedup().sorted()
  let cas = (b.ca,..pdx.map(i => i.ca),..sdx.map(i => i.ca),..odx.map(i => i.ca),..op.map(i => i.ca)).dedup().sorted()
  box[
    #label_note([HCODE],[#hospcode])\
    *From Review* #label_note([HN : ],[#b.rev_hn]) #label_note([AN : ],[#b.rev_an]) #label_note([DATE Adm : ],[#datetime_th(b.rev_adm_datetime)]) #label_note([Disch : ],[#datetime_th(b.rev_dch_datetime)]) #label_note([LEAVE : ],[#b.rev_leaveday]) #label_note([SEX : ],[#parse_sex(b.rev_sex)]) #label_note([DOB : ],[#date_th(b.rev_birthday)]) #label_note([AdmW (Kg) : ],[#strfmt("{:.3}",b.rev_bw / 1000)]) #label_note([Disch status : ],[#b.rev_dchstts]) #label_note([type : ],[#b.rev_dchtype]) #label_note([PID : ],[#b.rev_pid])\
    *In Computer* #label_note([HN : ],[#b.com_hn]) #label_note([AN : ],[#b.com_an]) #label_note([DATE Adm : ],[#datetime_th(b.com_adm_datetime)]) #label_note([Disch : ],[#datetime_th(b.com_dch_datetime)]) #label_note([LEAVE : ],[#b.com_leaveday]) #label_note([SEX : ],[#parse_sex(b.com_sex)]) #label_note([DOB : ],[#date_th(b.com_birthday)]) #label_note([AdmW (Kg) : ],[#strfmt("{:.3}",b.com_bw / 1000)]) #label_note([Disch status : ],[#b.com_dchstts]) #label_note([type : ],[#b.com_dchtype]) #label_note([PID : ],[#b.com_pid])\
    *Correct (Y/N)* #h(30pt)#is_yn(b.rev_hn, b.com_hn) #h(50pt)#is_yn(b.rev_an, b.com_an) #h(105pt)#is_yn(b.rev_adm_datetime, b.com_adm_datetime) #h(95pt)#is_yn(b.rev_dch_datetime, b.com_dch_datetime) #h(55pt)#is_yn(b.rev_leaveday, b.com_leaveday) #h(23pt)#is_yn(b.rev_sex, b.com_sex) #h(43pt)#is_yn(b.rev_birthday, b.com_birthday) #h(77pt)#is_yn(b.rev_bw, b.com_bw) #h(64pt)#is_yn(b.rev_dchstts, b.com_dchstts) #h(29pt)#is_yn(b.rev_dchtype, b.com_dchtype) #h(45pt)#is_yn(b.rev_pid, b.com_pid)\ 
    #label_note([Payer type : ],[#b.payer]) #h(30pt)#label_note([Physician's signature : ],[#parse_auth(b.doctor_auth)]) #h(30pt)#label_note([Audit type : ],[#parse_audit(b.audit_type)])\
    *Summary and Coding Audit*\
    #v(-10pt)
    #table(columns:(35pt,55pt,2fr,40pt,2fr,40pt,30pt,30pt,1fr),stroke:.5pt,
      table.header(
        table_h[Dx/Op],table_h[ICD in Com],table_h[Dx/Op in Summary],table_h[S ICD],table_h[Dx/Op from Review],table_h[R ICD],table_h[SA],table_h[CA],table_h[Remark]
      ),
      ..pdx.map(a => audit_row(none,a)).flatten(),
      ..sdx.enumerate(start:1).map(((i,a)) => audit_row(i,a)).flatten(),
      ..odx.enumerate(start:1).map(((i,a)) => audit_row(i,a)).flatten(),
      ..op.enumerate(start:1).map(((i,a)) => audit_row(i,a)).flatten(),
    )
    #v(-7pt)
    #label_note([Summary Assess : ], sas.join(" "))\
    #label_note([Code Assess : ], cas.join(" "))\
    #label_note([DRG in Computer : ], b.com_drg) #h(30pt)#label_note([RW in Computer : ], b.com_rw) #h(30pt)#label_note([AdjRW in Computer : ], b.com_adjrw)\
    #label_note([DRG from R ICD : ], b.rev_drg) #h(37pt)#label_note([RW from R ICD : ], b.rev_rw) #h(37pt)#label_note([AdjRW from R ICD : ], b.rev_adjrw)\
    #label_note([Auditor : ], [#b.create_username \(ตรวจสอบ #datetime_th(b.create_datetime)#if b.create_datetime != b.update_datetime [ แก้ไข #datetime_th(b.update_datetime)]\)])
  ]
}
// render all pages
#for p in audits {
  body(p)
}