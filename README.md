# Radiation

Radio receiver absorbs electromagnetic radiation and converts it in some signal.
Radio transmitter converts a signal into electromagnetic radiation and emit it.

This crate calls raw bytes as a radiation. It allows 'absorb' the radiation
and convert it into Rust type, and vice versa, 'emit' Rust type as a radiation.

Trait `Absorb` parse bytes and return the typed value. It may fail and return
an error.

Trait `Emit` convert the typed value into raw bytes.

## Derive

### Attribute `tag`

At enum. Specifies the type of tag, it may be any type implementing
`Absorb` and `Display` (for error handling). Default is `u16`.

At variant. Specifies the value of tag for the variant. If the type of the tag
implements `Default` and `Add` numeric (e.g. it is meaningful to do `tag + 1`).
Then the attribute can be omitted. Default is 0, 1, 2,...

### Attributes `custom_absorb` and `custom_emit`

At field. Allows to specify custom function for absorb and emit the field.
Useful if the field has foreign type, for example `SocketAddr`.

### Attribute `as_str`

At field. The implementation will absorb and emit the string and use
`FromStr` and `Display` on the field to convert.

### Example

```
#[derive(Debug, PartialEq, Eq, Absorb, Emit)]
#[tag(u8)]
enum SomeEnum {
    #[tag(1)]
    A {
        one: u8,
        two: u8,
        // specify a function to parse the field with
        #[custom_absorb(absorb)]
        #[custom_emit(emit)]
        three: u16,
    },
    // use `FromStr` implementation to parse the value from `str`
    B(#[as_str] u16),
    C(u32),
}
```

See `tests.rs` for more information and examples.

### Limit

Attribute `limit` overwrite the `L` argument when parse the field.
This attribute provides a type which implements `Limit` trait.

The trait provides lower and upper limit in bytes. Also, it carry information
which limit will be on next field, or in inner structure.

See `tests.rs` for more information and examples.
