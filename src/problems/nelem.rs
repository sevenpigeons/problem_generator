use crate::util::Question;

//pub static THREE_ELEM:Question<impl Fn(f32, f32) -> f32>  = Question {
//
pub fn build_nelem() -> Question<impl Fn(&[f32]) -> f32> {
Question{
    subject: crate::util::Subject::Math,
    theme: crate::util::Theme::Math(crate::util::MathTheme::PartialDerivatives),
    text: "f(x,y) = x^{} + x^ {} *y^{} - {}*y^{}

f_x( {} , {} ) = ?\n",
    var_conditions: &[(1,5),(1,5),(1,5),(1,5),(1,5),(2,4),(3,5)],
    ans_expression: | v: &[f32] | -> f32 {
        v[0]*(v[5].powf(&v[0]-1.0)) + v[1]*v[5].powf(&v[1]-1.0)*v[6].powf(v[6]) - v[3]*v[6].powf(v[4])
    }}
}

// f_y( {} , {} ) = ?
// giving answer gets complicated with closures for now lets keep a single answer
// x => 5, y=>6
