# KPHIS `dump-builder`

Tool for building binary dump data for
- TDRG Grouper (base on v.6.3.3)
- Highlight Syntax and Theme (for `syntect` crate)
- ICD10-ClaML
- ICD10-Index (Book 3)

Aim of this crate is to "NOT parse raw data to rust struct at runtime" so we parse raw data to struct and parse struct to memory dump file.
Client only parse memory dump file back to rust struct and use it.

Memory dump files are
- crates/kphis-drg-worker/dump/grouper.dump
- crates/kphis-ui-core/src/highlight/syntax_set.dump
- crates/kphis-ui-core/src/highlight/theme.dump

## How
- Dump all data by running `test_dump_all()` in lib.rs

## NOTE
- We comment our code with actual page of the pdf book, NOT the page number printed at the corner of page 
- Memory dump files NOT in .gitignore file, so you do not need to run this crate
- Run this crate only after updated `syntect` crate or DRG raw files

## Prepare CSV database sources
We store database from various places to `/raw-xxx` as comma-seperated `.csv` files
> `.dbf` can open and save-as csv with [LibreOffice](https://www.libreoffice.org)

### raw-grouper
- `dagger-asterisk.csv`
    1. Use "Dagger","Asterisk","PDx","MDC" as 1st line header
    2. Copy/Paste from https://www.tcmc.or.th/download-tcmc > คู่มือการจัดกลุ่ม TDRG V 6.3.3 > คู่มือการจัดกลุ่ม TDRG V 6.3 เล่ม 1 > Appendix A1: Dagger-asterisk substitution
    3. Remove repeated header 
    4. Replace ` ` with `,`
- `i10.csv` 
    1. Download from https://www.tcmc.or.th/download-tcmc > โปรแกรม TDRG Grouper หลายรายพร้อมกัน (TGRP) > โปรแกรม TDRG V.6305 Grouper > c63i10.dbf
    2. Save as csv
- `i10vx.csv` 
    1. Download from https://www.tcmc.or.th/download-tcmc > โปรแกรม TDRG Grouper หลายรายพร้อมกัน (TGRP) > โปรแกรม TDRG V.6305 Grouper > c63i10vx.dbf
    2. Save as csv
    3. Add `,"TM,L"` to header
    4. Add `,เท็จ` to the end of all line except header
    4. Edit ` [TM],เท็จ` and ` [TM]",เท็จ` to `,จริง` and `",จริง`
- `proc.csv`
    1. Download from https://www.tcmc.or.th/download-tcmc > โปรแกรม TDRG Grouper หลายรายพร้อมกัน (TGRP) > โปรแกรม TDRG V.6305 Grouper > c63proc.dbf
    2. Save as csv
- `i9vx.csv` 
    1. Download from https://www.tcmc.or.th/download-tcmc > โปรแกรม TDRG Grouper หลายรายพร้อมกัน (TGRP) > โปรแกรม TDRG V.6305 Grouper > c63i9vx.dbf
    2. Save as csv
- `drg.csv` 
    1. Download from https://www.tcmc.or.th/download-tcmc > โปรแกรม TDRG Grouper หลายรายพร้อมกัน (TGRP) > โปรแกรม TDRG V.6305 Grouper > c63drg.dbf
    2. Save as csv
- `ccex.csv` 
    1. Download from https://www.tcmc.or.th/download-tcmc > โปรแกรม TDRG Grouper หลายรายพร้อมกัน (TGRP) > โปรแกรม TDRG V.6305 Grouper > c63ccex.dbf
    2. Save as csv
- `book2.txt`, `mdc-pdc.csv`, `mdc-ppdc.csv`, `mdc-ax.csv`, `mdc-pax.csv` and `dc-pcl-drg.csv` from Book 2
    1. Create book2.txt
    2. Copy/Paste all of Book2 to book2.txt
    3. Run `test_book2_parser` in raw_parser.rs
- `dcl-14.txt`, `dcls.csv` and `dcl-eq.csv`
    1. Create dcl-14.txt
    2. Copy/Paste all of DCL Book 1-4
    3. Replace `\n= ` with `=`
    4. Run `test_dcl_from_raw` in raw_parser.rs

### raw-highlight
- read [syntect](https://github.com/trishume/syntect) at [dump.rs](https://github.com/trishume/syntect/blob/master/src/dumps.rs) for more detail

### raw-icd-tm
- `icd-10-tm2016-20210805.csv`
    1. Download from http://thcc.or.th/index.php > Database ICD-10-TM version 2019 > http://www.thcc.or.th/download/icd/Data%20TM2016-update_5aug2021.xlsx
    2. Replace `\n"` with `"`
    2. Save as csv

### raw-icd-who
- ICD10 WHO ClaML:
    1. Download from https://icdcdn.who.int/icd10/index.html ICD-10 2016 version (ClaML file), save as `icd102016en.xml`
    2. Parse `en` version by
        - parse_xml_en(): generate Struct
        - remove_unused_rubrics(): remove unsupported Rubric
        - fixed_references(): fix reference's label to be a code tag
        - fixed_modified_by(): fix modifiedBy
        - fixed_exclude_modifier(): fix excludeModifier
        - chain_modifier(): recursion of superClass to get a last child's modifiedBy
        - add_class_from_modifier(): create Classes from modifiedBy
    3. Update to `tm` version
        - Copy `icd102016en.xml` to `icd102016en-tm.xml`
        - test_diff_from_grouper(): test by parse `en` version + compare with grouper's `vx` data and create debug files for editing (at /debug/claml-vx-diff/)
            * `2-vx-noex-to-xml.xml: claMl file for copy/paste to `icd102016en-tm.xml`
            * `2-vx-noex-with-nested.txt`: `vx` that not use direct parent, maybe modifiedBy or No direct parent in `vx`
            * `2-vx-noex-with-special.txt`: need manual edit
            * `2-vx-noex-with-mod-by-1.txt`: need manual edit by crate Modifer and ModifierClass + add modifiedBy
            * `2-vx-noex-with-mod-by-2.txt`: need manual edit by crate Modifer and ModifierClass + add modifiedBy
            * `2-vx-noex-with-no-parent.txt`: need manual edit by create missed parent class
        - add more `inclusion` and `exclusion` rubric of TM's class with ICD10-TM book by
            * Flipbook : http://www.thcc.or.th/ebook1/2016/mobile/index.html
            * Image URL : http://www.thcc.or.th/ebook1/2016/files/mobile/1.jpg
            * Our HTML : at `raw-icd-who/index.html`
- ICD10 WHO Alphabetic index (Book3):
    1. Download from https://www.chi.or.th/Drg/ICD10_2016_WHO.html
    2. Create `icd102016en-book3-raw.txt`
    3. Copy/Paste all of PDF content to `icd102016en-book3-raw.txt`
    4. Remove intro, notes and `Abortion` part

## Update `raw-grouper`'s `i10vx` from  with `raw-icd-tm`'s `icd-10-tm2016-20210805.csv`
by running `test_diff_i10vx_tm()` with `verbose = true;`
- Different in `K091-` (change `csv` to `i10vx`, not add items)
    * `i10vx`
    K0910,"Developmental (nonodontogenic) cysts of oral region, globulomaxillary"
    K0911,"Developmental (nonodontogenic) cysts of oral region, median palatal"
    K0912,"Developmental (nonodontogenic) cysts of oral region, nasopalatine [incisive canal]"
    K0913,"Developmental (nonodontogenic) cysts of oral region, palatine papilla"
    * `csv`
    K0910,"Other specified developmental (nonodontogenic) cysts of oral origin, nasolabial [nasoalveolar] cyst"
    K0912,"Other specified developmental (nonodontogenic) cysts of oral origin, nasopalatine duct [incisive canal]"
- Not found in `i10vx` (DO NOTHING)
    read [TDS6307_EXCLUDES.md](TDS6307_EXCLUDES.md) for more details

- Different description in `M8980` (change `csv` to `i10vx`, not add items)
    * `i10vx`
    M8980,"Other specified disorders of bone, multiple sites"
    * `csv`
    M8980,"Infantile cortical hyperostoses"

### 4th `place` and 5th `activities` code different
- Read [update](references\drg-grouper\ICD10ExternalCauseCodeChanges.pdf) 2019-07-01
1. `W26`
    - the book has 4th code
    - `update` use only `W26.0`, `W26.8` and `W26.9`
    - `i10vx` discard the book's 4rd code, apply `place` and `activities` code  ex. `W26.00`
    - `csv` discard the book's 4rd code, apply `place` and `activities` code  ex. `W26.00`
2. `X34`
    - the book has 4th code
    - `update` use only `X34.0`, `X34.1`, `X34.8` and `X34.9` 
    - `i10vx` discard the book's 4rd code but naming as the book's 4rd code
    - `csv` discard the book's 4rd code
3. `Y06` and `Y07`
    - the book has 4th code
    - `update` not apply `place` code
    - `i10vx` not apply `place` code
    - `csv` apply all ex. `Y06.000`
4. `X59`
    - the book has 4th code
    - `update` use only `X59.0` and `X59.9`
    - `i10vx` not apply both `place` and `activities` code
    - `csv` apply all



---
This crate is part of the [KPHIS](https://github.com/Marisada/kphis) project.
