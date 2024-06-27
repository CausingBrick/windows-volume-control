use windows_volume_control::AudioController;

fn main() {
    unsafe {
        let mut controller = AudioController::init(None);
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        let sesstion_names = controller.get_all_session_names();
        println!("{:?}", sesstion_names);
        let master_session = controller.get_session_by_name("master".to_string());
        println!("{:?}", master_session.unwrap().getVolume());
    }
}
