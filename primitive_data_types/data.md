### Rust dasturlash tilida integer
 - (butun son) turlari to'rtta asosiy sinfga bo'linadi: signed (imzoli), unsigned (imzosiz), fixed-size (qat'iy o'lchamli), va isize va usize. Har bir sinfda bir nechta turlar mavjud.

## 1. Signed Integer Types (Imzoli butun son turlari)
Imzoli butun sonlar musbat va manfiy qiymatlarni qabul qilishi mumkin. Ular quyidagi turlarga bo'linadi:

- i8: 8-bit signed integer (-128 dan 127 gacha)
- i16: 16-bit signed integer (-32,768 dan 32,767 gacha)
- i32: 32-bit signed integer (-2,147,483,648 dan 2,147,483,647 gacha)
- i64: 64-bit signed integer (-9,223,372,036,854,775,808 dan 9,223,372,036,854,775,807 gacha)
- i128: 128-bit signed integer (-170,141,183,460,469,231,731,687,303,715,884,105,728 dan 170,141,183,460,469,231,731,687,303,715,884,105,727 gacha)
## 2. Unsigned Integer Types (Imzosiz butun son turlari)
Imzosiz butun sonlar faqat musbat qiymatlarni qabul qiladi. Quyidagi turlar mavjud:

- u8: 8-bit unsigned integer (0 dan 255 gacha)
- u16: 16-bit unsigned integer (0 dan 65,535 gacha)
- u32: 32-bit unsigned integer (0 dan 4,294,967,295 gacha)
- u64: 64-bit unsigned integer (0 dan 18,446,744,073,709,551,615 gacha)
- u128: 128-bit unsigned integer (0 dan 340,282,366,920,938,463,463,374,607,431,768,211,455 gacha)
## 3. isize va usize Turlari
Bu turlar kompyuterning arxitekturasiga (32-bit yoki 64-bit) bog'liq holda o'zgaradi:

- isize: Signed integer, tizimning so'z hajmi bilan bir xil (32-bit yoki 64-bit arxitektura)
- usize: Unsigned integer, tizimning so'z hajmi bilan bir xil (32-bit yoki 64-bit arxitektura)
## 4. Fixed-Size Integer Types (Qat'iy o'lchamli butun son turlari)
Rustda fixed-size integerlar yuqoridagi kabi qat'iy hajmli bo'lib, i8, u8 dan i128, u128 gacha bo'lgan turlarni o'z ichiga oladi. Ular muayyan vazifalarni bajarishda, masalan, ma'lum bir o'lchamli raqamli ma'lumotlarni saqlashda ishlatiladi.

Default Integer Type
Rustda integer tiplarini aniq ko'rsatib o'tmagan bo'lsangiz, u i32 tipini ishlatadi. Bu, asosan, balansli bo'lgani uchun: yetarli diapazon va samarali ishlov berish imkonini beradi.

Rust integer turlari bilan ishlashda, ular diapazonlaridan tashqarida qiymatlarni saqlashga urinmaslik kerak, aks holda bu xatoliklarni keltirib chiqaradi.

Bu integer turlari Rust dasturlash tilida turli xil ilovalar uchun keng qo'llaniladi va ular sonli ma'lumotlarni to'g'ri boshqarish imkonini beradi.