#import "templates/utils.typ": api, vnan_is_ipd, get_patient_main, datetime_th
#let data = json("data.json")
#let vnan = data.at("id", default: none)
#let doc_type_id = data.at("doc_type_id", default: 1)
#let per_page = data.at("per_page", default: 1)
#assert(vnan != none, message:"invalid 'id'")
#let is_ipd = vnan_is_ipd(vnan)
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(vnan) }
#let im_paths = data.at("im_paths",default: none)
#if im_paths == none {
  let docs = data.at("docs",default: none)
  if docs == none {
    if is_ipd {
      docs = json(api + "ipd/document/scan-an/" + vnan)
    } else {
      let id = if pt.opd_er_order_master_id != none {pt.opd_er_order_master_id} else {0}
      docs = if id == 0 {()} else { json(api + "opd-er/document/scan-id/" + str(id)) }
    }
  }
  let doc = if docs.len() == 0 {none} else {docs.find(d => d.document_type_id == doc_type_id)}
  let doc_id = if doc == none {none} else {doc.document_id}
  let usage_id = if is_ipd {"11"} else {"12"}
  im_paths = if doc_id == none {()} else {json(api + "image-usage-id/" + usage_id + "/" + str(doc_id))}
}
#let label_note(l,n) = [#text(weight:700,l) #n]
#let footer = if pt == none [] else [
  #label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([VN : ],pt.vn) #if is_ipd {label_note([AN : ],pt.an)}
]
#set page(paper:"a4",margin:1cm,columns:if per_page > 2 {2} else {1},footer:footer)
#if im_paths.len() > 0 {
  for im_path in im_paths {
    if per_page == 1 {
      figure(image("images/" + im_path.path,width: 100%),caption:text(14pt,[#im_path.title #datetime_th(im_path.create_datetime)]))
    } else if per_page == 2 {
      figure(image("images/" + im_path.path,height:340pt,fit:"contain"),caption:text(14pt,[#im_path.title #datetime_th(im_path.create_datetime)]),placement:auto)
    } else {
      figure(image("images/" + im_path.path,width:100%,fit:"contain"),caption:text(14pt,[#im_path.title #datetime_th(im_path.create_datetime)]))
    }
  }
}