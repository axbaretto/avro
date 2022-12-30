// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
use apache_avro::{
=======
use avro_rs::{
>>>>>>> 49c6f59 (AVRO-3317: JavaScript: Update dependencies)
=======
use avro_rs::{
=======
use apache_avro::{
>>>>>>> develop
>>>>>>> axbaretto
=======
use avro_rs::{
>>>>>>> f9bcab5 (AVRO-3317: JavaScript: Update dependencies)
    schema::Schema,
    types::{Record, Value},
    Codec, Writer,
};
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
use std::collections::HashMap;
=======
>>>>>>> axbaretto
use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};
<<<<<<< HEAD
=======
use std::collections::HashMap;
>>>>>>> 49c6f59 (AVRO-3317: JavaScript: Update dependencies)
=======
>>>>>>> develop
>>>>>>> axbaretto
=======
use std::collections::HashMap;
>>>>>>> f9bcab5 (AVRO-3317: JavaScript: Update dependencies)
use strum::IntoEnumIterator;

fn create_datum(schema: &Schema) -> Record {
    let mut datum = Record::new(schema).unwrap();
    datum.put("intField", 12_i32);
    datum.put("longField", 15234324_i64);
    datum.put("stringField", "hey");
    datum.put("boolField", true);
    datum.put("floatField", 1234.0_f32);
    datum.put("doubleField", -1234.0_f64);
    datum.put("bytesField", b"12312adf".to_vec());
    datum.put("nullField", Value::Null);
    datum.put(
        "arrayField",
        Value::Array(vec![
            Value::Double(5.0),
            Value::Double(0.0),
            Value::Double(12.0),
        ]),
    );
    let mut map = HashMap::new();
    map.insert(
        "a".into(),
        Value::Record(vec![("label".into(), Value::String("a".into()))]),
    );
    map.insert(
        "bee".into(),
        Value::Record(vec![("label".into(), Value::String("cee".into()))]),
    );
    datum.put("mapField", Value::Map(map));
    datum.put("unionField", Value::Union(1, Box::new(Value::Double(12.0))));
    datum.put("enumField", Value::Enum(2, "C".to_owned()));
    datum.put("fixedField", Value::Fixed(16, b"1019181716151413".to_vec()));
    datum.put(
        "recordField",
        Value::Record(vec![
            ("label".into(), Value::String("outer".into())),
            (
                "children".into(),
                Value::Array(vec![Value::Record(vec![
                    ("label".into(), Value::String("inner".into())),
                    ("children".into(), Value::Array(vec![])),
                ])]),
            ),
        ]),
    );

    datum
}

fn main() -> anyhow::Result<()> {
    let schema_str = std::fs::read_to_string("../../share/test/schemas/interop.avsc")
        .expect("Unable to read the interop Avro schema");
    let schema = Schema::parse_str(schema_str.as_str())?;

    for codec in Codec::iter() {
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
=======
>>>>>>> axbaretto
=======
>>>>>>> f9bcab5 (AVRO-3317: JavaScript: Update dependencies)
        let mut writer = Writer::with_codec(&schema, Vec::new(), codec);
        let datum = create_datum(&schema);
        writer.append(datum)?;
        let bytes = writer.into_inner()?;

<<<<<<< HEAD
<<<<<<< HEAD
>>>>>>> 49c6f59 (AVRO-3317: JavaScript: Update dependencies)
=======
=======
>>>>>>> develop
>>>>>>> axbaretto
=======
>>>>>>> f9bcab5 (AVRO-3317: JavaScript: Update dependencies)
        let codec_name = <&str>::from(codec);
        let suffix = if codec_name == "null" {
            "".to_owned()
        } else {
            format!("_{}", codec_name)
        };

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
=======
>>>>>>> f9bcab5 (AVRO-3317: JavaScript: Update dependencies)
        std::fs::write(
            format!("../../build/interop/data/rust{}.avro", suffix),
            bytes,
        )?;
<<<<<<< HEAD
=======
>>>>>>> axbaretto
        let file_name = format!("../../build/interop/data/rust{}.avro", suffix);
        let output_file = std::fs::File::create(&file_name)?;

        let mut writer = Writer::with_codec(&schema, BufWriter::new(output_file), codec);
        write_user_metadata(&mut writer)?;

        let datum = create_datum(&schema);
        writer.append(datum)?;
        writer.flush()?;
        println!("Wrote {}", file_name);
<<<<<<< HEAD
=======
        std::fs::write(
            format!("../../build/interop/data/rust{}.avro", suffix),
            bytes,
        )?;
>>>>>>> 49c6f59 (AVRO-3317: JavaScript: Update dependencies)
=======
>>>>>>> develop
>>>>>>> axbaretto
=======
>>>>>>> f9bcab5 (AVRO-3317: JavaScript: Update dependencies)
    }

    Ok(())
}
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
=======
>>>>>>> axbaretto

fn write_user_metadata<W: Write>(writer: &mut Writer<BufWriter<W>>) -> anyhow::Result<()> {
    writer.add_user_metadata("user_metadata".to_string(), b"someByteArray")?;

    Ok(())
}
<<<<<<< HEAD
=======
>>>>>>> 49c6f59 (AVRO-3317: JavaScript: Update dependencies)
=======
>>>>>>> develop
>>>>>>> axbaretto
=======
>>>>>>> f9bcab5 (AVRO-3317: JavaScript: Update dependencies)
