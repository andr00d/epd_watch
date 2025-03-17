// #[cfg(target_os = "none")]
// use rtt_target::rprintln;

use crate::pages::Pages;
use crate::shared_data::SharedData;
use crate::pages::vid_frames::{VID_TRIS, VID_VERTS};
use crate::io::Event;

impl Pages
{
    pub(super) fn pg_video(data: &mut SharedData) 
    {
        let mut i_tris:usize = 0;
        let mut i_verts:usize = 0;

        data.io.start_song();

        loop
        {
            data.io.rtc2_set_ms(333);

            let tricnt = VID_TRIS[i_tris].0 as usize;
            let vrtcnt = VID_VERTS[i_verts].0 as usize;
            i_tris += 1;
            i_verts += 1;

            for tris in i_tris..i_tris + tricnt
            {
                let tri = VID_TRIS[tris];
                let v1 = VID_VERTS[i_verts + tri.0 as usize];
                let v2 = VID_VERTS[i_verts + tri.1 as usize];
                let v3 = VID_VERTS[i_verts + tri.2 as usize];
                data.display.triangle(v1, v2, v3);
            }

            i_tris += tricnt;
            i_verts += vrtcnt;
            data.display.update(&mut data.io);
            let ev = data.io.rtc2_wait_finish();
    
            if ev == Event::BtnMid {break;}
            if i_tris >= VID_TRIS.len() || i_verts >= VID_VERTS.len() {break;}
        }

        data.io.stop_song();
    }

    ///////////////////////////////////////////
}