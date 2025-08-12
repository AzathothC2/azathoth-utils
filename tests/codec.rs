#[cfg(all(feature="codec", test))]

mod codec_tests {
    use azathoth_utils::codec::{Codec, Decoder, Encoder};
    use azathoth_utils::errors::AzUtilResult;

    #[derive(Debug, PartialEq)]
    struct Payload {
        id: u32,
        name: String,
        bytes: Option<Vec<u8>>,
        negative: i64,
        flags: bool,
    }
    impl Codec for Payload {
        fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
            enc.push_u32(self.id)?;
            enc.push_string(&self.name)?;
            enc.push_opt(&self.bytes)?;
            enc.push_i64(self.negative)?;
            enc.push_bool(self.flags)?;
            Ok(())
        }
        fn decode(dec: &mut Decoder) -> AzUtilResult<Self> where Self: Sized {
            let id    = dec.read_u32()?;
            let name  = dec.read_string()?;
            let bytes = dec.read_opt::<Vec<u8>>()?;
            let negative   = dec.read_i64()?;
            let flags = dec.read_bool()?;
            Ok(Self { id, name, bytes, negative, flags })
        }
    }

    #[test]
    fn codec_error_paths() {
        let mut dec = Decoder::new(&[]);
        let eof = dec.read_u8().err().expect("expected error");
        let _ = eof;
    }

    #[test]
    fn roundtrip_payload() {
        let msg = Payload {
            id: 0xDEAD_BEEF,
            name: "hello".into(),
            bytes: Some(vec![1, 2, 3, 4]),
            negative: -42,
            flags: true,
        };

        let mut enc = Encoder::new();
        msg.encode(&mut enc).expect("encode ok");
        let bytes = enc.into_inner();

        let mut dec = Decoder::new(&bytes);
        let got = Payload::decode(&mut dec).expect("decode ok");
        assert_eq!(got, msg);
    }
}