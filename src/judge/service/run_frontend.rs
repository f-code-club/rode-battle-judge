use crate::{
    judge::repository::model::{Problem, Submission},
    shared::Storage,
};

pub async fn run_frontend(
    storage: &Storage,
    sub: Submission,
    prob: Problem,
) -> color_eyre::Result<f32> {
    let expected = storage.download(prob.content).await?;
    let expected = image::load_from_memory(&expected)?;

    let code = sub.code;
    let rendered = html_renderer::render(&code, expected.width(), expected.height()).await?;
    let rendered = image::load_from_memory(&rendered)?;

    let score =
        image_compare::rgb_hybrid_compare(&expected.into_rgb8(), &rendered.into_rgb8())?.score;

    Ok(score as f32)
}
