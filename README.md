Work in progress

## Explanation/Motivation
In OpenGL theres a concept of "drawkeys" which encapsulates the information needed to draw an object to screen.

To draw an object to screen, multiple variables are needed, such as which shader to use, which mesh to use etc. These are called "states", and objects almost always share some state between them. Objects should be ordered before drawing, so that adjacent objects share as much state as possible.

Theres also the concept of state change cost, where for example changing the shader is one of the most expensive state changes, which material to use is the second most costly state change and so on...

When some state is not common between all objects, we need to prioritize which order to draw the objects, where expensive states should be changed least. To do this, the macro packs each field into the key in order of first-last field to most_significant_bits-least_siginificant_bits in the packed key.

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

## Example usage
```
#[pack_struct]
struct DrawKey {
  render_target: bool, // Most expensive
  program: u8,
  material: u8,
  mesh: u8,
  uniform_update_buffer: u8 // Least expensive
}
```
The macro defines this new struct and functions:

```
struct DrawKeyPacked {
 data: u64
}

impl DrawKey {
 pub fn pack() -> DrawKeyPacked {
  -- Snip --
 }
}

impl DrawKeyPacked {
 pub fn unpack() -> DrawKey {
  -- Snip --
 }
}
```


The total bitsize of this struct is 38, which can be packed into a 64 bit unsigned integer. (The next power of 2).
If the total bitsize is smaller or equal to 32, it would be packed into a 32 bit unsigned integer, or if <= 16; 16 etc..
Each field is packed into the unsigned integer in order, so render_target will be packed first into the most significant bits, program will be packed into the second most significant bits etc.

The [pack_struct] procedural macro, automatically provides a pack() method which returns a struct of the same name appended by "Packed", so "DrawKeyPacked". DrawKeyPacked has a unpack() method which reverses this process, providing the original struct with the same values from before packing.

Here's an example of how this can be used
```
pub fn set_changed_state(current_drawkey: &DrawKey, last_drawkey: Option<&DrawKey>) {
  // Implementation not included.
  // This function should compare each field of the two drawkeys, if the current_drawkey has some state which is different than last_drawkey, change the appropriate state in OpenGL.
  // If last_drawkey is Option(None), just set the state.
}

pub fn draw_to_screen(objects: &Vec<RenderableObjects>) {
  // Create and sort drawkeys
  let mut draw_keys: Vec<DrawKeyPacked> = objects.iter().map(|object| object.pack()).collect();
  draw_keys.sort();
  
  // Set the first drawkey and its state
  let mut last_drawkey = draw_keys.first().unpack();
  set_changed_state(last_drawkey, Option(None));
  
  // Compare drawkey to last_drawkey and change state if the state is different to the last_drawkey
  for drawkey_packed in draw_keys.iter() {
    let drawkey = drawkey_packed.unpack();
    set_changed_state(drawkey, last_drawkey);
    last_drawkey = drawkey;
  }
  
  -- Snip --
}
```

The reason we dont just implement sorting for DrawKey, is because its WAAAAAY faster to sort unsigned integers. This speed increase is important as sorting the drawkeys and drawing the renderable objects to screen should happen 60+ times each second. 

The definition of the drawkey will also change very often during development as new features are added, which can introduce errors if not careful implementing the packing functionality. The macro automatically defines the packed drawkey only from which field the original drawkey has, and so reduces development time.
