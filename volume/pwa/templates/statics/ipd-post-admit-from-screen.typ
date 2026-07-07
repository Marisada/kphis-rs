#let data = json("data.json")
#let row_data = data.at("rows",default: none)
// PREPARED FUNCTIONS
#let parse_d(date) = if date == none {none} else {
  let (y,mo,d) = date.split("-")
  datetime(year:int(y),month:int(mo),day:int(d))
}
#let audit_status(s) = {
  ((s == "review","รอ Review"),
   (s == "code","รอ Coder"),
   (s == "audit","รอ Audit"),
   (s == "claim","รอ Claim"),
   (s == "appeal","รออุทธรณ์"),
   (s == "done","สิ้นสุด"),
   (true, "ยังไม่สรุป")).find(t => t.at(0)).at(1)
}
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,top:1cm,bottom:.5cm),
  footer-descent:-10pt,
  footer: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
#table(columns:(40pt,50pt,60pt,100pt,50pt,50pt,140pt,45pt,100pt,100pt,1fr),stroke:.5pt,
  table.header(
    strong[HN],strong[AN],strong[Ward],strong[ชื่อ-สกุล],strong[อายุ],strong[สถานะ D/C],strong[สิทธิ์การรักษา],strong[จำนวนวันหลัง D/C],strong[แพทย์ผู้ Admit],strong[แพทย์ผู้ D/C],strong[สถานะ],
  ),..row_data.map(r => (
    [#r.hn],[#r.an],[#r.ward_name],[#r.fullname],[#r.age_y ปี #r.age_m เดือน #r.age_d วัน],[#r.dchtype_name],[#r.rtname],
    [#(datetime.today() - parse_d(r.dchdate)).days() วัน],[#r.admdoctor_name],[#r.dchdoctor_name],[#audit_status(r.summary_status)],
  )).flatten()
)