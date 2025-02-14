use crate::gravity::Gravity;
use crate::quaternion::Quaternion;

#[derive(Debug, Copy, Clone)]
pub struct YawPitchRoll {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl YawPitchRoll {}

impl From<Quaternion> for YawPitchRoll {
    fn from(q: Quaternion) -> Self {
        let gravity = Gravity::from(q);
        // yaw: (about Z axis)
        let yaw = libm::atan2(
            (2.0 * q.w * q.w + 2.0 * q.x * q.x - 1.0) as f64,
            (2.0 * q.x * q.y - 2.0 * q.w * q.z) as f64,
        );
        // pitch: (nose up/down, about Y axis)
        let mut pitch = libm::atan2(
            libm::sqrt((gravity.y * gravity.y + gravity.z * gravity.z) as f64),
            gravity.x as f64,
        );
        // roll: (tilt left/right, about X axis)
        let roll = libm::atan2(gravity.z as f64, gravity.y as f64);

        if gravity.z < 0.0 {
            if pitch > 0.0 {
                pitch = core::f64::consts::PI - pitch;
            } else {
                pitch = -core::f64::consts::PI - pitch;
            }
        }

        Self {
            yaw: yaw as f32,
            pitch: pitch as f32,
            roll: roll as f32,
        }
    }
}

// uint8_t MPU6050_6Axis_MotionApps20::dmpGetYawPitchRoll(float *data, Quaternion *q, VectorFloat *gravity) {
//     // yaw: (about Z axis)
//     data[0] = atan2(2*q.x*q.y - 2*q.w*q.z, 2*q.w*q.w + 2*q.x*q.x - 1);
//     // pitch: (nose up/down, about Y axis)
//     data[1] = atan2(gravity.x , sqrt(gravity.y*gravity.y + gravity.z*gravity.z));
//     // roll: (tilt left/right, about X axis)
//     data[2] = atan2(gravity.y , gravity.z);
//     if (gravity.z < 0) {
//         if(data[1] > 0) {
//             data[1] = PI - data[1];
//         } else {
//             data[1] = -PI - data[1];
//         }
//     }
//     return 0;
// }
