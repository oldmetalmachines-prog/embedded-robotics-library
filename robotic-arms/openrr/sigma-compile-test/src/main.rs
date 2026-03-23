// Verifies the `k` kinematics crate (the core of openrr) compiles
// without ROS system dependencies, in a headless embedded environment.
use k::*;

fn main() {
    // Build a 3-DOF arm: root -> shoulder -> elbow -> wrist
    let j0 = k::NodeBuilder::<f64>::new()
        .name("shoulder")
        .joint_type(k::JointType::Rotational { axis: nalgebra::Vector3::z_axis() })
        .translation(nalgebra::Translation3::new(0.0, 0.0, 0.3))
        .finalize();
    let j1 = k::NodeBuilder::<f64>::new()
        .name("elbow")
        .joint_type(k::JointType::Rotational { axis: nalgebra::Vector3::y_axis() })
        .translation(nalgebra::Translation3::new(0.0, 0.0, 0.3))
        .finalize();
    let j2 = k::NodeBuilder::<f64>::new()
        .name("wrist")
        .joint_type(k::JointType::Rotational { axis: nalgebra::Vector3::y_axis() })
        .translation(nalgebra::Translation3::new(0.0, 0.0, 0.3))
        .finalize();

    j0.set_parent(&j1);
    j1.set_parent(&j2);

    let chain = k::Chain::from_root(j2);
    println!("DOF: {}", chain.dof());
    chain.set_joint_positions_clamped(&vec![0.0f64; chain.dof()]);
    let transforms = chain.update_transforms();
    println!("Transforms computed: {}", transforms.len());
    println!("End effector: {:?}", transforms.last().unwrap().translation.vector);
}
