# PPCA Raytracer 2022

Raytracing simulation!

Final work:  
**"The Whale"**

![high5_1.jpg](https://s2.loli.net/2022/07/27/aKfk9OorRm6NWvS.jpg)

---

## Book 1

### Final Scene Render Result:

![image21.jpg](https://s2.loli.net/2022/07/20/mBzZNlSYThy2Cox.jpg)

### Completed Basic Functions (`basic_tools` series):

- **`vec3.rs` Vector:**
  - `Vec3`, `Color`, `Point` vector class declarations.
  - Simulates reflection and refraction of light.
  - Overloaded various operators.
  - Dot product and cross product of vectors.
  - Generate random unit vectors and random vectors on a disk.
  - Calculate vector length and normalize vectors.

- **`ray.rs` Ray:**
  - `Ray` class declaration, including origin and direction.

- **`camera.rs` Camera:**
  - `Camera` class declaration.
  - Canvas positioning, including width, height, and bottom-left corner.
  - Camera positioning, including location, orientation, and field of view.
  - Aperture size for depth-of-field effects.

### Completed Materials (`material` series):

- **`lambertian.rs` Lambertian Material:**
  - Simulates diffuse reflection by generating random light directions.

- **`metal.rs` Metal Material:**
  - Calculates perfectly symmetric light reflection paths with a `fuzz` attribute to represent surface glossiness and reflection blur.

- **`dielectric.rs` Glass Material:**
  - Uses refraction formulas; reflects light if the refraction ratio exceeds 1.

### Completed Hittable Objects:

- **Abstract Trait `Hittable`:**
  - Currently contains only `fn hit` to determine if an object is hit and calculate the `HitRecord`.

- **`HitRecord`:**
  - Stores the state after the ray hits a point.

- **`sphere.rs`:**
  - Sphere object abstraction, defined by the sphere equation.

---

## Book 2

### Final Scene Render Result:

![book3_image13_1_10.jpg](https://s2.loli.net/2022/07/20/BdbxwjCRDha1WQf.jpg)

### Modified Basic Functions (`basic_tools` series):

- **`camera.rs` Camera:**
  - Added time parameters `$t_0$` and `$t_1$`.

### New Hittable Objects:

- **`moving_sphere.rs` Moving Sphere:**
  - Uses `$t_0$` and `$t_1$` to make the sphere's center move over time, creating motion blur effects.

- **`XYRectangle`, `XZRectangle`, `YZRectangle`:**
  - Rectangular planes.

- **`Cube`:**
  - Rectangular box composed of rectangular planes.
  - **`Translate`**: Moves the box by altering the ray position.
  - **`RotateY`**: Rotates the box along the Y-axis.

- **`fog` Fog Class:**
  - **`Isotropic`**: Uniform medium material.
  - **`ConstantMedium`**: Simulates fog or clouds by calculating refraction and reflection on convex shapes.

### New Data Structure: BVH (Bounding Volume Hierarchy)

- **`AABB` Box:**
  - Wraps objects in an axis-aligned bounding box to check if a ray hits them.

- **`BVH Node`:**
  - Functions like a K-D tree, narrowing down hits layer by layer. If a ray misses the bounding box, smaller objects inside it are skipped, significantly reducing computations to logarithmic complexity.

### Completed Textures (`texture` series):

- **`SolidColor`:**
  - Solid color textures.

- **`CheckerTexture`:**
  - Checkerboard pattern textures with alternating colors.

- **`NoiseTexture`:**
  - Uses Perlin noise to create patterns like marble or speckles.

- **`ImageTexture`:**
  - Projects rectangular images onto spheres for texture mapping.

### Added Materials (`material` series):

- **`diffuse_light.rs`:**
  - Added a diffuse light class, usable for ceiling lights.

---

## Book 3

### Cornell Box Render Result:

![book3_image12_1000.jpg](https://s2.loli.net/2022/07/20/1IQjtnovgpTuNbH.jpg)

### Improved Ray Rendering Algorithm:

- Reduced noise in rendered images.

### Completed ONB (Orthonormal Basis):

- Generates random directions on a sphere based on actual surface normals.
- Builds three orthogonal vectors starting from `rec.normal`.
- Extends corresponding vectors using the `local` function.

### Completed PDF Classes for Objects:

- **`CosinePDF`:**
  - Importance sampling for rays based on $cos\ \theta$.

- **`HittablePDF`:**
  - Directly samples light sources.

- **`MixturePDF`:**
  - Combines the above two methods (50/50 blend).

---

## Extra Features

**Added Hittable Classes:**

- **`triangle.rs` Triangle:**
  - Represents triangles formed by any three points in space.

- **Object:**
  - Object class imports `.obj` files and constructs arbitrary objects from triangular faces.
  - Supports texture mapping based on `.obj` file texture information.

![Patrick Star](https://s2.loli.net/2022/07/27/vVIfHakB4cTChQx.jpg)

- **`Ring`:**
  - Ring class with no thickness, used for Saturn rings.

- **`RotateX`:**
  - Rotates objects along the X-axis.

**Added Material Classes:**

- **`MixtureMaterial`:**
  - Mixes any two materials in a customizable ratio.

- **Modified `DiffuseLight`:**
  - Allows setting custom light intensity.

**Improved Multithreading:**

- Added random shuffling to ensure nearly identical completion times across threads.

![thread.png](https://s2.loli.net/2022/07/27/sfxidcmE7b6Kh5a.png)

**Converted Material and Texture Classes to Generics.**

**Explored Unity Shaders:**

- Studied rasterization techniques, lighting algorithms, shadow algorithms, various texture mappings, and implementations for glass and metal materials.
- Completed convolution edge detection for outlining.

![Edge Detection](https://s2.loli.net/2022/07/27/SHFTruDbpYcI9G8.jpg)

---

### Final Work: "The Whale"

![high5_1.jpg](https://s2.loli.net/2022/07/27/aKfk9OorRm6NWvS.jpg)