use chart_js_rs::{
    Annotation, Annotations, ChartExt, ChartInteraction, ChartOptions, ChartPlugins, ChartScale, Dataset, DatasetDataExt, DatasetIterExt, DisplayFormats, FnWithArgs, Grid, LabelAnnotation,
    LegendLabel, LineAnnotation, LineAnnotationType, PluginLegend, PluginZoom, ScaleAdapters, ScaleAdaptersDate, ScaleTicks, ScaleTime, Title, TooltipCallbacks, TooltipPlugin, XYDataset, ZoomPan,
    ZoomPinchOptions, ZoomWheelOptions, ZoomZoom, scatter::Scatter,
};
use dominator::{Dom, html};
use std::{collections::HashMap, rc::Rc};
use time::{PrimitiveDateTime, Time};

use kphis_model::vital_sign::{VitalSign, VsMode};
use kphis_util::datetime::{date_8601, datetime_ts, js_now};

use kphis_ui_core::class;

pub fn render(data: &[Rc<VitalSign>], start_vs_date: &str, end_vs_date: &str, vs_mode: VsMode, zoomable: bool) -> Dom {
    let day_ts = 24 * 3600 * 1000;
    let now = datetime_ts(&js_now());
    let start = date_8601(start_vs_date).map(|d| datetime_ts(&PrimitiveDateTime::new(d, Time::MIDNIGHT))).unwrap_or(now);
    let end = date_8601(end_vs_date).map(|d| datetime_ts(&PrimitiveDateTime::new(d, Time::MIDNIGHT))).unwrap_or(now);
    let ts_diff = end - start;

    let min = start;
    let max = end + day_ts;

    let time_unit = match ts_diff / day_ts {
        ..=3 => "hour",
        4..=30 => "day",
        31.. => "month",
    };
    let is_lr = matches!(vs_mode, VsMode::Labour);
    let is_psy = matches!(vs_mode, VsMode::Psychia);

    let data = Dataset::new().datasets([
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.bt.map(|bt| (datetime_ts(&vs.vs_datetime), bt)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y1")
            .label("BT")
            .background_color("dodgerblue")
            .border_color("dodgerblue")
            .point_radius(5)
            .point_hover_radius(5)
            .show_line(true)
            .hidden(is_lr || is_psy),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.pr.map(|pr| (datetime_ts(&vs.vs_datetime), pr)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y2")
            .label("PR")
            .background_color("red")
            .border_color("red")
            .point_radius(5)
            .point_hover_radius(5)
            .show_line(true)
            .hidden(is_lr || is_psy),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.rr.map(|rr| (datetime_ts(&vs.vs_datetime), rr)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y3")
            .label("RR")
            .background_color("green")
            .border_color("green")
            .point_style("rect")
            .point_radius(5)
            .point_hover_radius(5)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.sbp.map(|sbp| (datetime_ts(&vs.vs_datetime), sbp)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y2")
            .label("SBP")
            .background_color("white")
            .border_color("deeppink")
            .border_dash([3, 2])
            .point_style("triangle")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(2)
            .rotation(180)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.map.map(|map| (datetime_ts(&vs.vs_datetime), map)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y2")
            .label("MAP")
            .border_color("deeppink")
            .border_dash([1, 3])
            .point_style("crossRot")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(2)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.dbp.map(|dbp| (datetime_ts(&vs.vs_datetime), dbp)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y2")
            .label("DBP")
            .background_color("white")
            .border_color("deeppink")
            .border_dash([3, 2])
            .point_style("triangle")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(2)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.sat.map(|sat| (datetime_ts(&vs.vs_datetime), sat)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y3")
            .label("O\u{2082} Sat")
            .background_color("orange")
            .border_color("orange")
            .point_radius(5)
            .point_hover_radius(5)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.lr_cer.as_ref().and_then(|s| s.parse::<u32>().ok().map(|lr_cer| (datetime_ts(&vs.vs_datetime), lr_cer))))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y5")
            .label("Cervix")
            .background_color("#868A08")
            .border_color("#868A08")
            .point_style("crossRot")
            .point_radius(10)
            .point_hover_radius(10)
            .point_border_width(3)
            .show_line(true)
            .hidden(!is_lr || is_psy),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.lr_eff.map(|lr_eff| (datetime_ts(&vs.vs_datetime), lr_eff)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y3")
            .label("Effacement")
            .background_color("#81F7D8")
            .border_color("#81F7D8")
            .point_radius(5)
            .point_hover_radius(5)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.lr_sta.map(|lr_sta| (datetime_ts(&vs.vs_datetime), (lr_sta as i32) - 3)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y4")
            .label("Station")
            .background_color("white")
            .border_color("#D0A9F5")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(3)
            .show_line(true)
            .hidden(!is_lr || is_psy),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.lr_fsh.map(|lr_fsh| (datetime_ts(&vs.vs_datetime), lr_fsh)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y2")
            .label("FHS")
            .background_color("red")
            .border_color("red")
            .point_style("triangle")
            .point_radius(8)
            .point_hover_radius(8)
            .show_line(true)
            .hidden(!is_lr || is_psy),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| {
                        vs.amphetamine_awq
                            .as_ref()
                            .and_then(|concat| concat.split(',').nth(0))
                            .and_then(|s| s.parse::<u8>().ok())
                            .map(|u| (datetime_ts(&vs.vs_datetime), u))
                    })
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y3")
            .label("AWQv2")
            .background_color("#868A08")
            .border_color("#868A08")
            .point_radius(5)
            .point_hover_radius(5)
            .show_line(true)
            .hidden(!is_psy || is_lr),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| {
                        vs.amphetamine_awq
                            .as_ref()
                            .and_then(|concat| concat.split(',').nth(1))
                            .and_then(|s| s.parse::<u8>().ok())
                            .map(|u| (datetime_ts(&vs.vs_datetime), u))
                    })
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y5")
            .label("AWQ-H")
            .border_color("orange")
            .border_dash([3, 2])
            .point_style("crossRot")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(2)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| {
                        vs.amphetamine_awq
                            .as_ref()
                            .and_then(|concat| concat.split(',').nth(2))
                            .and_then(|s| s.parse::<u8>().ok())
                            .map(|u| (datetime_ts(&vs.vs_datetime), u))
                    })
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y5")
            .label("AWQ-A")
            .border_color("cyan")
            .border_dash([3, 2])
            .point_style("crossRot")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(2)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| {
                        vs.amphetamine_awq
                            .as_ref()
                            .and_then(|concat| concat.split(',').nth(3))
                            .and_then(|s| s.parse::<u8>().ok())
                            .map(|u| (datetime_ts(&vs.vs_datetime), u))
                    })
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y5")
            .label("AWQ-R")
            .border_color("grey")
            .border_dash([3, 2])
            .point_style("crossRot")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(2)
            .show_line(true)
            .hidden(true),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| {
                        vs.aggression_oas
                            .as_ref()
                            .and_then(|concat| concat.split(',').nth(0))
                            .and_then(|s| s.parse::<u8>().ok())
                            .map(|u| (datetime_ts(&vs.vs_datetime), u))
                    })
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y5")
            .label("OAS")
            .background_color("#D0A9F5")
            .border_color("#D0A9F5")
            .point_radius(5)
            .point_hover_radius(5)
            .show_line(true)
            .hidden(!is_psy || is_lr),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.motivation_scale.map(|mot| (datetime_ts(&vs.vs_datetime), mot)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y5")
            .label("Motivation")
            .background_color("white")
            .border_color("green")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(3)
            .show_line(true)
            .hidden(!is_psy || is_lr),
        XYDataset::new()
            .data(
                data.iter()
                    .filter_map(|vs| vs.craving_scale.map(|crav| (datetime_ts(&vs.vs_datetime), crav)))
                    .into_data_iter()
                    .unsorted_to_dataset_data(),
            )
            .y_axis_id("y5")
            .label("Craving")
            .background_color("white")
            .border_color("deeppink")
            .point_radius(8)
            .point_hover_radius(8)
            .point_border_width(3)
            .show_line(true)
            .hidden(!is_psy || is_lr),
    ]);

    let annotation: HashMap<String, Annotation> = HashMap::from([(
        String::from("line"),
        Annotation::Line(
            LineAnnotation::new()
                .annotation_type(LineAnnotationType)
                .draw_time("beforeDatasetsDraw")
                .mode("horizontal")
                .scale_id("y1")
                .value(37)
                .border_color("crimson")
                .border_width(2)
                .label(LabelAnnotation::new().display(true).position("end").content(["BT 37°C"]).color("crimson").background_color("white")),
        ),
    )]);

    let mut plugins = ChartPlugins::new()
        .legend(PluginLegend::new().labels(LegendLabel::new().use_point_style(true)).position("bottom"))
        .tooltip(
            TooltipPlugin::new()
                .use_point_style(true)
                .callbacks(TooltipCallbacks::new().title(FnWithArgs::new().args(["tooltipItems"]).rust_closure(|_ctx| "รายการ: (วันที่ประเมิน, ผลลัพธ์)".into()))),
        )
        .annotation(Annotations::new().annotations(annotation));
    if zoomable {
        plugins = plugins.zoom(
            PluginZoom::new().pan(ZoomPan::new().enabled(true).mode("xy")).zoom(
                ZoomZoom::new()
                    .mode("xy")
                    .wheel(ZoomWheelOptions::new().enabled(true).speed(0.1))
                    .pinch(ZoomPinchOptions::new().enabled(true)),
            ),
        );
    }

    let scales: HashMap<String, ChartScale> = HashMap::from([
        (
            String::from("x"),
            ChartScale::new()
                .scale_type("time")
                .adapters(ScaleAdapters::new().date(ScaleAdaptersDate::new().output_calendar("buddhist")))
                .time(
                    ScaleTime::new()
                        .unit(time_unit)
                        .tooltip_format("d MMM yyyy HH:mm")
                        .display_formats(DisplayFormats::new().minute("HH:mm").hour("(d MMM) HH:mm").day("d MMM")),
                )
                .position("top")
                .min(min)
                .max(max)
                .grid(Grid::new().color("green"))
                .ticks(ScaleTicks::new().color("green")),
        ),
        (
            String::from("y1"),
            ChartScale::new()
                .title(Title::new().display(true).text("Body Temp (°C)").color("dodgerblue"))
                .suggested_min(35)
                .suggested_max(41)
                .grid(Grid::new().color("dodgerblue"))
                .ticks(ScaleTicks::new().color("dodgerblue")),
        ),
        (
            String::from("y2"),
            ChartScale::new()
                .title(Title::new().display(true).text("Pulse Rate, Blood Pressure, FHS").color("red"))
                .suggested_min(40)
                .suggested_max(160)
                .grid(Grid::new().color("red"))
                .ticks(ScaleTicks::new().color("red")),
        ),
        (
            String::from("y3"),
            ChartScale::new()
                .title(Title::new().display(true).text("O\u{2082} Sat, RR, Effacement, AWQv2"))
                .position("right")
                .suggested_min(0)
                .suggested_max(100),
        ),
        (
            String::from("y4"),
            ChartScale::new()
                .title(Title::new().display(true).text("Station"))
                .position("right")
                .suggested_min(-2)
                .suggested_max(2)
                .reverse(true),
        ),
        (
            String::from("y5"),
            ChartScale::new()
                .title(Title::new().display(true).text("Cervix, AWQ-HAR, OAS, Motivation, Craving"))
                .position("right")
                .suggested_min(0)
                .suggested_max(10),
        ),
    ]);

    let options = ChartOptions::new()
        .locale("th")
        .span_gaps(true)
        .aspect_ratio(2)
        .maintain_aspect_ratio(false)
        .responsive(true)
        .interaction(ChartInteraction::new().mode("index").intersect(false))
        .plugins(plugins)
        .scales(scales);

    let chart = Scatter::new("canvas").data(data).options(options);

    html!("canvas", {
        .attr("id", "canvas")
        .class(class::ROUND_WHITE)
        .after_inserted(move |_| {
            chart.into_chart().render()
        })
    })
}
