use crate::antifraud::AppliedFilters;
use crate::templates::schema::LayoutTemplate;
use std::path::Path;

pub struct FilterGraphBuilder;

impl FilterGraphBuilder {
    pub fn build(
        template: &LayoutTemplate,
        filters: &AppliedFilters,
        font_path: Option<&Path>,
    ) -> (String, String) {
        let target_w = template.target_width;
        let target_h = template.target_height;
        let top_h = (target_h as f32 * template.top_ratio) as u32;
        let bottom_h = target_h.saturating_sub(top_h);

        let banner_h = template.banner.height_px;
        let banner_text = template.banner.text.replace('\'', "\\'").replace(':', "\\:");

        let mut vf_chain = Vec::new();

        // 1. Scale and crop top video
        vf_chain.push(format!(
            "[0:v]scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h}[top_v]",
            w = target_w,
            h = top_h
        ));

        // 2. Scale and crop bottom retention video
        vf_chain.push(format!(
            "[1:v]scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h}[bottom_v]",
            w = target_w,
            h = bottom_h
        ));

        // 3. Stack vertically
        vf_chain.push("[top_v][bottom_v]vstack=inputs=2[v_stacked]".to_string());

        // 4. Banner background & text
        let banner_y = (target_h / 2).saturating_sub(banner_h / 2);
        let banner_box = format!(
            "drawbox=x=0:y={y}:w={w}:h={h}:color=black@0.85:t=fill",
            y = banner_y,
            w = target_w,
            h = banner_h
        );

        let banner_text_filter = if let Some(fp) = font_path {
            format!(
                "drawtext=fontfile='{font}':text='{text}':fontsize={size}:fontcolor=white:x=(w-text_w)/2:y={y}+(({h}-text_h)/2)",
                font = fp.display(),
                text = banner_text,
                size = template.banner.font_size,
                y = banner_y,
                h = banner_h
            )
        } else {
            format!(
                "drawtext=text='{text}':fontsize={size}:fontcolor=white:x=(w-text_w)/2:y={y}+(({h}-text_h)/2)",
                text = banner_text,
                size = template.banner.font_size,
                y = banner_y,
                h = banner_h
            )
        };

        // 5. Anti-fraud modifications: micro-zoom, speed, noise
        let mut post_filters = vec![banner_box, banner_text_filter];

        if (filters.zoom_factor - 1.0).abs() > 0.001 {
            let zw = ((target_w as f32) * filters.zoom_factor) as u32;
            let zh = ((target_h as f32) * filters.zoom_factor) as u32;
            post_filters.push(format!(
                "scale={zw}:{zh},crop={w}:{h}",
                zw = zw,
                zh = zh,
                w = target_w,
                h = target_h
            ));
        }

        if (filters.speed_factor - 1.0).abs() > 0.001 {
            let pts_factor = 1.0 / filters.speed_factor;
            post_filters.push(format!("setpts={:.4}*PTS", pts_factor));
        }

        if filters.add_grain {
            post_filters.push("noise=c0s=2:allf=t".to_string());
        }

        let combined_post = post_filters.join(",");
        vf_chain.push(format!("[v_stacked]{}[vout]", combined_post));

        let v_filter = vf_chain.join(";");

        // Audio filter graph: speed shift + pitch jitter
        let mut af_chain = Vec::new();
        if (filters.speed_factor - 1.0).abs() > 0.001 {
            af_chain.push(format!("atempo={:.3}", filters.speed_factor));
        }
        if (filters.pitch_factor - 1.0).abs() > 0.001 {
            let sample_rate = (44100.0 * filters.pitch_factor) as u32;
            af_chain.push(format!("asetrate={},aresample=44100", sample_rate));
        }

        let a_filter = if af_chain.is_empty() {
            "[0:a]anull[aout]".to_string()
        } else {
            format!("[0:a]{}[aout]", af_chain.join(","))
        };

        (v_filter, a_filter)
    }
}
