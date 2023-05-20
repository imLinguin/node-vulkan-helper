use ash::{vk, Entry};
use neon::prelude::*;

const APP_NAME: &str = "Heroic\0";

fn get_instance_version(mut cx: FunctionContext) -> JsResult<JsArray> {
    let entry = unsafe { Entry::load() }.expect("Failed to load vulkan");
    if let Ok(Some(version)) = entry.try_enumerate_instance_version() {
        let major = cx.number(vk::api_version_major(version));
        let minor = cx.number(vk::api_version_minor(version));
        let patch = cx.number(vk::api_version_patch(version));

        let array = cx.empty_array();
        array.set(&mut cx, 0, major)?;
        array.set(&mut cx, 1, minor)?;
        array.set(&mut cx, 2, patch)?;
        Ok(array)
    } else {
        Ok(cx.empty_array())
    }
}

struct Device {
    pub name: String,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Device {
    fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let title = cx.string(&self.name);
        obj.set(cx, "title", title)?;
        let major = cx.number(self.major);
        obj.set(cx, "major", major)?;
        let minor = cx.number(self.minor);
        obj.set(cx, "minor", minor)?;
        let patch = cx.number(self.patch);
        obj.set(cx, "patch", patch)?;

        Ok(obj)
    }
}

fn get_physical_versions(mut cx: FunctionContext) -> JsResult<JsArray> {
    let entry = unsafe { Entry::load() }.expect("Failed to load vulkan");

    let app_info = vk::ApplicationInfo {
        p_application_name: APP_NAME.as_ptr() as *const i8,
        application_version: vk::make_api_version(0, 1, 0, 0),
        api_version: vk::make_api_version(0, 1, 3, 0),
        ..Default::default()
    };

    let instance_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
    };

    let instance = unsafe { entry.create_instance(&instance_info, None) }
        .expect("Failed to create Vulkan instance");

    let devices =
        unsafe { instance.enumerate_physical_devices() }.expect("Failed to enumerate devices");

    let array = cx.empty_array();
    let mut index = 0;
    for device in devices {
        let properties = unsafe { instance.get_physical_device_properties(device) };

        if properties.device_type == vk::PhysicalDeviceType::CPU {
            continue;
        }

        let slice: &[u8; 256] = unsafe { std::mem::transmute(&properties.device_name) };
        let name = String::from(std::str::from_utf8(slice).unwrap().trim_end_matches('\0'));

        let major = vk::api_version_major(properties.api_version);
        let minor = vk::api_version_minor(properties.api_version);
        let patch = vk::api_version_patch(properties.api_version);
        let device_struct = Device {
            name,
            major,
            minor,
            patch,
        };
        let js_object = device_struct.to_object(&mut cx)?;
        array.set(&mut cx, index, js_object)?;
        index += 1;
    }

    unsafe { instance.destroy_instance(None) };
    Ok(array)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get_instance_version", get_instance_version)?;
    cx.export_function("get_physical_versions", get_physical_versions)?;
    Ok(())
}
