## Example
```
#[pack_struct]
struct MyStruct {
  a: u16, // Most significant
  b: u8,
  c: u8,
  d: u8 // Least significant
}
```
Using the macro, the fields in this struct can be packed into a single unsigned integer. In this specific case the layout would be as follows:

<table style="border-collapse: collapse; width: 100%; border: 1px solid;">
  <tr>
    <td>Least significant</td>
    <td></td>
    <td></td>
    <td></td>
    <td>Most significant</td>
  <tr>
    <td style="width: 12.5%; border: 1px solid;">Empty (8 bit)</td>
    <td style="width: 12.5%; border: 1px solid;">D (8 bit)</td>
    <td style="width: 12.5%; border: 1px solid;">C (8 bit)</td>
    <td style="width: 12.5%; border: 1px solid;">B (8 bit)</td>
    <td style="width: 25%; border: 1px solid;">A (16 bit)</td>
  </tr>
</table>

The total bitsize of this struct is 40 bits, which can be packed into a 64 bit unsigned integer. (The next power of 2).
If the total bitsize is smaller or equal to 32, it would be packed into a 32 bit unsigned integer, or if <= 16; 16 etc..
Each field is packed into the unsigned integer in order.

The macro defines this new struct and functions:

```
struct MyStructPacked {
 data: u64
}

impl MyStruct {
 pub fn pack() -> DrawKeyPacked {
  -- Snip --
 }
}

impl MyStructPacked {
 pub fn unpack() -> DrawKey {
  -- Snip --
 }
}
```

The [pack_struct] procedural macro, automatically provides a pack() method which returns a struct of the same name appended by "Packed", so "DrawKeyPacked". DrawKeyPacked has a unpack() method which reverses this process, providing the original struct with the same values from before packing.

## Known limitations
* Does not support fields having types other than the Rust primitive types:
  * Bool, i8, u8, i16, u16, i32, u32, f32, char, i64, u64, f64, i128, u128, usize, isize.
  * This also means a field cannot be a struct even though that struct only contains these primitive types.
* Does not support fields with lifetimes, types with typeparameters (generics), or other complex types.


## Explanation/Motivation
In OpenGL theres a concept of "drawkeys" which encapsulates the information needed to draw an object to screen.

To draw an object to screen, multiple variables are needed, such as which shader to use, which mesh to use etc. These are called "states", and objects almost always share some state between them. Objects should be ordered before drawing, so that adjacent objects share as much state as possible.

Theres also the concept of state change cost, where for example changing the shader is one of the most expensive state changes, which material to use is the second most costly state change and so on...

When some state is not common between all objects, we need to prioritize which order to draw the objects, where expensive states should be changed least. To do this, the macro packs each field into the key in order of first -> last field to most_significant_bits -> least_significant_bits in the packed key.

The cost associated with each state change in OpenGL, in decreasing order is as follows:
  * Render target
  * Program (shader)
  * ROP (Raster operations)
  * Texture bindings
  * Vertex format
  * UBO bindings
  * Vertex format
  * UBO bindings
  * Vertex bindings
  * Uniform updates

[Source](https://www.youtube.com/watch?v=-bCeNzgiJ8I)

If our software provides 2 render targets, we can describe this as a bool (2 possible values).
In the same manner, depending on how our software is implemented, we set a max possible value for each of these state changes, and pack it into a unsigned integer. 

In a drawkey, the most expensive state changes are placed in the most significant bits of the drawkey. We only change the OpenGL state whenever we are about to draw an object with a different state than the one we last drew, as OpenGL remembers which states we have set already.

When sorted, the array of drawkeys will be ordered in a way where expensive state changes are minimized.

## Example usage in graphical applications
Here's an example of how this can be used
```
pub fn set_changed_state(current_drawkey: &DrawKey, last_drawkey: Option<&DrawKey>) {
  // Implementation not included.
  // This function should compare each field of the two drawkeys, if the current_drawkey has some state which is different than last_drawkey, change the appropriate state in OpenGL.
  // If last_drawkey is Option(None), just set the state.
}

pub fn draw_to_screen(objects: &Vec<RenderableObjects>) {
  // Create and sort drawkeys
  let mut draw_keys: Vec<DrawKeyPacked> = objects.iter().map(|object| object.drawkey.pack()).collect();
  draw_keys.sort();
  
  // Set the first drawkey and its state
  let mut last_drawkey = draw_keys.first().unpack();
  set_changed_state(last_drawkey, Option(None));
  
  // Compare drawkey to last_drawkey and change state if the state is different to the last_drawkey
  for drawkey_packed in draw_keys.iter().skip(1) {
    let drawkey = drawkey_packed.unpack();
    set_changed_state(drawkey, Some(last_drawkey));
    last_drawkey = drawkey;
  }
  
  -- Snip --
}
```

The reason we dont just implement sorting for DrawKey, is because its way faster to sort unsigned integers, as it requires only a single comparison. This speed increase is important as sorting the drawkeys and drawing the renderable objects to screen should happen 60+ times each second. 

The definition of the drawkey will potentially change often during development as new features are added, which can introduce errors if not careful implementing the packing functionality. The macro automatically defines the packed drawkey only from which field the original drawkey has, and so reduces development time.

For another explanation of drawkeys, check out https://realtimecollisiondetection.net/blog/?p=86 