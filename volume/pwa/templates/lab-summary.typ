#import "templates/utils.typ": api, vnan_is_ipd, get_patient_main, date_th, time_th, parse_d_t, dt2th, is_lab_ab
#let max_col = 10
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let is_ipd = vnan_is_ipd(data.id)
#let pt = data.at("patient", default: none)
#if pt == none { pt = get_patient_main(data.id) }
#let labs = data.at("lab", default: none)
#if labs == none {labs = json(api + "lab/head?vn=" + data.id)}
// PREPARED FUNCTIONS
#let table_h(c) = [#align(center,strong(c))]
#let label_note(l,n) = [#text(weight:700,l) #n]
// RENDER
#set text(font:"TH Sarabun New",size:12pt)
#set page(paper:"a4",flipped:true,margin:(x:1cm,y:1.5cm),
  header: context[#h(1fr) #text(size:20pt,weight:700,[LAB SUMMARY])#h(1fr)#counter(page).display("1/1",both:true)],
  footer: [#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([VN : ],pt.vn) #label_note([AN : ],pt.an)],
)
#let labs = labs.filter(lab => lab.confirm_report == "Y")
#let groups = labs.map(lab => lab.lab_items_group.map(gr => gr.lab_items_group_name)).flatten().dedup().sorted()
#let items = labs.map(lab => lab.lab_items_group.map(gr => gr.lab_items.map(it => {
  it.rdt = parse_d_t(it.report_date,it.report_time);it
}).filter(it => it.rdt != none))).flatten()
#for group in groups {
  let gr_items_all = items.filter(it => it.lab_items_group_name == group)
  let dts_all = gr_items_all.map(it => it.rdt).dedup().sorted()
  let dts_len = dts_all.len()
  let tbls = calc.ceil(dts_len / max_col)
  for tbl in range(tbls) {
    let end_col = (tbl + 1) * max_col
    let dts = dts_all.slice(tbl * max_col, if end_col > dts_len {dts_len} else {end_col})
    let gr_items = gr_items_all.filter(it => dts.contains(it.rdt))
    let item_heads = gr_items.map(it => (it.lab_items_name_ref,it.lab_items_normal_value_ref,it.lab_items_unit)).dedup(key: ((a,b,c)) => a)
    let remains_dts = range(max_col - dts.len())
    [
      #strong[#group #if tbls > 1 [(#(tbl + 1)/#tbls)]]#v(-8pt)
      #table(columns:(1fr,110pt,..(60pt,)*max_col),stroke:.5pt,
        table.header(strong[#group#linebreak()LAB NAME],table_h[NORMAL RANGE],..dts.map(dt => table_h(dt2th(dt))),..remains_dts.map(x => [])),
        ..item_heads.map(((name,normal,unit)) => (
          strong(name),if normal == none or normal == "" [\u{2012} #unit] else [#normal #unit],..dts.map(dt => {
            let opt = gr_items.find(it => it.rdt == dt and it.lab_items_name_ref == name);
            if opt != none {
              if opt.lab_order_result != none and normal != "-" {
                let ab_res = is_lab_ab(opt.lab_order_result, normal)
                if ab_res == "T" {text(red,weight:700,opt.lab_order_result)}
                else if ab_res == "H" {text(red,weight:700,[#opt.lab_order_result#h(1fr)H])}
                else if ab_res == "L" {text(red,weight:700,[#opt.lab_order_result#h(1fr)L])}
                else {opt.lab_order_result}
              } else {opt.lab_order_result}
            } else []
          }),..remains_dts.map(x => [])
        )).flatten()
      )
    ]
  }
}