use crate::online_judges::OnlineJudges;
use std::borrow::Cow;
use yew::{prelude::*, ServerRenderer};

mod olympiads;

const BASE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    %%FAVICON%%
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css" rel="stylesheet"
          integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3" crossorigin="anonymous">
    <!-- https://afeld.github.io/bootstrap-toc -->
    <link rel="stylesheet" href="https://cdn.rawgit.com/afeld/bootstrap-toc/v1.0.1/dist/bootstrap-toc.min.css"
          integrity="sha384-oJyFk/zeMJXNIGMVvmH262FT6dbSYss66WJHHgp1RlUk4/LfONQTzkAsHHwfcqat" crossorigin="anonymous">
    <style>
        a {
            text-decoration: none;
        }

        a:hover {
            text-decoration: underline;
        }

        nav[data-toggle="toc"] {
            top: 42px;
        }

        nav[data-toggle="toc"] .nav .active .nav {
            display: none;
        }
    </style>
    <title>OI Checklist</title>
</head>
<body>
%%PAGE%%
<script src="https://cdn.jsdelivr.net/npm/jquery@3.6.0/dist/jquery.min.js"
        integrity="sha256-/xUj+3OJU5yExlq6GSYGSHk7tPXikynS7ogEvDej/m4=" crossorigin="anonymous"></script>
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js"
        integrity="sha384-ka7Sk0Gln4gmtz2MlQnikT1wXgYsOg+OMhuP+IlRH9sENBO0LRn5q+8nbTov4+1p"
        crossorigin="anonymous"></script>
<script src="https://cdn.rawgit.com/afeld/bootstrap-toc/v1.0.1/dist/bootstrap-toc.min.js"
        integrity="sha384-OGf04BRlCdmgZXHhCupHT3BIkznbahjfhX8DKSIEXyU9PvSO0/8iMiiVJPcA5vi7"
        crossorigin="anonymous"></script>
<script>
    $(`[data-bs-toggle="popover"]`).each(function () {
        const hoverPopover = $(this).attr("data-bs-trigger") === "hover-popover";
        const popover = new bootstrap.Popover(this, hoverPopover ? {trigger: "manual"} : {});
        if (hoverPopover) {
            $(this).on("mouseenter", () => {
                popover.show();
                $(".popover").on("mouseleave", () => popover.hide());
            }).on("mouseleave", () => {
                setTimeout(() => {
                    if (!$(".popover:hover").length) {
                        popover.hide();
                    }
                }, 100);
            });
        }
    });
</script>
</body>
</html>
"#;

fn favicon() -> String {
    use base64::prelude::*;

    macro_rules! to_url {
        ($file:expr) => {
            format!(
                "data:image/png;base64,{}",
                BASE64_STANDARD.encode(include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/",
                    $file
                )))
            )
        };
    }

    format!(
        r#"<link rel="apple-touch-icon" sizes="180x180" href="{}">
<link rel="icon" type="image/png" sizes="32x32" href="{}">
<link rel="icon" type="image/png" sizes="16x16" href="{}">"#,
        to_url!("/apple-touch-icon.png"),
        to_url!("/favicon-32x32.png"),
        to_url!("/favicon-16x16.png")
    )
}

fn unwrapping_cmp<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    a.partial_cmp(b).unwrap()
}

fn round_score(score: f32) -> f32 {
    (score * 100.).round() / 100.
}

#[derive(Clone, PartialEq, Properties)]
struct ProgressBarProps {
    points: f32,
    total: f32,
}

