// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod test {
    use common::{__MessageWithBytes, MessageWithBytes};
    use serde_json::{Value, json};
    use test_case::test_case;
    type Result = anyhow::Result<()>;

    #[test_case(MessageWithBytes::new(), json!({}))]
    fn test_ser(input: MessageWithBytes, want: Value) -> Result {
        let got = serde_json::to_value(__MessageWithBytes(input))?;
        assert_eq!(got, want);
        Ok(())
    }

    #[test_case(MessageWithBytes::new(), json!({}))]
    fn test_de(want: MessageWithBytes, input: Value) -> Result {
        let got = serde_json::from_value::<__MessageWithBytes>(input)?;
        assert_eq!(got.0, want);
        Ok(())
    }

    #[test_case(json!({"unknown": "test-value"}))]
    #[test_case(json!({"unknown": "test-value", "moreUnknown": {"a": 1, "b": 2}}))]
    fn test_unknown(input: Value) -> Result {
        let deser = serde_json::from_value::<__MessageWithBytes>(input.clone())?;
        let got = serde_json::to_value(deser)?;
        assert_eq!(got, input);
        Ok(())
    }

    #[test]
    fn test_serialize_singular() -> Result {
        let b = bytes::Bytes::from("the quick brown fox jumps over the laze dog");
        let msg = MessageWithBytes::new().set_singular(b);
        let got = serde_json::to_value(&msg)?;
        let want =
            json!({"singular": "dGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXplIGRvZw=="});
        assert_eq!(want, got);

        let roundtrip = serde_json::from_value::<MessageWithBytes>(got)?;
        assert_eq!(msg, roundtrip);
        Ok(())
    }

    #[test]
    fn test_serialize_optional() -> Result {
        let b = bytes::Bytes::from("the quick brown fox jumps over the laze dog");
        let msg = MessageWithBytes::new().set_optional(b);
        let got = serde_json::to_value(&msg)?;
        let want =
            json!({"optional": "dGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXplIGRvZw=="});
        assert_eq!(want, got);

        let roundtrip = serde_json::from_value::<MessageWithBytes>(got)?;
        assert_eq!(msg, roundtrip);
        Ok(())
    }

    #[test]
    fn test_serialize_repeated() -> Result {
        let b = bytes::Bytes::from("the quick brown fox jumps over the laze dog");
        let msg = MessageWithBytes::new().set_repeated([b]);
        let got = serde_json::to_value(&msg)?;
        let want =
            json!({"repeated": ["dGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXplIGRvZw=="]});
        assert_eq!(want, got);

        let roundtrip = serde_json::from_value::<MessageWithBytes>(got)?;
        assert_eq!(msg, roundtrip);
        Ok(())
    }

    #[test]
    fn test_serialize_map() -> Result {
        let b = bytes::Bytes::from("the quick brown fox jumps over the laze dog");
        let msg = MessageWithBytes::new().set_map([("quick", b)]);
        let got = serde_json::to_value(&msg)?;
        let want = json!({"map": {"quick": "dGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXplIGRvZw=="}});
        assert_eq!(want, got);

        let roundtrip = serde_json::from_value::<MessageWithBytes>(got)?;
        assert_eq!(msg, roundtrip);
        Ok(())
    }
}
