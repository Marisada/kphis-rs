#import "templates/utils.typ": api, vnan_is_ipd, get_patient_main, base64_to_byte
#let data = json("data.json")
#let vnan = data.at("id", default: none)
#let key = data.at("key", default: "opd")
#let per_page = data.at("per_page", default: 1)
#assert(vnan != none, message:"invalid 'id'")
#let is_ipd = vnan_is_ipd(vnan)
#let pt = data.at("patient",default: none)
#if pt == none { pt = get_patient_main(vnan) }
#let images = data.at("images",default: none)
#if images == none { images = json(api + "scan/his/image?key=" + key + "&vn=" + vnan) }
#let label_note(l,n) = [#text(weight:700,l) #n]
#let footer = if pt == none [] else [
  #label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([VN : ],pt.vn) #if is_ipd {label_note([AN : ],pt.an)}
]
#set page(paper:"a4",margin:1cm,columns:if per_page > 2 {2} else {1},footer:footer)
#if images.len() > 0 {
  for img in images {
    if per_page == 1 {
      figure(image(base64_to_byte(img.image.image),width:100%),caption:text(14pt,[#img.note]))
    } else if per_page == 2 {
      figure(image(base64_to_byte(img.image.image),height:340pt,fit:"contain"),caption:text(14pt,[#img.note]),placement:auto)
    } else {
      figure(image(base64_to_byte(img.image.image),width:100%,fit:"contain"),caption:text(14pt,[#img.note]))
    }
  }
}