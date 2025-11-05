//! Entry point to Rust code.

use ftc::{ftc, hardware::DcMotor};

// /// Example linear op mode.
// #[ftc(
//     name = "Example: My Linear Op Mode",
//     linear,
//     teleop,
//     group = "Example",
// )]
// fn my_linear_op_mode(ctx: &ftc::FtcContext) {
//     // equivalent to hardwareMap.get(DcMotor.class, "motor") in Java:
//     // let motor = ctx.hardware().get::<DcMotor>("motor");
//     // motor.set_direction(ftc::hardware::Direction::Forward);

//     // ctx.telemetry().add_data("Status", "Initalized");
//     // ctx.telemetry().update();

//     // ctx.wait_for_start();

//     // // ctx.running() instead of opModeIsActive()

//     // motor.set_power(0.5);
//     // ctx.sleep_s(2.0);
//     // motor.set_power(0.0);
// }

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_firstinspires_ftc_teamcode_MyLinearOpMode_runOpMode<
    'local,
>(
    mut unowned_env: ::ftc::jni::EnvUnowned<'local>,
    this: ::ftc::jni::objects::JObject<'local>,
) {
    let outcome = unowned_env.with_env_no_catch(|env| -> ::ftc::jni::errors::Result<_> {
        Ok(())
    });
}