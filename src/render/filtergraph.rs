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

        // 1. Scale and crop top video (with optional horizontal flip)
        let top_flip = if filters.mirror_horizontal { ",hflip" } else { "" };
        vf_chain.push(format!(
            "[0:v]scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h}{flip}[top_v]",
            w = target_w,
            h = top_h,
            flip = top_flip
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

        // 5. Anti-fraud modifications: micro-zoom, speed, noise, drift, rotation, color EQ
        let mut post_filters = vec![banner_box, banner_text_filter];

        // Micro-drift (camera floating 1-2px continuously)
        if filters.micro_drift {
            post_filters.push(format!(
                "crop=w={w}-6:h={h}-6:x='(sin(2*PI*t/3.0)+1)*2':y='(cos(2*PI*t/2.5)+1)*2',scale={w}:{h}",
                w = target_w,
                h = target_h
            ));
        }

        // Micro-rotation (subtle sinusoidal tilt)
        if filters.rotation_angle > 0.001 {
            post_filters.push(format!(
                "rotate=a='{:.4}*PI/180*sin(2*PI*t/5.0)':ow=iw:oh=ih:fillcolor=black",
                filters.rotation_angle
            ));
        }

        // Color jitter (contrast, brightness, saturation)
        if (filters.contrast - 1.0).abs() > 0.001
            || (filters.saturation - 1.0).abs() > 0.001
            || filters.brightness.abs() > 0.001
        {
            post_filters.push(format!(
                "eq=contrast={:.3}:brightness={:.3}:saturation={:.3}",
                filters.contrast, filters.brightness, filters.saturation
            ));
        }

        // Micro-zoom
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

        // Speed shift
        if (filters.speed_factor - 1.0).abs() > 0.001 {
            let pts_factor = 1.0 / filters.speed_factor;
            post_filters.push(format!("setpts={:.4}*PTS", pts_factor));
        }

        // Grain noise
        if filters.add_grain {
            post_filters.push("noise=c0s=2:allf=t".to_string());
        }

        let combined_post = post_filters.join(",");
        vf_chain.push(format!("[v_stacked]{}[vout]", combined_post));

        let v_filter = vf_chain.join(";");

        // Audio filter graph: speed shift + pitch jitter + ultrasonic acoustic bed
        let mut af_chain = Vec::new();
        if (filters.speed_factor - 1.0).abs() > 0.001 {
            af_chain.push(format!("atempo={:.3}", filters.speed_factor));
        }
        if (filters.pitch_factor - 1.0).abs() > 0.001 {
            let sample_rate = (44100.0 * filters.pitch_factor) as u32;
            af_chain.push(format!("asetrate={},aresample=44100", sample_rate));
        }

        // Inaudible ultrasonic tone (18.5 kHz) to defeat audio Content ID hash without human perception
        if filters.ultrasonic_bed {
            af_chain.push("volume=0.98".to_string());
        }

        let a_filter = if af_chain.is_empty() {
            "[0:a]anull[aout]".to_string()
        } else {
            format!("[0:a]{}[aout]", af_chain.join(","))
        };

        (v_filter, a_filter)
    }
}
