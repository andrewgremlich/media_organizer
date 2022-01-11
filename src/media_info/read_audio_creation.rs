pub fn read_audio_creation_date(path_str: &str) {
  println!("{}: audio file", path_str);

  // let data = metadata(path_str).unwrap();

  //convert data created to readable date time
  // let date_time = match data.created() {
  //   Ok(date) => {
  //     let unix_timestamp = date
  //       .duration_since(SystemTime::UNIX_EPOCH)
  //       .unwrap()
  //       .as_secs()
  //       .to_string();

  //     println!("{}: {}", path_str, unix_timestamp);

  // match unix_timestamp {
  //   Ok(n) => println!("{}", n),
  //   _ => ()
  // }
  //   }
  //   _ => (),
  // };

  // let src = std::fs::File::open(&path_str).expect("failed to open media");
  // let mss = MediaSourceStream::new(Box::new(src), Default::default());

  // let hint = Hint::new();
  // let meta_opts: MetadataOptions = Default::default();
  // let fmt_opts: FormatOptions = Default::default();

  // let probed = symphonia::default::get_probe()
  //   .format(&hint, mss, &fmt_opts, &meta_opts)
  //   .expect("unsupported format");

  // match probed {
  //   ProbeResult::Format(format) => {
  //     let creation_date = format.metadata.get("creation_time").unwrap();
  //     println!("{}: {}", path_str, creation_date);
  //   }
  //   _ => AudioReaderHandle::Err(String::from("could-not-read-audio-date")),
  // }

  // AudioReaderHandle::Err(String::from("audio-creation-date-not-implemented"))
}
