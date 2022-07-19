# PPCA Raytracer 2022

光追模拟！

目前进度：完成Book 1 的 final scence

## Book 1

final scence 渲染结果：

![image21.jpg (2560×1600) (raw.githubusercontent.com)](https://raw.githubusercontent.com/lillian039/PPCA-Raytracer-2022/main/img/image21.jpg)

完成基础函数 basic_tools 系列：

- vec3.rs 向量
  - Vec3，Color，Point 向量类声明
  - 光线反射（reflection），折射（refraction）的模拟计算
  - 多种运算符重载
  - 向量的点乘与叉乘
  - 随机单位向量的生成，随机圆盘上的向量生成
  - 计算向量长度，向量归一化处理
- ray.rs 光线
  - Ray 类光线向量声明，包括起点与方向
- camera.rs 相机
  - Camera 类声明
  - 画布定位，包括画布的宽与高，画布左下角定位
  - 相机定位，包括相机的位置，相机的朝向，相机的角度
  - 光圈大小，实现景深效果

完成材质 material 系列：

- lambertian.rs 漫反射材质
  - 漫反射，通过随机数产生随机的光线方向，模拟漫反射
- metal.rs 金属材质
  - 计算严格对称的光线反射路径，并增加fuzz属性，代表金属表面的光泽程度，即反射的虚化度
- dielectric.rs 玻璃材质
  - 运用折射公式计算效果，折射比率大于1时，视作反射

完成物品 hittable 类：

- 抽象trait Hittable 
  - 目前仅有fn hit，判断是否击打到并计算HitRecord
- HitRecord
  - 记录光线碰撞到该点后的状态
- sphere.rs
  - 物品球的抽象类，通过球的方程确定球

## Book 2

final scene 渲染结果：

![book3_image13_1_10.jpg (800×800)](https://raw.githubusercontent.com/lillian039/PPCA-Raytracer-2022/main/img/book3_image13_1_10.jpg)

修改基础函数basic_tools系列：

- camera.rs：相机
  - 增加时间参数 $t_0$ 与 $t_1$ 

新增物品 hittable 类：

- moving_sphere.rs：会动的球
  - 利用时间 $t_0$ 与 $t_1$ ，使小球圆心轨迹随时间变化，做出小球跳动模糊的效果

- XYRectangle XZRectangle YZRectangle
  - 三个方形的面
- Cube：长方体盒（由三个方形面组成）
  - Translate：移动长方体（只移动光线）
  - RotateY：沿Y轴旋转长方体

- fog：雾气类
  - Isotropic：均匀介质材料
  - ConstantMeidum：通过计算两个面的折射与反射，模拟可以允许光线穿过的云雾类物品（也因此仅限于凸物品）

新增数据结构BVH（Bounding Volume Hierarchy）

- AABB盒
  - 将物品通过一个正方向盒子包裹住，用来判断物品是否被光线击中
- BVH Node
  - 类似K-D树，一层层向下搜索，由多个物品盒到单个物品，若击中装着多个物品的盒子，则继续向下搜索，否则一定不会击打到更小的物品，以此减少物品光线判断次数，大幅度提升效率，将效率控制在 $log$ 

完成材质 texture 系列

- SolidColor
  - 纯色纹样
- CheckerTexture
  - 棋盘纹样，分块填上不同的颜色
- NoisTexture
  - 使用Perlin Noise 形成噪点、大理石等等图样
- ImageTexture
  - 将长方形图片按比例投影到球上，做出贴图效果

新增材质 material 系列：

- diffuse_light.rs
  - 增加发光类 diffuse light 可以用作顶灯

## Book 3

cornell_box 渲染结果：

![book3_image12_1000.jpg (500×500)](https://raw.githubusercontent.com/lillian039/PPCA-Raytracer-2022/main/img/book3_image12_1000.jpg)

改进光线渲染算法，减少噪点

完成ONB（标准正交基）类：

- 生成基于真实法线方向的球随机方向

- 由 rec.normal 出发，生成三个方向相互垂直的标准正交基
- 通过 local 函数通过标准正交基拓展出对应的向量

完成部分物品的PDF类：

- CosinePDF
  - 由与$ cos\ \theta $ 相关的pdf出发的对光线进行的重要性采样
- HittablePDF
  - 直接对光照进行采样
- MixturePDF
  - 将二者结合（各0.5）的混合密度采样

