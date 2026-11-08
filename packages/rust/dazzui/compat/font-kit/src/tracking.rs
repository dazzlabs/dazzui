//! The normal track of an AAT `trak` table, expressed in design units.
//!
//! Core Text truncates an interpolated tracking value to whole design units.
//! Browser shaping keeps its fraction, then resolves it in 16.16 pixel units.
//! The difference belongs to each shaped grapheme cluster, including the last.
//!
//! Format: https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6trak.html
//! Browser interpolation: HarfBuzz's `hb-aat-layout-trak-table.hh`.

#[derive(Clone, Debug)]
pub struct Tracking {
    samples: Vec<(f64, f64)>,
}

fn u16_at(data: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        data.get(at..at.checked_add(2)?)?.try_into().ok()?,
    ))
}

fn u32_at(data: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(
        data.get(at..at.checked_add(4)?)?.try_into().ok()?,
    ))
}

impl Tracking {
    /// Read horizontal tracking at the normal (zero) track. Malformed and
    /// unsupported tables leave the platform's existing layout untouched.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if u32_at(data, 0)? != 0x0001_0000 || u16_at(data, 4)? != 0 {
            return None;
        }
        let horizontal = usize::from(u16_at(data, 6)?);
        if horizontal == 0 {
            return None;
        }
        let tracks = usize::from(u16_at(data, horizontal)?);
        let sizes = usize::from(u16_at(data, horizontal + 2)?);
        let size_offset = usize::try_from(u32_at(data, horizontal + 4)?).ok()?;
        if tracks == 0 || sizes == 0 {
            return None;
        }
        data.get(horizontal.checked_add(8)?..horizontal.checked_add(8 + tracks * 8)?)?;
        data.get(size_offset..size_offset.checked_add(sizes * 4)?)?;
        let mut records = Vec::with_capacity(tracks);
        for index in 0..tracks {
            let record = horizontal + 8 + index * 8;
            let track = (u32_at(data, record)? as i32) as f64 / 65536.;
            let values = usize::from(u16_at(data, record + 6)?);
            data.get(values..values.checked_add(sizes * 2)?)?;
            if records
                .last()
                .is_some_and(|&(previous, _)| previous >= track)
            {
                return None;
            }
            records.push((track, values));
        }
        let right = records
            .partition_point(|&(track, _)| track < 0.)
            .min(tracks - 1);
        let left = if records[right].0 <= 0. {
            right
        } else {
            right.saturating_sub(1)
        };
        let (a, values_a) = records[left];
        let (b, values_b) = records[right];
        let fraction = if left == right { 0. } else { -a / (b - a) };
        let mut samples = Vec::with_capacity(sizes);
        for index in 0..sizes {
            let size = (u32_at(data, size_offset + index * 4)? as i32) as f64 / 65536.;
            if size <= 0.
                || samples
                    .last()
                    .is_some_and(|&(previous, _)| previous >= size)
            {
                return None;
            }
            let a = u16_at(data, values_a + index * 2)? as i16 as f64;
            let b = u16_at(data, values_b + index * 2)? as i16 as f64;
            samples.push((size, a + fraction * (b - a)));
        }
        Some(Self { samples })
    }

    fn units(&self, point_size: f64) -> f64 {
        let right = self.samples.partition_point(|&(size, _)| size < point_size);
        if right == 0 {
            return self.samples[0].1;
        }
        if right == self.samples.len() {
            return self.samples[right - 1].1;
        }
        let (a_size, a) = self.samples[right - 1];
        let (b_size, b) = self.samples[right];
        a + (point_size - a_size) / (b_size - a_size) * (b - a)
    }

    /// The pixel correction to Core Text's advance for one grapheme cluster.
    pub fn adjustment(&self, point_size: f32, units_per_em: u32) -> f32 {
        if !point_size.is_finite() || point_size <= 0. || units_per_em == 0 {
            return 0.;
        }
        let point_size = f64::from(point_size);
        let units = self.units(point_size);
        let scale = f64::from(units_per_em);
        let native = units.trunc() * point_size / scale;
        let browser_scale = (point_size * 65536.).round();
        let browser = (units * browser_scale / scale).round() / 65536.;
        (browser - native) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Vec<u8> {
        // A single normal track: 0 at 12pt, -12 at 13pt and -22 at 14pt.
        let mut data = vec![0; 46];
        data[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
        data[6..8].copy_from_slice(&12u16.to_be_bytes());
        data[12..14].copy_from_slice(&1u16.to_be_bytes());
        data[14..16].copy_from_slice(&3u16.to_be_bytes());
        data[16..20].copy_from_slice(&28u32.to_be_bytes());
        data[26..28].copy_from_slice(&40u16.to_be_bytes());
        for (index, size) in [12u32, 13, 14].into_iter().enumerate() {
            data[28 + index * 4..32 + index * 4].copy_from_slice(&(size * 65536).to_be_bytes());
        }
        data[42..44].copy_from_slice(&(-12i16).to_be_bytes());
        data[44..46].copy_from_slice(&(-22i16).to_be_bytes());
        data
    }

    #[test]
    fn fractional_tracking_preserves_the_browser_advance() {
        let track = Tracking::from_bytes(&table()).unwrap();
        assert_eq!(track.adjustment(12., 2048), 0.);
        assert_eq!(track.adjustment(13., 2048), 0.);
        assert_eq!(track.adjustment(14., 2048), 0.);
        let corrected = 291.047010 + f64::from(track.adjustment(13.33, 2048)) * 48.;
        assert!((corrected - 290.952942).abs() < 0.001, "{corrected}");
        assert_eq!(track.adjustment(f32::NAN, 2048), 0.);
        assert_eq!(track.adjustment(13.33, 0), 0.);
    }

    #[test]
    fn incomplete_or_unsorted_tables_are_ignored() {
        let data = table();
        for end in 0..data.len() {
            assert!(Tracking::from_bytes(&data[..end]).is_none());
        }
        let mut data = data;
        data[32..36].copy_from_slice(&(11u32 * 65536).to_be_bytes());
        assert!(Tracking::from_bytes(&data).is_none());
    }

    #[test]
    fn normal_tracking_interpolates_between_named_tracks_when_zero_is_absent() {
        let mut data = vec![0; 60];
        data[..20].copy_from_slice(&table()[..20]);
        data[12..14].copy_from_slice(&2u16.to_be_bytes());
        data[16..20].copy_from_slice(&36u32.to_be_bytes());
        data[20..24].copy_from_slice(&(-65536i32).to_be_bytes());
        data[26..28].copy_from_slice(&48u16.to_be_bytes());
        data[28..32].copy_from_slice(&65536i32.to_be_bytes());
        data[34..36].copy_from_slice(&54u16.to_be_bytes());
        for (index, size) in [12u32, 13, 14].into_iter().enumerate() {
            data[36 + index * 4..40 + index * 4].copy_from_slice(&(size * 65536).to_be_bytes());
            let value = [-10i16, -20, -30][index];
            data[48 + index * 2..50 + index * 2].copy_from_slice(&value.to_be_bytes());
            data[54 + index * 2..56 + index * 2].copy_from_slice(&(value + 20).to_be_bytes());
        }
        let tracking = Tracking::from_bytes(&data).unwrap();
        assert_eq!(tracking.units(12.), 0.);
        assert_eq!(tracking.units(13.), -10.);
        assert_eq!(tracking.units(13.5), -15.);
        assert_eq!(tracking.units(5.), 0.);
        assert_eq!(tracking.units(40.), -20.);
    }

    #[test]
    fn whole_point_sizes_can_still_interpolate_fractional_design_units() {
        let mut data = table();
        for (index, (size, value)) in [(24u32, 6i16), (26, 17), (28, 28)].into_iter().enumerate() {
            data[28 + index * 4..32 + index * 4].copy_from_slice(&(size * 65536).to_be_bytes());
            data[40 + index * 2..42 + index * 2].copy_from_slice(&value.to_be_bytes());
        }
        let tracking = Tracking::from_bytes(&data).unwrap();
        assert_eq!(tracking.units(25.), 11.5);
        assert_eq!(tracking.adjustment(25., 2048), 0.0061035156);
    }
}