#[function_component]
fn ProgressBar(props: &ProgressBarProps) -> Html {
    let percent = props.points / props.total * 100.;

    html! {
        <div class="container px-0"><div class="row align-items-center">
            <div class="col"><div class="progress">
                    <div class="progress-bar" role="progressbar"
                         style={format!("width: {percent}%")}
                         aria-valuenow={props.points.to_string()} aria-valuemin="0"
                         aria-valuemax={props.total.to_string()}></div>
            </div></div>
            <div class="col-md-auto">
                {format!("{:.0}/{:.0} points ({percent:.2}%)", props.points, props.total)}
            </div>
        </div></div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct IndexProps {
    olympiads: Vec<OlympiadProps>,
}

impl IndexProps {
    fn points_scored(&self) -> f32 {
        self.olympiads
            .iter()
            .map(OlympiadProps::points_scored)
            .sum()
    }

    fn total_points(&self) -> f32 {
        self.olympiads
            .iter()
            .map(OlympiadProps::total_points)
            .sum()
    }
}

#[function_component]
fn Index(props: &IndexProps) -> Html {
    html! {
        <main class="container">
            <div class="row">
                <h1 class="text-center pb-2 mt-4 mb-2">{"OI Checklist"}</h1>
                <hr/>
            </div>
            <div class="row mt-2 mb-4"><ProgressBar points={props.points_scored()} total={props.total_points()}/></div>
            <div class="row">
                <div class="col-sm-2 col-lg-1">
                    <nav id="toc" data-toggle="toc" class="sticky-top"></nav>
                </div>
                <div class="col-sm-10 col-lg-11">
                    {for props.olympiads.iter().map(|p| html! {
                        <>
                            <Olympiad ..p.clone() />
                        </>
                    })}
                </div>
            </div>
        </main>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct OlympiadProps {
    name: &'static str,
    seasons: Vec<SeasonProps>,
}

impl OlympiadProps {
    fn points_scored(&self) -> f32 {
        self.seasons
            .iter()
            .map(|s| {
                s.problems
                    .iter()
                    .map(|p| p.best_score().unwrap_or(0.))
                    .sum::<f32>()
            })
            .sum()
    }

    fn total_points(&self) -> f32 {
        self.seasons
            .iter()
            .map(|s| s.problems.len() as f32 * 100.)
            .sum()
    }
}

#[function_component]
fn Olympiad(props: &OlympiadProps) -> Html {
    html! {
        <>
            <h2 class="pb-2 mb-2">{&props.name}</h2>
            <div class="mt-2 mb-4">
                <ProgressBar points={props.points_scored()} total={props.total_points()} />
            </div>
            <table class="table table-bordered">
                <tbody>
                    {for props.seasons.iter().map(|p| html! {
                        <Season ..p.clone() />
                    })}
                </tbody>
            </table>
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct SeasonProps {
    name: &'static str,
    problems: Vec<ProblemProps>,
}

#[function_component]
fn Season(props: &SeasonProps) -> Html {
    html! {
        <tr>
            <th class="bg-light">{&props.name}</th>
            {for props.problems.iter().map(|p| html! {
                <Problem ..p.clone() />
            })}
        </tr>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ProblemProps {
    short_name: Cow<'static, str>,
    long_name: Cow<'static, str>,
    locations: Vec<ProblemLocation>,
}

impl ProblemProps {
    fn best_score(&self) -> Option<f32> {
        self.locations
            .iter()
            .filter_map(|s| s.best_score)
            .max_by(unwrapping_cmp)
    }
}

#[derive(Clone, PartialEq)]
struct ProblemLocation {
    name: &'static str,
    url: &'static str,
    best_score: Option<f32>,
}

#[function_component]
fn Problem(props: &ProblemProps) -> Html {
    fn score_to_style(score: Option<f32>) -> Option<String> {
        score.map(|s| {
            format!(
                "background-color: hsl({}, 70%, 80%)",
                (s / 100. * 120.).clamp(0., 120.)
            )
        })
    }

    let content = props
        .locations
        .iter()
        .map(|l| {
            format!(
                "<li><a href={} target='_blank'>{}</a>{}</li>",
                &l.url,
                &l.name,
                match l.best_score {
                    Some(score) => format!(": {} points", round_score(score)),
                    None => "".to_string(),
                }
            )
        })
        .fold("".to_string(), |mut a, b| {
            a.push_str(&b);
            a
        });
    let content = if content.is_empty() {
        "Nowhere to submit".to_string()
    } else {
        content
    };

    let suffix = match props.best_score() {
        Some(score) if score < 100. => format!(" ({})", round_score(score)),
        _ => "".to_string(),
    };

    html! {
        <td style={score_to_style(props.best_score())}>
            <a href={props.locations.first().map(|l| l.url)} target="_blank"
            title={props.long_name.to_string()} data-bs-placement="top"
            data-bs-toggle="popover" data-bs-trigger="hover-popover"
            data-bs-html="true" data-bs-content={content}>
                {props.short_name.to_string() + &suffix}
            </a>
        </td>
    }
}

pub async fn render_checklist(ojs: OnlineJudges) -> String {
    let page = ServerRenderer::<Index>::with_props(move || IndexProps {
        olympiads: vec![
            olympiads::ioi::to_olympiad(&ojs),
            olympiads::joisc::to_olympiad(&ojs),
            olympiads::joi_open::to_olympiad(&ojs),
            olympiads::apio::to_olympiad(&ojs),
            olympiads::ceoi::to_olympiad(&ojs),
            olympiads::cnoi::to_olympiad(&ojs),
            olympiads::cco::to_olympiad(&ojs),
            olympiads::ccc::to_olympiad(&ojs),
            olympiads::boi::to_olympiad(&ojs),
        ],
    })
    .render()
    .await;

    BASE_HTML
        .replace("%%FAVICON%%", &favicon())
        .replace("%%PAGE%%", &page)
}
