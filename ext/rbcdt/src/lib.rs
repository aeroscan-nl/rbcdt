use magnus::{define_module, function, prelude::*, Error};

fn triangulate_cdt(points: Vec<(f64, f64)>) -> Option<Vec<usize>> {
    let mut contour = (0..points.len()).collect::<Vec<_>>();
    contour.push(0);

    std::panic::catch_unwind(|| {
        cdt::triangulate_contours(&points, &[contour])
            .ok()
            .map(|result| result.into_iter().flat_map(|(a, b, c)| [a, b, c]).collect())
    })
    .ok()
    .flatten()
}

fn triangulate_earcut(points: Vec<(f64, f64)>) -> Option<Vec<usize>> {
    let mut earcut = earcut::Earcut::<f64>::new();
    let mut result = vec![];
    earcut.earcut::<usize>(points.into_iter().map(|(x, y)| [x, y]), &[], &mut result);
    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RbCDT")?;
    module.define_singleton_method("triangulate_cdt", function!(triangulate_cdt, 1))?;
    module.define_singleton_method("triangulate_earcut", function!(triangulate_earcut, 1))?;
    Ok(())
}
