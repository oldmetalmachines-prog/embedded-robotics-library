use rs_opw_kinematics::kinematics_impl::OPWKinematics;
use rs_opw_kinematics::parameters::opw_kinematics::Parameters;
use rs_opw_kinematics::kinematic_traits::{Kinematics, Joints, JOINTS_AT_ZERO};

fn main() {
    // ABB IRB 2400/10 parameters
    let parameters = Parameters {
        dof: 6,
        a1: 0.100, a2: -0.135, b: 0.000,
        c1: 0.615, c2: 0.705, c3: 0.755, c4: 0.085,
        offsets: [0.0, 0.0, -std::f64::consts::FRAC_PI_2, 0.0, 0.0, 0.0],
        sign_corrections: [1, 1, 1, 1, 1, 1],
    };
    let robot = OPWKinematics::new(parameters);
    let joints: Joints = JOINTS_AT_ZERO;

    // Forward kinematics
    let pose = robot.forward(&joints);
    println!("FK pose translation: {:?}", pose.translation);

    // Inverse kinematics
    let solutions = robot.inverse(&pose);
    println!("IK solutions found: {}", solutions.len());
}
