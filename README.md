# PPCA Raytracer 2022

光追模拟！

目前进度：完成Book 1 的 final scence

## Book 1

final scence 渲染结果：

![img](https://raw.githubusercontent.com/lillian039/PPCA-Raytracer-2022/main/img/image211.jpg)

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
  - 目前仅有fn hit
- HitRecord
  - 记录光线碰撞到该点后的状态
- sphere.rs
  - 物品球的抽象类，通过球的方程确定球

## Book 2
