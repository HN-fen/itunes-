use std::intrinsics::transmute;
use std::mem;
use std::ptr::null_mut;
use std::time::Duration;

use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::System::Com::*;
use bindings::Windows::Win32::System::OleAutomation::*;
use windows::*;

const ITUNES_CLSID: Guid = Guid::from_values(
    0xDC0C2640,
    0x1415,
    0x4644,
    [0x87, 0x5C, 0x6F, 0x4D, 0x76, 0x98, 0x39, 0xBA],
);

pub struct ITunes(IiTunes);

impl ITunes {
    pub fn new() -> windows::Result<Self> {
        unsafe {
            CoInitialize(null_mut())?;
            Ok(Self(CoCreateInstance(
                &ITUNES_CLSID,
                None,
                CLSCTX_LOCAL_SERVER,
            )?))
        }
    }

    fn get_instance(&self) -> &IiTunes {
        &self.0
    }

    pub fn check_if_alive(&self) -> bool {
        unsafe { self.get_instance().GetPlayerState() }.is_ok()
    }

    pub fn is_playing(&self) -> bool {
        unsafe { self.get_instance().GetPlayerState() }
            .map(|state| state == 1)
            .unwrap_or(false)
    }

    pub fn get_current_track_info(&self) -> Option<TrackInfo> {
        unsafe {
            self.get_instance()
                .GetCurrentTrack()
                .unwrap_or(None)
                .and_then(|track| {
                    (|| -> windows::Result<TrackInfo> {
                        let name = track.GetName()?.to_string();
                        let artist = track.GetArtist()?.to_string();
                        Ok(TrackInfo { name, artist })
                    })()
                    .map(Some)
                    .unwrap_or(None)
                })
        }
    }

    pub fn get_player_position(&self) -> Option<Duration> {
        unsafe {
            self.get_instance()
                .GetPlayerPosition()
                .map(|secs| Some(Duration::from_secs(secs as u64)))
                .unwrap_or(None)
        }
    }
}

impl Drop for ITunes {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

#[derive(Debug)]
pub struct TrackInfo {
    pub name: String,
    pub artist: String,
}

struct IiTunes(IDispatch);

#[repr(C)]
#[allow(non_camel_case_types)]
struct IiTunes_abi(
    // IUnknown
    pub unsafe extern "system" fn(this: RawPtr, iid: *const Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    // IDispatch (TODO)
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    // IiTunes
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr, value: *mut i32) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr, value: *mut i64) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr, track: *mut RawPtr) -> HRESULT,
);

unsafe impl Interface for IiTunes {
    type Vtable = IiTunes_abi;

    const IID: Guid = Guid::from_values(
        0x9DD6_680B,
        0x3EDC,
        0x40db,
        [0xA7, 0x71, 0xE6, 0xFE, 0x48, 0x32, 0xE3, 0x4A],
    );
}

#[allow(non_snake_case)]
impl IiTunes {
    pub unsafe fn GetPlayerState(&self) -> Result<i32> {
        let mut value: i32 = 0;
        (Interface::vtable(self).39)(Abi::abi(self), &mut value).ok()?;
        Ok(value)
    }

    pub unsafe fn GetPlayerPosition(&self) -> Result<i64> {
        let mut value: i64 = 0;
        (Interface::vtable(self).40)(Abi::abi(self), &mut value).ok()?;
        Ok(value)
    }

    pub unsafe fn GetCurrentTrack(&self) -> Result<Option<IITTrack>> {
        let mut abi = null_mut();
        (Interface::vtable(self).62)(Abi::abi(self), &mut abi).ok()?;
        if abi.is_null() {
            Ok(None)
        } else {
            Ok(transmute(abi))
        }
    }
}

struct IITTrack(IDispatch);

#[repr(C)]
#[allow(non_camel_case_types)]
struct IITTrack_abi(
    // IUnknown
    pub unsafe extern "system" fn(this: RawPtr, iid: *const Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    // IDispatch (TODO)
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    // IITObject
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr, value: *mut *mut u16) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    // IITTrack
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr, value: *mut *mut u16) -> HRESULT,
);

unsafe impl Interface for IITTrack {
    type Vtable = IITTrack_abi;

    const IID: Guid = Guid::from_values(
        0x4cb0_915d,
        0x1e54,
        0x4727,
        [0xba, 0xf3, 0xce, 0x6c, 0xc9, 0xa2, 0x25, 0xa1],
    );
}

#[allow(non_snake_case)]
impl IITTrack {
    pub unsafe fn GetName(&self) -> Result<BSTR> {
        let mut abi: <BSTR as Abi>::Abi = mem::zeroed();
        (Interface::vtable(self).8)(Abi::abi(self), &mut abi).from_abi(abi)
    }

    pub unsafe fn GetArtist(&self) -> Result<BSTR> {
        let mut abi: <BSTR as Abi>::Abi = mem::zeroed();
        (Interface::vtable(self).22)(Abi::abi(self), &mut abi).from_abi(abi)
    }
}
