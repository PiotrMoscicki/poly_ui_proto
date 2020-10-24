use super::WindowsManagerTrait;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait AppTrait {
    fn exec(&mut self) -> Result<(), String>;
    fn get_windows_manager(&mut self) -> &mut dyn WindowsManagerTrait;
}