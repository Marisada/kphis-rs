#import "@preview/oxifmt:1.0.0": strfmt
#import "templates/utils.typ": date_th, thousands
// PRELUDE
#let data = json("data.json")
#let wards = data.at("ward",default: ())
#let audit_types = data.at("audit_type",default: ())
#let is_psys = data.at("is_psychiatry",default: ())
#let ipt = data.at("data",default: ())
// PREPARED FUNCTIONS
#let text_c(c) = align(center,c)
#let table_h(c) = [#align(center,strong(c))]
#let percent(v,m) = if m == 0 [--] else [#strfmt("{:.2}",v * 100 / m)]
// PREPARED VARIABLES
#let ward = wards.join(", ")
#let audit_type = audit_types.join(" และ ")
#let is_psy = is_psys.join("และ")
#let notna(v) = if v == none {0} else {1}
#let some1(v) = if v != none and v {1} else {0}
#let some0(v) = if v != none and not v {1} else {0}
#let sum_len = ipt.filter(i => i.s_id != none).len()
#let audit = ipt.filter(i => i.mra_id != none)
#let audit_len = audit.len()
#let ipt_len = ipt.len()
#let sd_max = audit.map(a => notna(a.sd_1)+notna(a.sd_2)+notna(a.sd_3)+notna(a.sd_4)+notna(a.sd_5)+notna(a.sd_6)+notna(a.sd_7)+notna(a.sd_8)+notna(a.sd_9)).sum(default:0)
#let so_max = audit.map(a => notna(a.so_1)+notna(a.so_2)+notna(a.so_3)+notna(a.so_4)+notna(a.so_5)+notna(a.so_6)+notna(a.so_7)).sum(default:0)
#let ic_max = audit.map(a => notna(a.ic_1)+notna(a.ic_2)+notna(a.ic_3)+notna(a.ic_4)+notna(a.ic_5)+notna(a.ic_6)+notna(a.ic_7)+notna(a.ic_8)+notna(a.ic_9)).sum(default:0)
#let hx_max = audit.map(a => notna(a.hx_1)+notna(a.hx_2)+notna(a.hx_3)+notna(a.hx_4)+notna(a.hx_5)+notna(a.hx_6)+notna(a.hx_7)+notna(a.hx_8)+notna(a.hx_9)).sum(default:0)
#let pe_max = audit.map(a => notna(a.pe_1)+notna(a.pe_2)+notna(a.pe_3)+notna(a.pe_4)+notna(a.pe_5)+notna(a.pe_6)+notna(a.pe_7)+notna(a.pe_8)+notna(a.pe_9)).sum(default:0)
#let pn_max = audit.map(a => notna(a.pn_1)+notna(a.pn_2)+notna(a.pn_3)+notna(a.pn_4)+notna(a.pn_5)+notna(a.pn_6)+notna(a.pn_7)+notna(a.pn_8)+notna(a.pn_9)).sum(default:0)
#let cr_max = audit.map(a => notna(a.cr_1)+notna(a.cr_2)+notna(a.cr_3)+notna(a.cr_4)+notna(a.cr_5)+notna(a.cr_6)+notna(a.cr_7)+notna(a.cr_8)+notna(a.cr_9)).sum(default:0)
#let ar_max = audit.map(a => notna(a.ar_1)+notna(a.ar_2)+notna(a.ar_3)+notna(a.ar_4)+notna(a.ar_5)+notna(a.ar_6)+notna(a.ar_7)+notna(a.ar_8)+notna(a.ar_9)).sum(default:0)
#let on_max = audit.map(a => notna(a.on_1)+notna(a.on_2)+notna(a.on_3)+notna(a.on_4)+notna(a.on_5)+notna(a.on_6)+notna(a.on_7)+notna(a.on_8)+notna(a.on_9)).sum(default:0)
#let lr_max = audit.map(a => notna(a.lr_1)+notna(a.lr_2)+notna(a.lr_3)+notna(a.lr_4)+notna(a.lr_5)+notna(a.lr_6)+notna(a.lr_7)+notna(a.lr_8)+notna(a.lr_9)).sum(default:0)
#let rr_max = audit.map(a => notna(a.rr_1)+notna(a.rr_2)+notna(a.rr_3)+notna(a.rr_4)+notna(a.rr_5)+notna(a.rr_6)+notna(a.rr_7)+notna(a.rr_8)+notna(a.rr_9)).sum(default:0)
#let nn_max = audit.map(a => notna(a.nn_1)+notna(a.nn_2)+notna(a.nn_3)+notna(a.nn_4)+notna(a.nn_5)+notna(a.nn_6)+notna(a.nn_7)+notna(a.nn_8)+notna(a.nn_9)).sum(default:0)
#let sd_v = audit.map(a => some1(a.sd_1)+some1(a.sd_2)+some1(a.sd_3)+some1(a.sd_4)+some1(a.sd_5)+some1(a.sd_6)+some1(a.sd_7)+some1(a.sd_8)+some1(a.sd_9)).sum(default:0)
#let so_v = audit.map(a => some1(a.so_1)+some1(a.so_2)+some1(a.so_3)+some1(a.so_4)+some1(a.so_5)+some1(a.so_6)+some1(a.so_7)).sum(default:0)
#let ic_v = audit.map(a => some1(a.ic_1)+some1(a.ic_2)+some1(a.ic_3)+some1(a.ic_4)+some1(a.ic_5)+some1(a.ic_6)+some1(a.ic_7)+some1(a.ic_8)+some1(a.ic_9)).sum(default:0)
#let hx_v = audit.map(a => some1(a.hx_1)+some1(a.hx_2)+some1(a.hx_3)+some1(a.hx_4)+some1(a.hx_5)+some1(a.hx_6)+some1(a.hx_7)+some1(a.hx_8)+some1(a.hx_9)).sum(default:0)
#let pe_v = audit.map(a => some1(a.pe_1)+some1(a.pe_2)+some1(a.pe_3)+some1(a.pe_4)+some1(a.pe_5)+some1(a.pe_6)+some1(a.pe_7)+some1(a.pe_8)+some1(a.pe_9)).sum(default:0)
#let pn_v = audit.map(a => some1(a.pn_1)+some1(a.pn_2)+some1(a.pn_3)+some1(a.pn_4)+some1(a.pn_5)+some1(a.pn_6)+some1(a.pn_7)+some1(a.pn_8)+some1(a.pn_9)).sum(default:0)
#let cr_v = audit.map(a => some1(a.cr_1)+some1(a.cr_2)+some1(a.cr_3)+some1(a.cr_4)+some1(a.cr_5)+some1(a.cr_6)+some1(a.cr_7)+some1(a.cr_8)+some1(a.cr_9)).sum(default:0)
#let ar_v = audit.map(a => some1(a.ar_1)+some1(a.ar_2)+some1(a.ar_3)+some1(a.ar_4)+some1(a.ar_5)+some1(a.ar_6)+some1(a.ar_7)+some1(a.ar_8)+some1(a.ar_9)).sum(default:0)
#let on_v = audit.map(a => some1(a.on_1)+some1(a.on_2)+some1(a.on_3)+some1(a.on_4)+some1(a.on_5)+some1(a.on_6)+some1(a.on_7)+some1(a.on_8)+some1(a.on_9)).sum(default:0)
#let lr_v = audit.map(a => some1(a.lr_1)+some1(a.lr_2)+some1(a.lr_3)+some1(a.lr_4)+some1(a.lr_5)+some1(a.lr_6)+some1(a.lr_7)+some1(a.lr_8)+some1(a.lr_9)).sum(default:0)
#let rr_v = audit.map(a => some1(a.rr_1)+some1(a.rr_2)+some1(a.rr_3)+some1(a.rr_4)+some1(a.rr_5)+some1(a.rr_6)+some1(a.rr_7)+some1(a.rr_8)+some1(a.rr_9)).sum(default:0)
#let nn_v = audit.map(a => some1(a.nn_1)+some1(a.nn_2)+some1(a.nn_3)+some1(a.nn_4)+some1(a.nn_5)+some1(a.nn_6)+some1(a.nn_7)+some1(a.nn_8)+some1(a.nn_9)-some1(a.nn_sub)).sum(default:0)
#let total_v = sd_v+so_v+ic_v+hx_v+pe_v+pn_v+cr_v+ar_v+on_v+lr_v+rr_v+nn_v
#let total_max = sd_max+so_max+ic_max+hx_max+pe_max+pn_max+cr_max+ar_max+on_max+lr_max+rr_max+nn_max
#let oppo = (
  (i:119,k:"1. Discharge summary: Dx., OP ข้อที่ 1",v:audit.map(a => some0(a.sd_1)).sum(default:0)),
  (i:118,k:"1. Discharge summary: Dx., OP ข้อที่ 2",v:audit.map(a => some0(a.sd_2)).sum(default:0)),
  (i:117,k:"1. Discharge summary: Dx., OP ข้อที่ 3",v:audit.map(a => some0(a.sd_3)).sum(default:0)),
  (i:116,k:"1. Discharge summary: Dx., OP ข้อที่ 4",v:audit.map(a => some0(a.sd_4)).sum(default:0)),
  (i:115,k:"1. Discharge summary: Dx., OP ข้อที่ 5",v:audit.map(a => some0(a.sd_5)).sum(default:0)),
  (i:114,k:"1. Discharge summary: Dx., OP ข้อที่ 6",v:audit.map(a => some0(a.sd_6)).sum(default:0)),
  (i:113,k:"1. Discharge summary: Dx., OP ข้อที่ 7",v:audit.map(a => some0(a.sd_7)).sum(default:0)),
  (i:112,k:"1. Discharge summary: Dx., OP ข้อที่ 8",v:audit.map(a => some0(a.sd_8)).sum(default:0)),
  (i:111,k:"1. Discharge summary: Dx., OP ข้อที่ 9",v:audit.map(a => some0(a.sd_9)).sum(default:0)),
  (i:107,k:"2. Discharge summary: Other ข้อที่ 1",v:audit.map(a => some0(a.so_1)).sum(default:0)),
  (i:106,k:"2. Discharge summary: Other ข้อที่ 2",v:audit.map(a => some0(a.so_2)).sum(default:0)),
  (i:105,k:"2. Discharge summary: Other ข้อที่ 3",v:audit.map(a => some0(a.so_3)).sum(default:0)),
  (i:104,k:"2. Discharge summary: Other ข้อที่ 4",v:audit.map(a => some0(a.so_4)).sum(default:0)),
  (i:103,k:"2. Discharge summary: Other ข้อที่ 5",v:audit.map(a => some0(a.so_5)).sum(default:0)),
  (i:102,k:"2. Discharge summary: Other ข้อที่ 6",v:audit.map(a => some0(a.so_6)).sum(default:0)),
  (i:101,k:"2. Discharge summary: Other ข้อที่ 7",v:audit.map(a => some0(a.so_7)).sum(default:0)),
  (i:99,k:"3. Informed consent ข้อที่ 1",v:audit.map(a => some0(a.ic_1)).sum(default:0)),
  (i:98,k:"3. Informed consent ข้อที่ 2",v:audit.map(a => some0(a.ic_2)).sum(default:0)),
  (i:97,k:"3. Informed consent ข้อที่ 3",v:audit.map(a => some0(a.ic_3)).sum(default:0)),
  (i:96,k:"3. Informed consent ข้อที่ 4",v:audit.map(a => some0(a.ic_4)).sum(default:0)),
  (i:95,k:"3. Informed consent ข้อที่ 5",v:audit.map(a => some0(a.ic_5)).sum(default:0)),
  (i:94,k:"3. Informed consent ข้อที่ 6",v:audit.map(a => some0(a.ic_6)).sum(default:0)),
  (i:93,k:"3. Informed consent ข้อที่ 7",v:audit.map(a => some0(a.ic_7)).sum(default:0)),
  (i:92,k:"3. Informed consent ข้อที่ 8",v:audit.map(a => some0(a.ic_8)).sum(default:0)),
  (i:91,k:"3. Informed consent ข้อที่ 9",v:audit.map(a => some0(a.ic_9)).sum(default:0)),
  (i:89,k:"4. History ข้อที่ 1",v:audit.map(a => some0(a.hx_1)).sum(default:0)),
  (i:88,k:"4. History ข้อที่ 2",v:audit.map(a => some0(a.hx_2)).sum(default:0)),
  (i:87,k:"4. History ข้อที่ 3",v:audit.map(a => some0(a.hx_3)).sum(default:0)),
  (i:86,k:"4. History ข้อที่ 4",v:audit.map(a => some0(a.hx_4)).sum(default:0)),
  (i:85,k:"4. History ข้อที่ 5",v:audit.map(a => some0(a.hx_5)).sum(default:0)),
  (i:84,k:"4. History ข้อที่ 6",v:audit.map(a => some0(a.hx_6)).sum(default:0)),
  (i:83,k:"4. History ข้อที่ 7",v:audit.map(a => some0(a.hx_7)).sum(default:0)),
  (i:82,k:"4. History ข้อที่ 8",v:audit.map(a => some0(a.hx_8)).sum(default:0)),
  (i:81,k:"4. History ข้อที่ 9",v:audit.map(a => some0(a.hx_9)).sum(default:0)),
  (i:79,k:"5. Physical exam ข้อที่ 1",v:audit.map(a => some0(a.pe_1)).sum(default:0)),
  (i:78,k:"5. Physical exam ข้อที่ 2",v:audit.map(a => some0(a.pe_2)).sum(default:0)),
  (i:77,k:"5. Physical exam ข้อที่ 3",v:audit.map(a => some0(a.pe_3)).sum(default:0)),
  (i:76,k:"5. Physical exam ข้อที่ 4",v:audit.map(a => some0(a.pe_4)).sum(default:0)),
  (i:75,k:"5. Physical exam ข้อที่ 5",v:audit.map(a => some0(a.pe_5)).sum(default:0)),
  (i:74,k:"5. Physical exam ข้อที่ 6",v:audit.map(a => some0(a.pe_6)).sum(default:0)),
  (i:73,k:"5. Physical exam ข้อที่ 7",v:audit.map(a => some0(a.pe_7)).sum(default:0)),
  (i:72,k:"5. Physical exam ข้อที่ 8",v:audit.map(a => some0(a.pe_8)).sum(default:0)),
  (i:71,k:"5. Physical exam ข้อที่ 9",v:audit.map(a => some0(a.pe_9)).sum(default:0)),
  (i:69,k:"6. Progress note ข้อที่ 1",v:audit.map(a => some0(a.pn_1)).sum(default:0)),
  (i:68,k:"6. Progress note ข้อที่ 2",v:audit.map(a => some0(a.pn_2)).sum(default:0)),
  (i:67,k:"6. Progress note ข้อที่ 3",v:audit.map(a => some0(a.pn_3)).sum(default:0)),
  (i:66,k:"6. Progress note ข้อที่ 4",v:audit.map(a => some0(a.pn_4)).sum(default:0)),
  (i:65,k:"6. Progress note ข้อที่ 5",v:audit.map(a => some0(a.pn_5)).sum(default:0)),
  (i:64,k:"6. Progress note ข้อที่ 6",v:audit.map(a => some0(a.pn_6)).sum(default:0)),
  (i:63,k:"6. Progress note ข้อที่ 7",v:audit.map(a => some0(a.pn_7)).sum(default:0)),
  (i:62,k:"6. Progress note ข้อที่ 8",v:audit.map(a => some0(a.pn_8)).sum(default:0)),
  (i:61,k:"6. Progress note ข้อที่ 9",v:audit.map(a => some0(a.pn_9)).sum(default:0)),
  (i:59,k:"7. Consultation record ข้อที่ 1",v:audit.map(a => some0(a.cr_1)).sum(default:0)),
  (i:58,k:"7. Consultation record ข้อที่ 2",v:audit.map(a => some0(a.cr_2)).sum(default:0)),
  (i:57,k:"7. Consultation record ข้อที่ 3",v:audit.map(a => some0(a.cr_3)).sum(default:0)),
  (i:56,k:"7. Consultation record ข้อที่ 4",v:audit.map(a => some0(a.cr_4)).sum(default:0)),
  (i:55,k:"7. Consultation record ข้อที่ 5",v:audit.map(a => some0(a.cr_5)).sum(default:0)),
  (i:54,k:"7. Consultation record ข้อที่ 6",v:audit.map(a => some0(a.cr_6)).sum(default:0)),
  (i:53,k:"7. Consultation record ข้อที่ 7",v:audit.map(a => some0(a.cr_7)).sum(default:0)),
  (i:52,k:"7. Consultation record ข้อที่ 8",v:audit.map(a => some0(a.cr_8)).sum(default:0)),
  (i:51,k:"7. Consultation record ข้อที่ 9",v:audit.map(a => some0(a.cr_9)).sum(default:0)),
  (i:49,k:"8. Anesthetic record ข้อที่ 1",v:audit.map(a => some0(a.ar_1)).sum(default:0)),
  (i:48,k:"8. Anesthetic record ข้อที่ 2",v:audit.map(a => some0(a.ar_2)).sum(default:0)),
  (i:47,k:"8. Anesthetic record ข้อที่ 3",v:audit.map(a => some0(a.ar_3)).sum(default:0)),
  (i:46,k:"8. Anesthetic record ข้อที่ 4",v:audit.map(a => some0(a.ar_4)).sum(default:0)),
  (i:45,k:"8. Anesthetic record ข้อที่ 5",v:audit.map(a => some0(a.ar_5)).sum(default:0)),
  (i:44,k:"8. Anesthetic record ข้อที่ 6",v:audit.map(a => some0(a.ar_6)).sum(default:0)),
  (i:43,k:"8. Anesthetic record ข้อที่ 7",v:audit.map(a => some0(a.ar_7)).sum(default:0)),
  (i:42,k:"8. Anesthetic record ข้อที่ 8",v:audit.map(a => some0(a.ar_8)).sum(default:0)),
  (i:41,k:"8. Anesthetic record ข้อที่ 9",v:audit.map(a => some0(a.ar_9)).sum(default:0)),
  (i:39,k:"9. Operative note ข้อที่ 1",v:audit.map(a => some0(a.on_1)).sum(default:0)),
  (i:38,k:"9. Operative note ข้อที่ 2",v:audit.map(a => some0(a.on_2)).sum(default:0)),
  (i:37,k:"9. Operative note ข้อที่ 3",v:audit.map(a => some0(a.on_3)).sum(default:0)),
  (i:36,k:"9. Operative note ข้อที่ 4",v:audit.map(a => some0(a.on_4)).sum(default:0)),
  (i:35,k:"9. Operative note ข้อที่ 5",v:audit.map(a => some0(a.on_5)).sum(default:0)),
  (i:34,k:"9. Operative note ข้อที่ 6",v:audit.map(a => some0(a.on_6)).sum(default:0)),
  (i:33,k:"9. Operative note ข้อที่ 7",v:audit.map(a => some0(a.on_7)).sum(default:0)),
  (i:32,k:"9. Operative note ข้อที่ 8",v:audit.map(a => some0(a.on_8)).sum(default:0)),
  (i:31,k:"9. Operative note ข้อที่ 9",v:audit.map(a => some0(a.on_9)).sum(default:0)),
  (i:29,k:"10. Labour record ข้อที่ 1",v:audit.map(a => some0(a.lr_1)).sum(default:0)),
  (i:28,k:"10. Labour record ข้อที่ 2",v:audit.map(a => some0(a.lr_2)).sum(default:0)),
  (i:27,k:"10. Labour record ข้อที่ 3",v:audit.map(a => some0(a.lr_3)).sum(default:0)),
  (i:26,k:"10. Labour record ข้อที่ 4",v:audit.map(a => some0(a.lr_4)).sum(default:0)),
  (i:25,k:"10. Labour record ข้อที่ 5",v:audit.map(a => some0(a.lr_5)).sum(default:0)),
  (i:24,k:"10. Labour record ข้อที่ 6",v:audit.map(a => some0(a.lr_6)).sum(default:0)),
  (i:23,k:"10. Labour record ข้อที่ 7",v:audit.map(a => some0(a.lr_7)).sum(default:0)),
  (i:22,k:"10. Labour record ข้อที่ 8",v:audit.map(a => some0(a.lr_8)).sum(default:0)),
  (i:21,k:"10. Labour record ข้อที่ 9",v:audit.map(a => some0(a.lr_9)).sum(default:0)),
  (i:19,k:"11. Rehabilitation record ข้อที่ 1",v:audit.map(a => some0(a.rr_1)).sum(default:0)),
  (i:18,k:"11. Rehabilitation record ข้อที่ 2",v:audit.map(a => some0(a.rr_2)).sum(default:0)),
  (i:17,k:"11. Rehabilitation record ข้อที่ 3",v:audit.map(a => some0(a.rr_3)).sum(default:0)),
  (i:16,k:"11. Rehabilitation record ข้อที่ 4",v:audit.map(a => some0(a.rr_4)).sum(default:0)),
  (i:15,k:"11. Rehabilitation record ข้อที่ 5",v:audit.map(a => some0(a.rr_5)).sum(default:0)),
  (i:14,k:"11. Rehabilitation record ข้อที่ 6",v:audit.map(a => some0(a.rr_6)).sum(default:0)),
  (i:13,k:"11. Rehabilitation record ข้อที่ 7",v:audit.map(a => some0(a.rr_7)).sum(default:0)),
  (i:12,k:"11. Rehabilitation record ข้อที่ 8",v:audit.map(a => some0(a.rr_8)).sum(default:0)),
  (i:11,k:"11. Rehabilitation record ข้อที่ 9",v:audit.map(a => some0(a.rr_9)).sum(default:0)),
  (i:9,k:"12. Nurses' note ข้อที่ 1",v:audit.map(a => some0(a.nn_1)).sum(default:0)),
  (i:8,k:"12. Nurses' note ข้อที่ 2",v:audit.map(a => some0(a.nn_2)).sum(default:0)),
  (i:7,k:"12. Nurses' note ข้อที่ 3",v:audit.map(a => some0(a.nn_3)).sum(default:0)),
  (i:6,k:"12. Nurses' note ข้อที่ 4",v:audit.map(a => some0(a.nn_4)).sum(default:0)),
  (i:5,k:"12. Nurses' note ข้อที่ 5",v:audit.map(a => some0(a.nn_5)).sum(default:0)),
  (i:4,k:"12. Nurses' note ข้อที่ 6",v:audit.map(a => some0(a.nn_6)).sum(default:0)),
  (i:3,k:"12. Nurses' note ข้อที่ 7",v:audit.map(a => some0(a.nn_7)).sum(default:0)),
  (i:2,k:"12. Nurses' note ข้อที่ 8",v:audit.map(a => some0(a.nn_8)).sum(default:0)),
  (i:1,k:"12. Nurses' note ข้อที่ 9",v:audit.map(a => some0(a.nn_9)).sum(default:0)),
  (i:0,k:"12. Nurses' note หัก 1 คะแนน",v:audit.map(a => some1(a.nn_sub)).sum(default:0)),
).filter(s => s.v > 0).sorted(key: s => (s.v, s.i)).rev()
#let total_oppo = oppo.map(op => op.v).sum(default:0)
// RENDER
#set text(font:"TH Sarabun New",size:16pt)
#set page(paper:"a4",margin:(x:1cm,y:1.5cm),header-ascent:5pt,footer-descent:0pt,
  header: context[#h(1fr)#counter(page).display("1/1",both:true)],
)
#h(1fr) #text(size:20pt,weight:700,align(center,[สรุปร้อยละการ Audit เวชระเบียนผู้ป่วยใน#linebreak()#audit_type Audit #is_psy #ward#linebreak()ระหว่างวันที่ #date_th(data.start) ถึงวันที่ #date_th(data.end)]))#h(1fr)
#v(-15pt)
- เวชระเบียนทั้งหมด #ipt_len เวชระเบียน
- ได้รับการสรุปเวชระเบียน #sum_len เวชระเบียน คิดเป็นร้อยละ #percent(sum_len,ipt_len) ของเวชระเบียนทั้งหมด
- ได้รับการตรวจสอบ #audit_len เวชระเบียน คิดเป็นร้อยละ #percent(audit_len,ipt_len) ของเวชระเบียนทั้งหมด
#table(columns:(35pt,1fr,80pt,80pt,80pt),stroke:.5pt,
  table.header(table_h[ลำดับ],table_h[หัวข้อ],table_h[คะแนนที่ได้],table_h[คะแนนเต็ม],table_h[ร้อยละ]),
  text_c[1],strong[Discharge summary: Dx., OP],text_c(thousands(sd_v)),text_c(thousands(sd_max)),text_c(percent(sd_v,sd_max)),
  text_c[2],strong[Discharge summary: Other],text_c(thousands(so_v)),text_c(thousands(so_max)),text_c(percent(so_v,so_max)),
  text_c[3],strong[Informed consent],text_c(thousands(ic_v)),text_c(thousands(ic_max)),text_c(percent(ic_v,ic_max)),
  text_c[4],strong[History],text_c(thousands(hx_v)),text_c(thousands(hx_max)),text_c(percent(hx_v,hx_max)),
  text_c[5],strong[Physical exam],text_c(thousands(pe_v)),text_c(thousands(pe_max)),text_c(percent(pe_v,pe_max)),
  text_c[6],strong[Progress note],text_c(thousands(pn_v)),text_c(thousands(pn_max)),text_c(percent(pn_v,pn_max)),
  text_c[7],strong[Consultation record],text_c(thousands(cr_v)),text_c(thousands(cr_max)),text_c(percent(cr_v,cr_max)),
  text_c[8],strong[Anesthetic record],text_c(thousands(ar_v)),text_c(thousands(ar_max)),text_c(percent(ar_v,ar_max)),
  text_c[9],strong[Operative note],text_c(thousands(on_v)),text_c(thousands(on_max)),text_c(percent(on_v,on_max)),
  text_c[10],strong[Labour record],text_c(thousands(lr_v)),text_c(thousands(lr_max)),text_c(percent(lr_v,lr_max)),
  text_c[11],strong[Rehabilitation record],text_c(thousands(rr_v)),text_c(thousands(rr_max)),text_c(percent(rr_v,rr_max)),
  text_c[12],strong[Nurses' note],text_c(thousands(nn_v)),text_c(thousands(nn_max)),text_c(percent(nn_v,nn_max)),
  table.cell(colspan:2,table_h[รวม]),table_h(thousands(total_v)),table_h(thousands(total_max)),table_h(percent(total_v,total_max))
)
#table(columns:(35pt,1fr,120pt),stroke:.5pt,
  table.header(table_h[ลำดับ],table_h[เกณฑ์ที่สูญเสียคะแนน],table_h[คะแนนที่สูญเสีย]),
  ..oppo.enumerate(start:1).map(((i,op)) => (text_c[#i],strong[#op.k],text_c(thousands(op.v)))).flatten(),
  table.cell(colspan:2,table_h[รวม]),table_h(thousands(total_oppo))
)