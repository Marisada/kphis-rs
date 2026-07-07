#import "templates/utils.typ": api, get_patient_main
// PRELUDE
#let data = json("data.json")
#assert(data.id != none, message:"no 'id' in data")
#let pt = data.at("patient",default: none)
#if pt == none {pt = get_patient_main(data.id)}
#let id = if pt.opd_er_order_master_id != none {pt.opd_er_order_master_id} else {0}
#let count = data.at("count",default: none)
#if count == none {count = json(api + "opd-er/document/list-vn-id/" + data.id +  "/" + str(id))}
// PREPARED FUNCTIONS
#let label_note(l,n) = [#text(weight:700,l) #n]
#let mark(c) = if c [#align(center,sym.checkmark)] else []
#let table_h(c) = [#align(center,strong(c))]
#let pad_l(c) = [#pad(left:20pt,c)]
// RENDER
#set page(paper:"a4",margin:1cm)
#set text(font:"TH Sarabun New",size:14pt)
#set table(stroke:.5pt)
#align(center,text(size:20pt,weight:700,[เอกสารใบปะหน้า]))
#label_note([ชื่อ - สกุล : ],[#pt.pname #pt.fname #pt.lname]) #label_note([อายุ : ],[#pt.age_y ปี #pt.age_m เดือน]) #label_note([HN : ],pt.hn) #label_note([VN : ],pt.vn)
#table(columns:(45pt,60pt,65pt,65pt,65pt,1fr),
  table.header(table_h[ในแฟ้ม],table_h[ระบบ Scan],table_h[ระบบ HOSxP],table_h[ระบบ KPHIS],table_h[รูปใน KPHIS],table_h[ชื่อเอกสาร]),
  [],[],[],[],mark(count.has_scan_consent),[Informed Consent],
  [],[],[],[],mark(count.has_scan_refer_in),[ใบ Refer-In],
  [],[],mark(count.has_data_refer_out),[],mark(count.has_scan_refer_out),[ใบ Refer-Out],
  [],[],[],mark(count.has_data_er_master_id),[],[ประวัติผู้ป่วย],
  [],[],[],mark(count.has_data_med_reconciliation),[],[Med Reconciliation],
  [],[],[],mark(count.has_data_order),[],[Order],
  [],[],[],[],mark(count.has_scan_oper),[Operation Report],
  [],[],[],[],mark(count.has_scan_anes),[Anesthetic Record],
  [],[],[],[],mark(count.has_scan_labour),[Labour Record],
  [],[],[],[],mark(count.has_scan_physio),[Physiotherapy Sheet \(กายภาพบำบัด\)],
  [],[],[],[],mark(count.has_scan_culture),[ผลการเพาะเชื้อ/ชิ้นเนื้อ],
  [],[],mark(count.has_data_lab),[],[],[Laboratory Report],
  [],[],[],[],mark(count.has_scan_ekg),[Electrocardiogram Report],
  [],[],[],[],mark(count.has_scan_xray),[X-rays Report],
  [],[],[],[],mark(count.has_scan_ct),[CT scan],
  [],[],[],[],mark(count.has_scan_mri),[MRI],
  [],[],[],[],mark(count.has_scan_special),[ผลตรวจพิเศษ],
  [],[],[],mark(count.has_data_focus_list),[],[Focus List],
  [],[],[],mark(count.has_data_focus_note),[],[Nurse Notes],
  [],[],[],mark(count.has_data_index_plan),[],[Index \(Nurse Planning\)],
  [],[],[],mark(count.has_data_vital_sign),[],[Vital Sign Record],
  [],[],[],mark(count.has_data_vital_sign),[],[Graphic Record],
  [],[],[],mark(count.has_data_io),[],[Fluid Balance Summary],
  [],[],[],[],mark(count.has_scan_blood),[Blood transfusion Report],
  [],[],[],[],mark(count.has_scan_opd_card),[OPD card],
  [],[],[],[],mark(count.has_scan_insure),[ใบตรวจสอบสิทธิ์],
  [],[],[],[],mark(count.has_scan_alt_med),[บันทึกการแพทย์ทางเลือก],
  [],[],[],[],mark(count.has_scan_nutrition),[บันทึกโภชนาการ],
  [],[],[],[],mark(count.has_scan_other_sp_clinic),[Other Specified Clinical Report],
  [],[],[],[],mark(count.has_scan_others),[เอกสารอื่นๆ],
  [],[],[],[],mark(count.has_scan_finance),[เอกสารค่าใช้จ่าย],
)